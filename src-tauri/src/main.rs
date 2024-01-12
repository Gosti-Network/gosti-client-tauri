// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hex::ToHex;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem, Manager};
use std::fs::File;
use std::io::Read;
use std::fs::write;

mod rqbit_config;
mod chia_rpc;

use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    path::Path,
    sync::Arc,
};

use anyhow::Context;
use rqbit_config::RqbitDesktopConfig;
use http::StatusCode;
use librqbit::{
    api::{
        ApiAddTorrentResponse, EmptyJsonResponse, TorrentDetailsResponse, TorrentListResponse,
        TorrentStats,
    },
    dht::PersistentDhtConfig,
    tracing_subscriber_config_utils::{init_logging, InitLoggingOptions, InitLoggingResult},
    AddTorrent, AddTorrentOptions, Api, ApiError, PeerConnectionOptions, Session, SessionOptions,
};
use parking_lot::RwLock;
use serde::Serialize;
use tracing::{error, error_span};

const ERR_NOT_CONFIGURED: ApiError =
    ApiError::new_from_text(StatusCode::FAILED_DEPENDENCY, "not configured");

struct StateShared {
    config: RqbitDesktopConfig,
    api: Option<Api>,
}

struct State {
    config_filename: String,
    shared: Arc<RwLock<Option<StateShared>>>,
    init_logging: InitLoggingResult,
}

fn read_rqbit_config(path: &str) -> anyhow::Result<RqbitDesktopConfig> {
    let rdr = BufReader::new(File::open(path)?);
    Ok(serde_json::from_reader(rdr)?)
}

fn write_rqbit_config(path: &str, config: &RqbitDesktopConfig) -> anyhow::Result<()> {
    std::fs::create_dir_all(Path::new(path).parent().context("no parent")?)
        .context("error creating dirs")?;
    let tmp = format!("{}.tmp", path);
    let mut tmp_file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&tmp)?,
    );
    serde_json::to_writer(&mut tmp_file, config)?;
    std::fs::rename(tmp, path)?;
    Ok(())
}

async fn api_from_rqbit_config(
    init_logging: &InitLoggingResult,
    config: &RqbitDesktopConfig,
) -> anyhow::Result<Api> {
    let session = Session::new_with_opts(
        config.default_download_location.clone(),
        SessionOptions {
            disable_dht: config.dht.disable,
            disable_dht_persistence: config.dht.disable_persistence,
            dht_config: Some(PersistentDhtConfig {
                config_filename: Some(config.dht.persistence_filename.clone()),
                ..Default::default()
            }),
            persistence: !config.persistence.disable,
            persistence_filename: Some(config.persistence.filename.clone()),
            peer_opts: Some(PeerConnectionOptions {
                connect_timeout: Some(config.peer_opts.connect_timeout),
                read_write_timeout: Some(config.peer_opts.read_write_timeout),
                ..Default::default()
            }),
            listen_port_range: if !config.tcp_listen.disable {
                Some(config.tcp_listen.min_port..config.tcp_listen.max_port)
            } else {
                None
            },
            enable_upnp_port_forwarding: !config.upnp.disable,
            ..Default::default()
        },
    )
    .await
    .context("couldn't set up librqbit session")?;

    let api = Api::new(
        session.clone(),
        Some(init_logging.rust_log_reload_tx.clone()),
        Some(init_logging.line_broadcast.clone()),
    );

    if !config.http_api.disable {
        let http_api_task = librqbit::http_api::HttpApi::new(
            api.clone(),
            Some(librqbit::http_api::HttpApiOptions {
                cors_enable_all: true,
                read_only: config.http_api.read_only,
            }),
        )
        .make_http_api_and_run(config.http_api.listen_addr);

        session.spawn(error_span!("http_api"), http_api_task);
    }
    Ok(api)
}

impl State {
    async fn new(init_logging: InitLoggingResult) -> Self {
        let config_filename = directories::ProjectDirs::from("io", "gosti", "desktop")
            .expect("directories::ProjectDirs::from")
            .config_dir()
            .join("config.json")
            .to_str()
            .expect("to_str()")
            .to_owned();

        if let Ok(config) = read_rqbit_config(&config_filename) {
            let api = api_from_rqbit_config(&init_logging, &config).await.ok();
            let shared = Arc::new(RwLock::new(Some(StateShared { config, api })));

            return Self {
                config_filename,
                shared,
                init_logging,
            };
        }

        Self {
            config_filename,
            init_logging,
            shared: Arc::new(RwLock::new(None)),
        }
    }

    fn api(&self) -> Result<Api, ApiError> {
        let g = self.shared.read();
        match g.as_ref().and_then(|s| s.api.as_ref()) {
            Some(api) => Ok(api.clone()),
            None => Err(ERR_NOT_CONFIGURED),
        }
    }

    async fn configure(&self, config: RqbitDesktopConfig) -> Result<(), ApiError> {
        {
            let g = self.shared.read();
            if let Some(shared) = g.as_ref() {
                if shared.api.is_some() && shared.config == config {
                    // The config didn't change, and the API is running, nothing to do.
                    return Ok(());
                }
            }
        }

        let existing = self.shared.write().as_mut().and_then(|s| s.api.take());

        if let Some(api) = existing {
            api.session().stop().await;
        }

        let api = api_from_rqbit_config(&self.init_logging, &config).await?;
        if let Err(e) = write_rqbit_config(&self.config_filename, &config) {
            error!("error writing rqbit_config: {:#}", e);
        }

        let mut g = self.shared.write();
        *g = Some(StateShared {
            config,
            api: Some(api),
        });
        Ok(())
    }
}

#[derive(Default, Serialize)]
struct CurrentState {
    config: Option<RqbitDesktopConfig>,
    configured: bool,
}

#[tauri::command]
fn config_default() -> rqbit_config::RqbitDesktopConfig {
    rqbit_config::RqbitDesktopConfig::default()
}

#[tauri::command]
fn config_current(state: tauri::State<'_, State>) -> CurrentState {
    let g = state.shared.read();
    match &*g {
        Some(s) => CurrentState {
            config: Some(s.config.clone()),
            configured: s.api.is_some(),
        },
        None => Default::default(),
    }
}

#[tauri::command]
async fn config_change(
    state: tauri::State<'_, State>,
    config: RqbitDesktopConfig,
) -> Result<EmptyJsonResponse, ApiError> {
    state.configure(config).await.map(|_| EmptyJsonResponse {})
}


#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_config_impl(app) {
        Ok(config) => Ok(serde_json::json!({"result": config, "valid": true, "message": "Config loaded"})),
        Err(e) => Err(format!("Error in get_config: {}", e)),
    }
}

fn get_config_impl(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    println!("Loading config");
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // Parse the contents as JSON
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    
    Ok(config)
}


#[tauri::command]
fn save_config(app: tauri::AppHandle, config: serde_json::Value) -> Result<serde_json::Value, String> {
    match save_config_impl(app, config) {
        Ok(_) => Ok(serde_json::json!({"message": "Config saved"})),
        Err(e) => Err(format!("Error in save_config: {}", e)),
    }
}

fn save_config_impl(app: tauri::AppHandle, config: serde_json::Value) -> Result<(), String> {
    println!("Saving config");
    let contents = serde_json::to_string(&config).map_err(|e| format!("Error serializing config: {}", e))?;
    write(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource"), contents).map_err(|e| format!("Error writing file: {}", e))?;
    Ok(())
}


#[tauri::command]
fn get_operating_system() -> Result<serde_json::Value, String> {
    match get_operating_system_impl() {
        Ok(os) => Ok(serde_json::json!({"os": os})),
        Err(e) => Err(format!("Error in get_operating_system: {}", e)),
    }
}

fn get_operating_system_impl() -> Result<String, String> {
    let os = std::env::consts::OS;
    Ok(os.to_string())
}




#[tauri::command]
async fn publish_media(params: serde_json::Value) -> Result<serde_json::Value, String> {
    match publish_media_impl( params).await {
        Ok(_) => Ok(serde_json::json!({"message": "Update Published"})),
        Err(e) => Err(format!("Error publishing: {}", e)),
    }
}

async fn publish_media_impl(params: serde_json::Value) -> Result<(), String> {

    let root = chia_rpc::make_chia_rpc_call(
        "get_root".into(),
         serde_json::json!({"id": params["dataStoreId"]})).await?;

    println!("root: {:?}", root);
    
    let entries = chia_rpc::make_chia_rpc_call(
        "get_keys_values".into(), serde_json::json!({"id": params["dataStoreId"], "root_hash": root["hash"]})).await?;

    println!("entries: {:?}", entries);

    for entry in entries["keys_values"].as_array().unwrap() {
        let media_product_id: serde_json::Value = serde_json::from_str(&hex::decode(entry["key"].as_str().unwrap().trim_start_matches("0x")).unwrap().iter().map(|b| *b as char).collect::<String>()).unwrap();
        println!("media_product_id: {:?}", media_product_id.as_str());
        println!("media_product_id: {:?}", media_product_id.to_string());
        println!("params_product_id: {:?}", params["media"]["productId"].as_str().expect("Asdf"));
        if media_product_id == params["media"]["productId"] {
            let changelist = serde_json::json!([
                {
                    "action": "delete",
                    "key": hex::encode(params["media"]["productId"].to_string()),
                },
                {
                    "action": "insert",
                    "key": hex::encode(params["media"]["productId"].to_string()),
                    "value": hex::encode(params["media"].to_string()),
                },
            ]);
            let response = chia_rpc::make_chia_rpc_call(
                "batch_update".into(),
                serde_json::json!({
                    "id": params["dataStoreId"],
                    "changelist": changelist,
                    "fee": params["fee"],
                }),
            )
            .await?;
            println!("response: {:?}", response);
            return Ok(())
        }
    }

    let changelist = serde_json::json!([
        {
            "action": "insert",
            "key": hex::encode(params["media"]["productId"].to_string()),
            "value": hex::encode(params["media"].to_string()),
        },
    ]);
    chia_rpc::make_chia_rpc_call(
        "batch_update".into(),
        serde_json::json!({
            "id": params["dataStoreId"],
            "changelist": changelist,
            "fee": params["fee"],
        }),
    )
    .await?;
    Ok(())
}


#[tauri::command]
async fn get_published_media(params: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_published_media_impl(params).await {
        Ok(media) => Ok(serde_json::json!({"media": media})),
        Err(e) => Err(format!("Error in get_published_media: {}", e)),
    }
}

async fn get_published_media_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let root = chia_rpc::make_chia_rpc_call(
        "get_root".into(),
         serde_json::json!({"id": params["dataStoreId"]})).await?;

    let medias = chia_rpc::make_chia_rpc_call(
        "get_keys_values".into(), serde_json::json!({"id": params["dataStoreId"], "root_hash": root["hash"]})).await?;

    let mut media_list = Vec::new();

    for media in medias["keys_values"].as_array().unwrap() {
        let media_json: serde_json::Value = serde_json::from_str(&hex::decode(media["value"].as_str().unwrap().trim_start_matches("0x")).unwrap().iter().map(|b| *b as char).collect::<String>()).unwrap();
        media_list.push(media_json);
    }


    println!("media_list: {:?}", media_list);

    Ok(serde_json::Value::Array(media_list))
}


#[tauri::command]
async fn open_app(app: tauri::AppHandle, app_name: String, title: String, url: String) {
    tauri::WindowBuilder::new(
            &app,
            app_name.clone(), /* set the window label to the app name */
            tauri::WindowUrl::App(
                url.into()
            )
        )
        .title(title) /* set the window title to the app name */
        .inner_size(1200.0, 700.0)
        .build()
        .expect("failed to build window");
}


#[tauri::command]
fn torrents_list(state: tauri::State<State>) -> Result<TorrentListResponse, ApiError> {
    Ok(state.api()?.api_torrent_list())
}

#[tauri::command]
async fn torrent_create_from_url(
    state: tauri::State<'_, State>,
    url: String,
    opts: Option<AddTorrentOptions>,
) -> Result<ApiAddTorrentResponse, ApiError> {
    state
        .api()?
        .api_add_torrent(AddTorrent::Url(url.into()), opts)
        .await
}

#[tauri::command]
async fn torrent_create_from_base64_file(
    state: tauri::State<'_, State>,
    contents: String,
    opts: Option<AddTorrentOptions>,
) -> Result<ApiAddTorrentResponse, ApiError> {
    use base64::{engine::general_purpose, Engine as _};
    let bytes = general_purpose::STANDARD
        .decode(&contents)
        .context("invalid base64")
        .map_err(|e| ApiError::new_from_anyhow(StatusCode::BAD_REQUEST, e))?;
    state
        .api()?
        .api_add_torrent(AddTorrent::TorrentFileBytes(bytes.into()), opts)
        .await
}

#[tauri::command]
async fn torrent_details(
    state: tauri::State<'_, State>,
    id: usize,
) -> Result<TorrentDetailsResponse, ApiError> {
    state.api()?.api_torrent_details(id)
}

#[tauri::command]
async fn torrent_stats(
    state: tauri::State<'_, State>,
    id: usize,
) -> Result<TorrentStats, ApiError> {
    state.api()?.api_stats_v1(id)
}

#[tauri::command]
async fn torrent_action_delete(
    state: tauri::State<'_, State>,
    id: usize,
) -> Result<EmptyJsonResponse, ApiError> {
    state.api()?.api_torrent_action_delete(id)
}

#[tauri::command]
async fn torrent_action_pause(
    state: tauri::State<'_, State>,
    id: usize,
) -> Result<EmptyJsonResponse, ApiError> {
    state.api()?.api_torrent_action_pause(id)
}

#[tauri::command]
async fn torrent_action_forget(
    state: tauri::State<'_, State>,
    id: usize,
) -> Result<EmptyJsonResponse, ApiError> {
    state.api()?.api_torrent_action_forget(id)
}

#[tauri::command]
async fn torrent_action_start(
    state: tauri::State<'_, State>,
    id: usize,
) -> Result<EmptyJsonResponse, ApiError> {
    state.api()?.api_torrent_action_start(id)
}

#[tauri::command]
fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}










#[tauri::command]
async fn gosti_bulk_nft_mint(params: serde_json::Value) -> Result<serde_json::Value, String> {
    match gosti_bulk_nft_mint_impl(params).await {
        Ok(_) => Ok(serde_json::json!({"message": "NFTs Minted"})),
        Err(e) => Err(format!("Error in gosti_bulk_nft_mint: {}", e)),
    }
}

async fn gosti_bulk_nft_mint_impl(params: serde_json::Value) -> Result<(), String> {

    println!("params: {:?}", params);

    let response = chia_rpc::make_chia_rpc_call("get_wallets", serde_json::json!({})).await?;

    let wallets = response["wallets"].as_array().unwrap();
    let mut mint_wallet = 0;

    for wallet in wallets {
        println!("wallet: {:?}", wallet);
        if wallet["type"].as_u64().expect("Invalid Wallet Type") == chia_rpc::WALLET_TYPE_NFT {
            println!("wallet!!!!: {:?}", wallet);
            if let Some(data) = wallet["data"].as_str() {
                println!("data: {:?}", data);
                if let Ok(did) = serde_json::from_str::<serde_json::Value>(data) {
                    println!("did: {:?}", did);
                    if let Some(did_id) = did["did_id"].as_str() {
                        if did_id == params["publisherDid"].as_str().expect("Invalid Publisher DID") {
                            mint_wallet = wallet["id"].as_u64().expect("Invalid Wallet ID");
                        }
                    }
                }
            }
        }
    }

    
    
    let quantity = params["quantity"].as_str().expect("Invalid Quantity").parse::<u64>().expect("Invalid Quantity");
    let mut metadata_list: Vec<serde_json::Value> = Vec::new();
    let mut target_list: Vec<serde_json::Value> = Vec::new();

    for num in 0..quantity {
        println!("num: {:?}", num);
        target_list.push(params["receiveAddress"].clone());
        metadata_list.push(serde_json::json!({
            "uris": params["imageUris"],
            "meta_uris": params["metadataUris"],
            "license_uris": params["licenseUris"],
            "hash": "",
            "meta_hash": "",
            "license_hash": "",
            "edition_number": num,
            "edition_total": quantity,
        }));
    }

    println!("metadata_list: {:?}", metadata_list);

    let response = chia_rpc::make_chia_rpc_call("nft_mint_bulk", serde_json::json!({
        "wallet_id": mint_wallet,
        "metadata_list": metadata_list,
        "royalty_percentage": params["royaltyPercentage"],
        "royalty_address": params["royaltyAddress"],
        "target_list": target_list,
        "mint_from_did": true,
        "fee": params["fee"],
        "reuse_puzhash": false,
    })).await?;

    println!("mint response: {:?}", response);

    let spend_bundle_hex: String = response["spend_bundle"].to_string().encode_hex();
    let transaction = chia_rpc::make_chia_rpc_call("push_tx", serde_json::json!({
        "spend_bundle": spend_bundle_hex,
        "wallet_id": mint_wallet,
    })).await?;
    println!("transaction: {:?}", transaction);
    Ok(())
}
















async fn start() {

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let init_logging_result = init_logging(InitLoggingOptions {
        default_rust_log_value: Some("info"),
        log_file: None,
        log_file_rust_log: None,
    })
    .unwrap();

     let state = State::new(init_logging_result).await;


    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);


    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
            let windows = app.windows();
            for (_, window) in &windows {
                window.show().unwrap();
            }
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let windows = app.windows();
                for (_, window) in &windows {
                    window.hide().unwrap();
                }
            }
            "show" => {
                let windows = app.windows();
                for (_, window) in &windows {
                    window.show().unwrap();
                }
            }
            _ => {}
            }
        }
        _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() == "main" {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            open_app,
            get_config,
            save_config,
            get_operating_system,
            torrents_list,
            torrent_create_from_url,
            torrent_create_from_base64_file,
            torrent_details,
            torrent_stats,
            torrent_action_delete,
            torrent_action_pause,
            torrent_action_forget,
            torrent_action_start,
            torrent_create_from_base64_file,
            get_version,
            config_default,
            config_current,
            config_change,
            publish_media,
            get_published_media,
            gosti_bulk_nft_mint,
            chia_rpc::make_chia_rpc_call,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("couldn't set up tokio runtime")
        .block_on(start())
}
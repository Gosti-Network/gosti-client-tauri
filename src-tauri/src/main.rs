// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod libtorrent;

use libtorrent::ffi::lt_create_session;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use std::fs::{self, File};
use std::io::{self, Read};
use std::fs::write;
use std::path::Path;
use hex::ToHex;
use secp256k1::{Secp256k1, SecretKey, Keypair, Message};


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
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in save_config: {}", e)),
    }
}

fn save_config_impl(app: tauri::AppHandle, config: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("Saving config");
    let contents = serde_json::to_string(&config).map_err(|e| format!("Error serializing config: {}", e))?;
    write(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource"), contents).map_err(|e| format!("Error writing file: {}", e))?;
    Ok(serde_json::json!({"message": "Config saved"}))
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
async fn add_nostr_keypair(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_nostr_keypair_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_nostr_keypair: {}", e)),
    }
}

async fn add_nostr_keypair_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("params: {:?}", params);

    let mut file = File::open(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let keys: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;



    let contents = serde_json::to_string(&keys).map_err(|e| format!("Error serializing params: {}", e))?;
    write(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource"), contents).map_err(|e| format!("Error writing file: {}", e))?;

    Ok(serde_json::json!({"message": "Config saved"}))
}


#[tauri::command]
async fn has_nostr_private_key(app: tauri::AppHandle, params: serde_json::Value) -> Result<bool, String> {
    match has_nostr_private_key_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in has_nostr_private_key: {}", e)),
    }
}

async fn has_nostr_private_key_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<bool, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let keys: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    let keys = &keys["keys"];

    for key in keys.as_array().unwrap() {
        if key["publicKey"].as_str().unwrap() == params["publicKey"].as_str().unwrap() {
            return Ok(true);
        }
    }

    Ok(false)
}


#[tauri::command]
async fn sign_nostr_message(app: tauri::AppHandle, message: serde_json::Value) -> Result<serde_json::Value, String> {
    match sign_nostr_message_impl(app, message).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in sign_nostr_message: {}", e)),
    }
}

async fn sign_nostr_message_impl(app: tauri::AppHandle, message: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config = get_config_impl(app).map_err(|e| format!("Error getting config: {}", e))?;
    let keychain: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    let keys = &keychain["keys"];

    let mut sig = serde_json::Value::Null;

    for key in keys.as_array().unwrap() {
        if key["publicKey"].as_str().unwrap() != config["identity"]["currentNostrPublicKey"].as_str().unwrap() {
            continue;
        }
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&hex::decode(key["privateKey"].as_str().unwrap()).unwrap()).unwrap();
        let keypair = Keypair::from_secret_key(&secp, &secret_key);
        let message_to_sign = Message::from_digest_slice(hex::decode(message.as_str().unwrap()).unwrap().as_slice()).unwrap();
        let real_sig = secp.sign_schnorr_no_aux_rand(&message_to_sign, &keypair);
        sig = serde_json::Value::String(real_sig.serialize().encode_hex::<String>());
    }

    Ok(sig)
}


#[tauri::command]
async fn get_install_status(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_install_status_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_install_status: {}", e)),
    }
}

async fn get_install_status_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    Ok(config)
}


#[tauri::command]
async fn get_local_media_metadata(app: tauri::AppHandle, product_id: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_local_media_metadata_impl(app, product_id).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_local_media_metadata: {}", e)),
    }
}

async fn get_local_media_metadata_impl(app: tauri::AppHandle, product_id: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("get_local_media_metadata_impl params: {:?}", product_id);
    let mut file = File::open(app.path_resolver().resolve_resource(format!("../resources/localmediametadata/{}.json", product_id.as_str().unwrap())).expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // Parse the contents as JSON
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    
    Ok(config)
}


#[tauri::command]
async fn save_local_media_metadata(app: tauri::AppHandle, media: serde_json::Value) -> Result<serde_json::Value, String> {
    match save_local_media_metadata_impl(app, media).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in save_local_media_metadata: {}", e)),
    }
}

async fn save_local_media_metadata_impl(app: tauri::AppHandle, media: serde_json::Value) -> Result<serde_json::Value, String> {
    let contents = serde_json::to_string(&media).map_err(|e| format!("Error serializing media: {}", e))?;
    println!("media contents: {:?}", contents);
    let path = app.path_resolver().resolve_resource(format!("../resources/localmediametadata/{}.json", media["productId"].as_str().unwrap())).expect("failed to resolve resource");
    let path = Path::new(&path);
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| format!("Error creating directory: {}", e))?;
    }
    write(path, contents).map_err(|e| format!("Error writing file: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}


#[tauri::command]
async fn download_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match download_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in download_media: {}", e)),
    }
}

async fn download_media_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    

    Ok(config)
}


#[tauri::command]
async fn delete_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match delete_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in delete_media: {}", e)),
    }
}

async fn delete_media_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    let torrents_path = format!("{}/{}", config["torrentsPath"], params["productId"].as_str().unwrap());
    let media_data_path = format!("{}/{}", config["mediaDataPath"], params["productId"].as_str().unwrap());
    
    fs::remove_dir_all(torrents_path).map_err(|e| format!("Error deleting directory: {}", e))?;
    fs::remove_dir_all(media_data_path).map_err(|e| format!("Error deleting directory: {}", e))?;

    Ok(config)
}


#[tauri::command]
async fn install_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match install_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in install_media: {}", e)),
    }
}

async fn install_media_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    let torrents_path = format!("{}/{}", config["torrentsPath"], params["productId"].as_str().unwrap());

    let file = File::open(&torrents_path).map_err(|e| format!("Error opening file: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Error reading zip: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Error reading file from zip: {}", e))?;
        let outpath = file.mangled_name();

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("Error creating directory: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| format!("Error creating directory: {}", e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| format!("Error creating file: {}", e))?;
            io::copy(&mut file, &mut outfile).map_err(|e| format!("Error writing file: {}", e))?;
        }
    }
    

    Ok(config)
}


#[tauri::command]
async fn uninstall_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match uninstall_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in uninstall_media: {}", e)),
    }
}

async fn uninstall_media_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    let installs_path = format!("{}/{}", config["installsPath"], params["productId"].as_str().unwrap());
    
    fs::remove_dir_all(installs_path).map_err(|e| format!("Error deleting directory: {}", e))?;
    Ok(config)
}


#[tauri::command]
async fn launch_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match launch_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in launch_media: {}", e)),
    }
}

async fn launch_media_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/gosti-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    let installs_path = format!("{}/{}", config["installsPath"], params["productId"].as_str().unwrap());

    // needs the executable added to the path
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "start", &installs_path])
            .spawn()
            .expect("failed to start application");
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("xdg-open")
            .args(&[&installs_path])
            .spawn()
            .expect("failed to start application");
    } else {
        std::process::Command::new("open")
            .args(&[&installs_path])
            .spawn()
            .expect("failed to start application");
    }

    Ok(config)
}



async fn start() {

    // lt_create_session();

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
        .invoke_handler(tauri::generate_handler![
            open_app,
            get_config,
            save_config,
            get_operating_system,
            add_nostr_keypair,
            has_nostr_private_key,
            sign_nostr_message,
            get_install_status,
            get_local_media_metadata,
            save_local_media_metadata,
            download_media,
            delete_media,
            install_media,
            uninstall_media,
            launch_media,
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
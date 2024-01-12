use uuid::Uuid;



#[tauri::command]
fn install_media(media: serde_json::Value) -> Result<serde_json::Value, String> {
    match install_media_impl(media) {
        Ok(_) => Ok(serde_json::json!({"status": "complete"})),
        Err(e) => Err(format!("Error in install_media: {}", e)),
    }
}

#[tauri::command]
fn uninstall_media(media: serde_json::Value) -> Result<serde_json::Value, String> {
    match uninstall_media_impl(media) {
        Ok(_) => Ok(serde_json::json!({"status": "Uninstalled"})),
        Err(e) => Err(format!("Error in uninstall_media: {}", e)),
    }
}

#[tauri::command]
fn play_media(media: serde_json::Value) -> Result<serde_json::Value, String> {
    match play_media_impl(media) {
        Ok(_) => Ok(serde_json::json!({"status": "Playing"})),
        Err(e) => Err(format!("Error in play_media: {}", e)),
    }
}

#[tauri::command]
fn get_install_status(media: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_install_status_impl(media) {
        Ok(status) => Ok(serde_json::json!({
            "status": {
                "isDownloaded": status.is_downloaded,
                "isDownloading": status.is_downloading,
                "isInstalled": status.is_installed,
                "isInstalling": false,
                "hasPendingUpdate": false,
                "isSeeding": status.is_seeding,
                "progress": status.progress,
                "downloadRate": status.download_rate,
                "uploadRate": status.upload_rate,
            },
            "message": "Status retrieved",
        })),
        Err(e) => Err(format!("Error in get_install_status: {}", e)),
    }
}

#[tauri::command]
fn get_local_data(product_id: String) -> Result<serde_json::Value, String> {
    match get_local_data_impl(product_id) {
        Ok(media) => Ok(serde_json::json!({"media": media})),
        Err(e) => Err(format!("Error in get_local_data: {}", e)),
    }
}

#[tauri::command]
fn save_local_data(media: serde_json::Value) -> Result<serde_json::Value, String> {
    match save_local_data_impl(media) {
        Ok(_) => Ok(serde_json::json!({"status": "saved"})),
        Err(e) => Err(format!("Error in save_local_data: {}", e)),
    }
}

#[tauri::command]
fn load_all_local_data() -> Result<serde_json::Value, String> {
    match load_all_local_data_impl() {
        Ok(media) => Ok(serde_json::json!({"media": media})),
        Err(e) => Err(format!("Error in load_all_local_data: {}", e)),
    }
}

#[tauri::command]
fn get_owned_stores() -> Result<serde_json::Value, String> {
    match get_owned_stores_impl() {
        Ok(data_store_ids) => Ok(serde_json::json!({"dataStoreIds": data_store_ids})),
        Err(e) => Err(format!("Error in get_owned_stores: {}", e)),
    }
}

#[tauri::command]
fn get_published_media(data_store_id: String) -> Result<serde_json::Value, String> {
    match get_published_media_impl(data_store_id) {
        Ok(media) => Ok(serde_json::json!({"media": media})),
        Err(e) => Err(format!("Error in get_published_media: {}", e)),
    }
}

#[tauri::command]
fn publish_media(data_store_id: String, media: serde_json::Value, fee: i32) -> Result<serde_json::Value, String> {
    match publish_media_impl(data_store_id, media, fee) {
        Ok(result) => Ok(serde_json::json!({"message": result})),
        Err(e) => Err(format!("Error in publish_media: {}", e)),
    }
}

#[tauri::command]
fn create_data_store(fee: i32) -> Result<serde_json::Value, String> {
    match create_data_store_impl(fee) {
        Ok(result) => Ok(serde_json::json!({"result": result, "success": true})),
        Err(e) => Err(format!("Error in create_data_store: {}", e)),
    }
}

#[tauri::command]
fn mint_nft_copies(minting_config: serde_json::Value) -> Result<serde_json::Value, String> {
    match mint_nft_copies_impl(minting_config) {
        Ok(_) => Ok(serde_json::json!({"status": "Minting"})),
        Err(e) => Err(format!("Error in mint_nft_copies: {}", e)),
    }
}


fn install_media_impl(media: serde_json::Value) -> Result<(), String> {
    // Implementation goes here
    unimplemented!()
}

fn uninstall_media_impl(media: serde_json::Value) -> Result<(), String> {
    // Implementation goes here
    unimplemented!()
}

fn play_media_impl(media: serde_json::Value) -> Result<(), String> {
    // Implementation goes here
    unimplemented!()
}

fn get_install_status_impl(media: serde_json::Value) -> Result<InstallStatus, String> {
    // Implementation goes here
    unimplemented!()
}

fn get_local_data_impl(product_id: String) -> Result<serde_json::Value, String> {
    // Implementation goes here
    unimplemented!()
}

fn save_local_data_impl(media: serde_json::Value) -> Result<(), String> {
    // Implementation goes here
    unimplemented!()
}

fn load_all_local_data_impl() -> Result<Vec<serde_json::Value>, String> {
    // Implementation goes here
    unimplemented!()
}

fn get_owned_stores_impl() -> Result<Vec<String>, String> {
    // Implementation goes here
    unimplemented!()
}

fn get_published_media_impl(data_store_id: String) -> Result<Vec<serde_json::Value>, String> {
    // Implementation goes here
    unimplemented!()
}

fn publish_media_impl(data_store_id: String, media: serde_json::Value, fee: i32) -> Result<String, String> {
    // Implementation goes here
    unimplemented!()
}

fn create_data_store_impl(fee: i32) -> Result<String, String> {
    // Implementation goes here
    unimplemented!()
}

fn mint_nft_copies_impl(minting_config: serde_json::Value) -> Result<(), String> {
    // Implementation goes here
    unimplemented!()
}


struct InstallStatus {
    is_downloaded: bool,
    is_downloading: bool,
    is_installed: bool,
    is_seeding: bool,
    progress: u32,
    download_rate: u32,
    upload_rate: u32,
}



struct Media {
    copies: u32,
    info: MediaInfo,
}

struct MediaInfo {
    mediaType: String,
    banner: String,
    capsuleImage: String,
    contentRating: String,
    description: String,
    creator: String,
    discord: String,
    executables: Executables,
    facebook: String,
    icon: String,
    instagram: String,
    longDescription: String,
    password: String,
    paymentAddress: String,
    productId: String,
    publisher: String,
    publisherDid: String,
    screenshots: Vec<String>,
    shortDescription: String,
    status: String,
    tags: Vec<String>,
    title: String,
    torrents: Torrents,
    trailer: String,
    trailerSource: String,
    twitter: String,
    version: String,
    website: String,
}

struct Executables {
    Windows: String,
    Mac: String,
    Linux: String,
}

struct Torrents {
    Windows: String,
    Mac: String,
    Linux: String,
}

impl Media {
    fn new(
        productId: String,
        title: String,
        shortDescription: String,
        description: String,
        longDescription: String,
        developer: String,
        publisher: String,
        publisherDid: String,
        website: String,
        twitter: String,
        discord: String,
        instagram: String,
        facebook: String,
        contentRating: String,
        capsuleImage: String,
        icon: String,
        banner: String,
        trailer: String,
        tags: Vec<String>,
        status: String,
        version: String,
        screenshots: Vec<String>,
        torrents: Torrents,
        executables: Executables,
        paymentAddress: String,
        password: String,
    ) -> Self {
        let copies = 1;
        if productId == "" {
            productId = Uuid::new_v4().to_string();
        }
        let info = MediaInfo {
            mediaType: String::new(),
            banner,
            capsuleImage,
            contentRating,
            description,
            creator: String::new(),
            discord,
            executables,
            facebook,
            icon,
            instagram,
            longDescription,
            password,
            paymentAddress,
            productId,
            publisher,
            publisherDid,
            screenshots,
            shortDescription,
            status,
            tags,
            title,
            torrents,
            trailer,
            trailerSource: String::new(),
            twitter,
            version,
            website,
        };

        Self { copies, info }
    }

    fn get_torrent(&self) -> &str {
        let p = if cfg!(target_os = "linux") {
            "Linux"
        } else if cfg!(target_os = "macos") {
            "Mac"
        } else if cfg!(target_os = "windows") {
            "Windows"
        } else {
            ""
        };

        // &self.info.torrents[p]
        &"Windows"
    }

    fn set_torrents(&mut self, torrents: Torrents) {
        self.info.torrents = torrents;
    }

    fn get_executable(&self) -> &str {
        let p = if cfg!(target_os = "linux") {
            "Linux"
        } else if cfg!(target_os = "macos") {
            "Mac"
        } else if cfg!(target_os = "windows") {
            "Windows"
        } else {
            ""
        };

        // &self.info.executables[p]
        &"Windows"
    }
}

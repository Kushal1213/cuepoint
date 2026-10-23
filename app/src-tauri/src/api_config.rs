/// The desktop app always talks to its own loopback-only bundled service.
/// Provider requests leave the machine only when a user invokes a feature
/// that needs their configured model/search/GitHub provider.
pub const API_BASE_URL: &str = env!("CUEPOINT_API_BASE_URL");
pub const CLIENT_KEY: &str = env!("CUEPOINT_CLIENT_KEY");

#[derive(serde::Serialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub client_key: String,
}

/// Lets the JS side (which has no build step / can't read CUEPOINT_ENV
/// itself) follow whatever this Rust build was compiled for, instead of
/// keeping a second hardcoded copy of the same switch in main.js.
#[tauri::command]
pub fn get_api_config() -> ApiConfig {
    ApiConfig {
        base_url: API_BASE_URL.to_string(),
        client_key: CLIENT_KEY.to_string(),
    }
}

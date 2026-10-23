/// The desktop app always talks to its own loopback-only bundled service.
/// Provider requests leave the machine only when a user invokes a feature
/// that needs their configured model/search/GitHub provider.
pub const API_BASE_URL: &str = "http://127.0.0.1:43127";

#[derive(serde::Serialize)]
pub struct ApiConfig {
    pub base_url: String,
}

/// Gives the JavaScript UI the same loopback address as native commands.
#[tauri::command]
pub fn get_api_config() -> ApiConfig {
    ApiConfig {
        base_url: API_BASE_URL.to_string(),
    }
}

use serde::Serialize;

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interface_name: String,
}

#[tauri::command]
pub fn get_network_info() -> Vec<NetworkInfo> {
    vec![]
}

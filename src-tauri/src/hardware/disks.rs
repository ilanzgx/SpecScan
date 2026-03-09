use serde::Serialize;

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
}

#[tauri::command]
pub fn get_disks_info() -> Vec<DiskInfo> {
    vec![]
}

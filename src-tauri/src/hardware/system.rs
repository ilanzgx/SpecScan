use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct SystemInfo {
    pub os_name: String,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
      os_name: System::name().unwrap_or("Unknown".to_string())
    }
}
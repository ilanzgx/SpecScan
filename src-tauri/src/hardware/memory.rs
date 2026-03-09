use serde::Serialize;
use sysinfo::{System};

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_memory: u64,
}

#[tauri::command]
pub fn get_memory_info() -> MemoryInfo {
    let mut sys = System::new();
    sys.refresh_all();

    MemoryInfo {
        total_memory: sys.total_memory(),
    }
}

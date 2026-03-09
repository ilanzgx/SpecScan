use serde::Serialize;

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_memory: u64,
}

#[tauri::command]
pub fn get_memory_info() -> MemoryInfo {
    MemoryInfo {
        total_memory: 0,
    }
}

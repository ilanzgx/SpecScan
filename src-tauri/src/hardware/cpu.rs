use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct CpuInfo {
    pub name: String,
    pub cores: u32,
    pub frequency: u64,
}

#[tauri::command]
pub fn get_cpu_info() -> CpuInfo {
    let mut sys = System::new();
    sys.refresh_all();

    let name = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or("Unknown CPU".into());

    CpuInfo {
        name: name,
        cores: sys.cpus().len() as u32,
        frequency: sys.cpus().first().map(|cpu| cpu.frequency()).unwrap_or(0),
    }
}

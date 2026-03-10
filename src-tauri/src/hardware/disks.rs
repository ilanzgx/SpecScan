use serde::Serialize;
use sysinfo::{Disks, DiskKind};

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub kind: String,
    pub file_system: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub available_gb: f64,
    pub usage_percent: f64,
    pub is_removable: bool,
}

#[tauri::command]
pub fn get_disks_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();

    disks
        .iter()
        .map(|disk| {
            let total = disk.total_space() as f64;
            let available = disk.available_space() as f64;
            let used = total - available;
            let usage_percent = if total > 0.0 {
                (used / total) * 100.0
            } else {
                0.0
            };

            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                kind: match disk.kind() {
                    DiskKind::HDD => "HDD".to_string(),
                    DiskKind::SSD => "SSD".to_string(),
                    DiskKind::Unknown(v) => format!("Unknown({})", v),
                },
                file_system: disk.file_system().to_string_lossy().to_string(),
                total_gb:        (total     / 1_073_741_824.0 * 100.0).round() / 100.0,
                used_gb:         (used      / 1_073_741_824.0 * 100.0).round() / 100.0,
                available_gb:    (available / 1_073_741_824.0 * 100.0).round() / 100.0,
                usage_percent:   (usage_percent * 100.0).round() / 100.0,
                is_removable: disk.is_removable(),
            }
        })
        .collect()
}

#[derive(Serialize)]
pub struct PhysicalDiskInfo {
    pub model: String,
    pub serial_number: String,
    pub size_gb: f64,
}

#[tauri::command]
pub fn get_physical_disks_info() -> Vec<PhysicalDiskInfo> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                r#"Get-CimInstance Win32_DiskDrive | Select-Object Model,SerialNumber,Size | ConvertTo-Json -Compress"#
            ])
            .output();

        if let Ok(output) = output {
            let json_str = String::from_utf8_lossy(&output.stdout);

            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let mut results = vec![];

                if let Some(arr) = value.as_array() {
                    for v in arr {
                        results.push(parse_disk_value(v));
                    }
                } else if value.is_object() {
                    results.push(parse_disk_value(&value));
                }

                return results;
            }
        }
        vec![]
    }
    #[cfg(not(target_os = "windows"))]
    {
        vec![]
    }
}

#[cfg(target_os = "windows")]
fn parse_disk_value(v: &serde_json::Value) -> PhysicalDiskInfo {
    let model = v["Model"].as_str().unwrap_or("Unknown").to_string();
    let serial = v["SerialNumber"].as_str().unwrap_or("Unknown").trim().to_string();
    let size = v["Size"].as_u64().unwrap_or(0);

    PhysicalDiskInfo {
        model,
        serial_number: serial,
        size_gb: (size as f64 / 1_073_741_824.0 * 100.0).round() / 100.0,
    }
}
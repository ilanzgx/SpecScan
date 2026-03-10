use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}

#[derive(Serialize)]
pub struct PhysicalMemorySlot {
    pub manufacturer: String,
    pub part_number: String,
    pub capacity_gb: f64,
    pub speed_mhz: u32,
    pub configured_voltage: f64,
    pub memory_type: String,
    pub form_factor: String,
}

#[tauri::command]
pub fn get_memory_info() -> MemoryInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();

    MemoryInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        free_memory: sys.free_memory(),
    }
}

#[tauri::command]
pub fn get_physical_memory_info() -> Vec<PhysicalMemorySlot> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                r#"Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer,PartNumber,Capacity,Speed,ConfiguredVoltage,SMBIOSMemoryType,FormFactor | ConvertTo-Json -Compress"#
            ])
            .output();

        if let Ok(output) = output {
            let json_str = String::from_utf8_lossy(&output.stdout);

            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let mut results = vec![];

                if let Some(arr) = value.as_array() {
                    for v in arr {
                        results.push(parse_ram_value(v));
                    }
                } else if value.is_object() {
                    results.push(parse_ram_value(&value));
                }

                return results;
            }
        }
        vec![]
    }
    #[cfg(not(target_os = "windows"))]
    {
        vec![] // Fallback linux/macos
    }
}

#[cfg(target_os = "windows")]
fn parse_ram_value(v: &serde_json::Value) -> PhysicalMemorySlot {
    let manufacturer = v["Manufacturer"].as_str().unwrap_or("Unknown").trim().to_string();
    let part_number = v["PartNumber"].as_str().unwrap_or("Unknown").trim().to_string();
    let capacity = v["Capacity"].as_u64().unwrap_or(0);
    let speed_mhz = v["Speed"].as_u64().unwrap_or(0) as u32;

    // ConfiguredVoltage is given in millivolts
    let voltage_mv = v["ConfiguredVoltage"].as_u64().unwrap_or(0);
    let configured_voltage = if voltage_mv > 0 { voltage_mv as f64 / 1000.0 } else { 0.0 };

    // SMBIOS Memory Type
    let smbios_mem_type = v["SMBIOSMemoryType"].as_u64().unwrap_or(0);
    let memory_type = match smbios_mem_type {
        20 => "DDR",
        21 => "DDR2",
        24 => "DDR3",
        26 => "DDR4",
        30 => "LPDDR4",
        34 => "DDR5",
        35 => "LPDDR5",
        _ => "Unknown",
    }.to_string();

    let form_factor_code = v["FormFactor"].as_u64().unwrap_or(0);
    let form_factor = match form_factor_code {
        8 => "DIMM".to_string(),
        12 => "SODIMM".to_string(),
        _ => format!("Unknown({})", form_factor_code),
    };

    PhysicalMemorySlot {
        manufacturer,
        part_number,
        capacity_gb: (capacity as f64 / 1_073_741_824.0 * 100.0).round() / 100.0,
        speed_mhz,
        configured_voltage,
        memory_type,
        form_factor,
    }
}

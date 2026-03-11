use serde::Serialize;
use sysinfo::Networks;

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub mac_address: String,
}

#[derive(Serialize)]
pub struct NetworkAdapterInfo {
    pub name: String,
    pub manufacturer: String,
    pub mac_address: String,
    pub speed_mbps: f64,
    pub connection_id: String,
    pub adapter_type: String,
}

/// Live network traffic stats via sysinfo (instant, no overhead)
#[tauri::command]
pub fn get_network_info() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();

    networks
        .iter()
        .map(|(name, data)| {
            NetworkInfo {
                interface_name: name.clone(),
                received_bytes: data.total_received(),
                transmitted_bytes: data.total_transmitted(),
                mac_address: data.mac_address().to_string(),
            }
        })
        .collect()
}

/// Physical network adapters via PowerShell/WMI (static data, one-time call)
#[tauri::command]
pub fn get_network_adapters_info() -> Vec<NetworkAdapterInfo> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        use std::os::windows::process::CommandExt;

        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "-NoProfile",
                "-Command",
                r#"Get-CimInstance Win32_NetworkAdapter | Where-Object { $_.PhysicalAdapter -eq $true } | Select-Object Name,Manufacturer,MACAddress,Speed,NetConnectionID,AdapterType | ConvertTo-Json -Compress"#
            ])
            .output();

        if let Ok(output) = output {
            let json_str = String::from_utf8_lossy(&output.stdout);

            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let mut results = vec![];

                if let Some(arr) = value.as_array() {
                    for v in arr {
                        results.push(parse_adapter_value(v));
                    }
                } else if value.is_object() {
                    results.push(parse_adapter_value(&value));
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
fn parse_adapter_value(v: &serde_json::Value) -> NetworkAdapterInfo {
    let name = v["Name"].as_str().unwrap_or("Unknown").to_string();
    let manufacturer = v["Manufacturer"].as_str().unwrap_or("Unknown").to_string();
    let mac_address = v["MACAddress"].as_str().unwrap_or("N/A").to_string();
    let connection_id = v["NetConnectionID"].as_str().unwrap_or("N/A").to_string();
    let adapter_type = v["AdapterType"].as_str().unwrap_or("Unknown").to_string();

    let speed = v["Speed"].as_u64().unwrap_or(0);
    let speed_mbps = (speed as f64 / 1_000_000.0 * 100.0).round() / 100.0;

    NetworkAdapterInfo {
        name,
        manufacturer,
        mac_address,
        speed_mbps,
        connection_id,
        adapter_type,
    }
}

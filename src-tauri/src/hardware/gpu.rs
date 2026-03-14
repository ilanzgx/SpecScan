use serde::Serialize;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Serialize)]
pub struct GpuInfo {
    pub name: String,
    pub manufacturer: String,
    pub video_processor: String,
    pub driver_version: String,
    pub driver_date: String,
    pub adapter_ram_gb: f64,
    pub adapter_dac_type: String,
    pub resolution: String,
    pub bits_per_pixel: u32,
    pub refresh_rate: u32,
    pub max_refresh_rate: u32,
    pub min_refresh_rate: u32,
    pub video_mode_description: String,
}

#[tauri::command]
pub fn get_gpu_info() -> Vec<GpuInfo> {
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
                r#"Get-CimInstance Win32_VideoController | Select-Object Name,AdapterCompatibility,VideoProcessor,DriverVersion,DriverDate,AdapterRAM,AdapterDACType,CurrentHorizontalResolution,CurrentVerticalResolution,CurrentBitsPerPixel,CurrentRefreshRate,MaxRefreshRate,MinRefreshRate,VideoModeDescription | ConvertTo-Json -Compress"#
            ])
            .output();

        if let Ok(output) = output {
            let json_str = String::from_utf8_lossy(&output.stdout);

            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let mut results = vec![];

                if let Some(arr) = value.as_array() {
                    for v in arr {
                        results.push(parse_gpu_value(v));
                    }
                } else if value.is_object() {
                    results.push(parse_gpu_value(&value));
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
fn parse_gpu_value(v: &serde_json::Value) -> GpuInfo {
    let name = v["Name"].as_str().unwrap_or("Unknown").to_string();
    let manufacturer = v["AdapterCompatibility"].as_str().unwrap_or("Unknown").to_string();
    let video_processor = v["VideoProcessor"].as_str().unwrap_or("Unknown").to_string();
    let driver_version = v["DriverVersion"].as_str().unwrap_or("Unknown").to_string();
    let adapter_dac_type = v["AdapterDACType"].as_str().unwrap_or("Unknown").to_string();
    let video_mode_description = v["VideoModeDescription"].as_str().unwrap_or("Unknown").to_string();

    // DriverDate comes as "/Date(timestamp)/" format from PowerShell
    let driver_date = if let Some(date_str) = v["DriverDate"].as_str() {
        // Parse "/Date(1741219200000)/" format
        let cleaned = date_str.replace("/Date(", "").replace(")/", "");
        if let Ok(timestamp_ms) = cleaned.parse::<i64>() {
            let secs = timestamp_ms / 1000;
            let days = secs / 86400;
            // Simple date calc from Unix epoch (1970-01-01)
            let (year, month, day) = unix_days_to_date(days);
            format!("{:04}-{:02}-{:02}", year, month, day)
        } else {
            date_str.to_string()
        }
    } else {
        "Unknown".to_string()
    };

    let horiz = v["CurrentHorizontalResolution"].as_u64().unwrap_or(0);
    let vert = v["CurrentVerticalResolution"].as_u64().unwrap_or(0);
    let resolution = if horiz > 0 && vert > 0 {
        format!("{}x{}", horiz, vert)
    } else {
        "Unknown".to_string()
    };

    let bits_per_pixel = v["CurrentBitsPerPixel"].as_u64().unwrap_or(0) as u32;
    let refresh_rate = v["CurrentRefreshRate"].as_u64().unwrap_or(0) as u32;
    let max_refresh_rate = v["MaxRefreshRate"].as_u64().unwrap_or(0) as u32;
    let min_refresh_rate = v["MinRefreshRate"].as_u64().unwrap_or(0) as u32;

    let ram = v["AdapterRAM"].as_u64().unwrap_or(0);
    let adapter_ram_gb = (ram as f64 / 1_073_741_824.0 * 100.0).round() / 100.0;

    GpuInfo {
        name,
        manufacturer,
        video_processor,
        driver_version,
        driver_date,
        adapter_ram_gb,
        adapter_dac_type,
        resolution,
        bits_per_pixel,
        refresh_rate,
        max_refresh_rate,
        min_refresh_rate,
        video_mode_description,
    }
}

#[cfg(target_os = "windows")]
fn unix_days_to_date(days: i64) -> (i64, u32, u32) {
    // Algorithm to convert days since Unix epoch to (year, month, day)
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year, m, d)
}

#[derive(Serialize, Default)]
pub struct GpuBenchmark {
    pub ranking: String,
    pub g3d_score: String,
    pub g2d_score: String,
    pub price: String,
    pub release_date: String,
    pub tdp: String,
    pub core_clock: String,
    pub memory_clock: String,
    pub vram: String,
}

#[tauri::command]
pub fn get_gpu_benchmark(name: String) -> GpuBenchmark {
    let csv_path = "resources/gpus.csv";

    let file = match File::open(csv_path) {
        Ok(f) => f,
        Err(_) => return GpuBenchmark::default(),
    };

    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    let search_name = name.to_lowercase()
        .replace("(tm)", "")
        .replace("(r)", "")
        .replace("(c)", "")
        .replace(" graphics", "");

    for record in rdr.records().flatten() {
        let model_name = record.get(1).unwrap_or("").to_lowercase();

        if search_name.contains(&model_name) || model_name.contains(&search_name) {
            return GpuBenchmark {
                ranking: record.get(0).unwrap_or("N/A").to_string(),
                g3d_score: record.get(4).unwrap_or("0").to_string(),
                g2d_score: record.get(5).unwrap_or("0").to_string(),
                price: record.get(6).unwrap_or("NA").to_string(),
                release_date: record.get(3).unwrap_or("N/A").to_string(),
                tdp: record.get(8).unwrap_or("NA").to_string(),
                core_clock: record.get(9).unwrap_or("N/A").to_string(),
                memory_clock: record.get(10).unwrap_or("N/A").to_string(),
                vram: record.get(11).unwrap_or("N/A").to_string(),
            };
        }
    }

    GpuBenchmark::default()
}

use serde::Serialize;
use sysinfo::System;
use raw_cpuid::CpuId;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Serialize)]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub brand: String,
    pub physical_cores: u32,
    pub logical_cores: u32,
    pub base_frequency_mhz: u64,
    pub max_frequency_mhz: u64,
    pub cache_l1_kb: u32,
    pub cache_l2_kb: u32,
    pub cache_l3_kb: u32,
    pub family: String,
    pub features: Vec<String>,
    pub socket_designation: String,
    pub voltage_v: f64,
    pub is_throttling: bool,
    pub current_clock_mhz: u64,
    pub has_htt: bool,
    pub max_logical_processor_ids: u8,
    pub has_vmx: bool,
    pub has_svm: bool,
    pub has_64bit_mode: bool,
    pub has_execute_disable: bool,
    pub has_1gib_pages: bool,
    pub has_rdtscp: bool,
    pub has_invariant_tsc: bool,
    pub pmu_version: u8,
    pub pmu_counters: u8,
    pub pmu_counter_width: u8,
    pub has_dts: bool,
    pub has_turbo_boost: bool,
    pub has_hwp: bool,
    pub has_hdc: bool,
    pub thermal_thresholds: u8,
    pub has_pln: bool,
}

#[tauri::command]
pub fn get_cpu_info() -> CpuInfo {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpuid = CpuId::new();

    let vendor = cpuid
        .get_vendor_info()
        .map(|vi| vi.as_str().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let brand = cpuid
        .get_processor_brand_string()
        .map(|s| s.as_str().trim().to_string())
        .unwrap_or_else(|| sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string()));

    let name = sys.cpus().first().map(|cpu| cpu.name().to_string()).unwrap_or_else(|| "Unknown".to_string());

    let mut cache_l1_kb = 0;
    let mut cache_l2_kb = 0;
    let mut cache_l3_kb = 0;

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = (cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets()) as u32;

            match cache.level() {
                1 => cache_l1_kb += size / 1024,
                2 => cache_l2_kb += size / 1024,
                3 => cache_l3_kb += size / 1024,
                _ => {}
            }
        }
    }

    let mut features = Vec::new();
    let mut family_str = "Family N/A - Model N/A - Stepping N/A".to_string();

    let mut has_htt = false;
    let mut max_logical_processor_ids = 0;
    let mut has_vmx = false;

    if let Some(fi) = cpuid.get_feature_info() {
        family_str = format!(
            "Family {} - Model {} - Stepping {}",
            fi.family_id() + (fi.extended_family_id() << 4),
            fi.model_id() + (fi.extended_model_id() << 4),
            fi.stepping_id()
        );

        has_htt = fi.has_htt();
        max_logical_processor_ids = fi.max_logical_processor_ids();
        has_vmx = fi.has_vmx();

        if fi.has_avx() { features.push("AVX".to_string()); }
        if fi.has_sse() { features.push("SSE".to_string()); }
        if fi.has_sse2() { features.push("SSE2".to_string()); }
        if fi.has_sse3() { features.push("SSE3".to_string()); }
        if fi.has_ssse3() { features.push("SSSE3".to_string()); }
        if fi.has_sse41() { features.push("SSE4.1".to_string()); }
        if fi.has_sse42() { features.push("SSE4.2".to_string()); }
        if fi.has_fma() { features.push("FMA".to_string()); }
        if fi.has_aesni() { features.push("AES".to_string()); }
        if fi.has_hypervisor() { features.push("Hypervisor".to_string()); }
    }

    if let Some(ext) = cpuid.get_extended_feature_info() {
        if ext.has_avx2() { features.push("AVX2".to_string()); }
        if ext.has_avx512f() { features.push("AVX-512".to_string()); }
    }

    let mut has_svm = false;
    let mut has_64bit_mode = false;
    let mut has_execute_disable = false;
    let mut has_1gib_pages = false;
    let mut has_rdtscp = false;
    let mut has_invariant_tsc = false;
    if let Some(ext) = cpuid.get_extended_processor_and_feature_identifiers() {
        has_svm = ext.has_svm();
        has_64bit_mode = ext.has_64bit_mode();
        has_execute_disable = ext.has_execute_disable();
        has_1gib_pages = ext.has_1gib_pages();
        has_rdtscp = ext.has_rdtscp();
    }

    if let Some(apm) = cpuid.get_advanced_power_mgmt_info() {
        has_invariant_tsc = apm.has_invariant_tsc();
    }

    let mut pmu_version = 0;
    let mut pmu_counters = 0;
    let mut pmu_counter_width = 0;
    if let Some(pmu) = cpuid.get_performance_monitoring_info() {
        pmu_version = pmu.version_id();
        pmu_counters = pmu.number_of_counters();
        pmu_counter_width = pmu.counter_bit_width();
    }

    let mut has_dts = false;
    let mut has_turbo_boost = false;
    let mut has_hwp = false;
    let mut has_hdc = false;
    let mut thermal_thresholds = 0;
    let mut has_pln = false;
    if let Some(tpi) = cpuid.get_thermal_power_info() {
        has_dts = tpi.has_dts();
        has_turbo_boost = tpi.has_turbo_boost();
        has_hwp = tpi.has_hwp();
        has_hdc = tpi.has_hdc();
        thermal_thresholds = tpi.dts_irq_threshold();
        has_pln = tpi.has_pln();
    }

    #[allow(unused_mut)]
    let mut base_frequency_mhz = cpuid
        .get_processor_frequency_info()
        .map(|f| f.processor_base_frequency() as u64)
        .unwrap_or(0);

    #[allow(unused_mut)]
    let mut max_frequency_mhz = cpuid
        .get_processor_frequency_info()
        .map(|f| f.processor_max_frequency() as u64)
        .unwrap_or(0);

    #[allow(unused_mut)]
    let mut voltage_v = 0.0;
    #[allow(unused_mut)]
    let mut socket_designation = "Unknown".to_string();
    #[allow(unused_mut)]
    let mut current_clock_mhz = 0;
    #[allow(unused_mut)]
    let mut is_throttling = false;

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        use std::os::windows::process::CommandExt;

        const CREATE_NO_WINDOW: u32 = 0x08000000;

        if let Ok(output) = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "-NoProfile",
                "-Command",
                r#"Get-CimInstance Win32_Processor | Select-Object MaxClockSpeed,CurrentClockSpeed,CurrentVoltage,SocketDesignation | ConvertTo-Json -Compress"#
            ])
            .output()
        {
            let json_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                // Handle multiple physical processors if it's an array
                if let Some(arr) = value.as_array() {
                    if let Some(first) = arr.first() {
                        value = first.clone();
                    }
                }

                if value.is_object() {
                    socket_designation = value["SocketDesignation"].as_str().unwrap_or("Unknown").to_string();

                    if let Some(vol) = value["CurrentVoltage"].as_u64() {
                        let mut v = vol as u16;
                        if v & 0x80 != 0 {
                            v &= 0x7F;
                        }
                        voltage_v = (v as f64) / 10.0;
                    }

                    let max_clock = value["MaxClockSpeed"].as_u64().unwrap_or(0);
                    let curr_clock = value["CurrentClockSpeed"].as_u64().unwrap_or(0);
                    current_clock_mhz = curr_clock;

                    if base_frequency_mhz == 0 {
                        base_frequency_mhz = max_clock;
                        max_frequency_mhz = max_clock;
                    }

                    is_throttling = curr_clock > 0 && max_clock > 0 && curr_clock < max_clock;
                }
            }
        }
    }

    let logical_cores = sys.cpus().len() as u32;
    let physical_cores = sysinfo::System::physical_core_count().unwrap_or(logical_cores as usize) as u32;

    // Use sysinfo frequency as fallback if WMI failed
    let sys_curr_freq = sys.cpus().first().map(|cpu| cpu.frequency()).unwrap_or(0);
    if current_clock_mhz == 0 {
        current_clock_mhz = sys_curr_freq;
    }

    CpuInfo {
        name,
        vendor,
        brand,
        physical_cores,
        logical_cores,
        base_frequency_mhz,
        max_frequency_mhz,
        cache_l1_kb,
        cache_l2_kb,
        cache_l3_kb,
        family: family_str,
        features,
        socket_designation,
        voltage_v,
        is_throttling,
        current_clock_mhz,
        has_htt,
        max_logical_processor_ids,
        has_vmx,
        has_svm,
        has_64bit_mode,
        has_execute_disable,
        has_1gib_pages,
        has_rdtscp,
        has_invariant_tsc,
        pmu_version,
        pmu_counters,
        pmu_counter_width,
        has_dts,
        has_turbo_boost,
        has_hwp,
        has_hdc,
        thermal_thresholds,
        has_pln,
    }
}

#[derive(Serialize, Default)]
pub struct CpuBenchmark {
    pub multi_score: String,
    pub single_score: String,
    pub price: String,
    pub ranking: String,
    pub release_date: String,
    pub socket: String,
    pub tdp: String,
    pub cores: String,
}

#[tauri::command]
pub fn get_cpu_benchmark(brand: String) -> CpuBenchmark {
    let csv_path = "resources/cpus.csv";

    let file = match File::open(csv_path) {
        Ok(f) => f,
        Err(_) => return CpuBenchmark::default(),
    };

    let mut rdr = ReaderBuilder::new().from_reader(file);

    let search_name = brand.to_lowercase()
        .replace("(tm)", "")
        .replace("(r)", "")
        .replace("(c)", "")
        .replace(" graphics", "");

    for record in rdr.records().flatten() {
        let model_name = record.get(1).unwrap_or("").to_lowercase();

        if search_name.contains(&model_name) || model_name.contains(&search_name) {
            return CpuBenchmark {
                ranking: record.get(0).unwrap_or("N/A").to_string(),
                multi_score: record.get(4).unwrap_or("0").to_string(),
                single_score: record.get(5).unwrap_or("0").to_string(),
                price: record.get(6).unwrap_or("NA").to_string(),
                release_date: record.get(3).unwrap_or("N/A").to_string(),
                socket: record.get(7).unwrap_or("N/A").to_string(),
                tdp: record.get(8).unwrap_or("NA").to_string(),
                cores: record.get(10).unwrap_or("N/A").to_string(),
            };
        }
    }

    CpuBenchmark::default()
}

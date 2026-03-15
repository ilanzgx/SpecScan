use serde::Serialize;
use smbioslib::DefinedStruct;

#[derive(Serialize)]
pub struct BiosInfo {
    pub vendor: String,
    pub version: String,
    pub release_date: String,
}

#[derive(Serialize)]
pub struct MemorySlot {
    pub slot_label: String,
    pub bank_label: String,
    pub manufacturer: String,
    pub serial_number: String,
    pub part_number: String,
    pub size_mb: u64,
    pub speed_mhz: u32,
    pub memory_type: String,
    pub form_factor: String,
    pub configured_voltage: String,
}

#[derive(Serialize)]
pub struct MotherboardInfo {
    pub manufacturer: String,
    pub product: String,
    pub version: String,
    pub serial_number: String,
    pub asset_tag: String,
    pub bios: BiosInfo,
    pub memory_slots: Vec<MemorySlot>,
}

#[tauri::command]
pub fn get_motherboard_info() -> MotherboardInfo {
    let mut manufacturer = "Unknown".to_string();
    let mut product = "Unknown".to_string();
    let mut version = "Unknown".to_string();
    let mut serial_number = "Unknown".to_string();
    let mut asset_tag = "Unknown".to_string();
    let mut bios = BiosInfo {
        vendor: "Unknown".to_string(),
        version: "Unknown".to_string(),
        release_date: "Unknown".to_string(),
    };
    let mut memory_slots: Vec<MemorySlot> = Vec::new();

    if let Ok(data) = smbioslib::table_load_from_device() {
        for structure in data.iter() {
            match structure.defined_struct() {
                DefinedStruct::BaseBoardInformation(board) => {
                    manufacturer = board.manufacturer().to_string();
                    product = board.product().to_string();
                    version = board.version().to_string();
                    serial_number = board.serial_number().to_string();
                    asset_tag = board.asset_tag().to_string();
                }
                DefinedStruct::Information(b) => {
                    bios = BiosInfo {
                        vendor: b.vendor().to_string(),
                        version: b.version().to_string(),
                        release_date: b.release_date().to_string(),
                    };
                }
                DefinedStruct::MemoryDevice(mem) => {
                    let size_mb = match mem.size() {
                        Some(smbioslib::MemorySize::Megabytes(mb)) => mb as u64,
                        Some(smbioslib::MemorySize::Kilobytes(kb)) => kb as u64 / 1024,
                        _ => 0,
                    };

                    let speed_mhz = match mem.speed() {
                        Some(smbioslib::MemorySpeed::MTs(mts)) => mts as u32,
                        Some(smbioslib::MemorySpeed::Unknown) => 0,
                        _ => 0,
                    };

                    memory_slots.push(MemorySlot {
                        slot_label: mem.device_locator().to_string(), // Ex: DIMM_A1, DIMM_A2, DIMM_B1, DIMM_B2
                        bank_label: mem.bank_locator().to_string(),   // Ex: BANK_A, P0 CHANNEL A
                        manufacturer: mem.manufacturer().to_string(), // Ex: Samsung, Kingston, Corsair
                        serial_number: mem.serial_number().to_string(), // Ex: 123456789
                        part_number: mem.part_number().to_string(),   // Ex: 123456789
                        size_mb,                                      // Ex: 8192
                        speed_mhz,                                    // Ex: 2666
                        memory_type: format!("{:?}", mem.memory_type()), // Ex: DDR4
                        form_factor: format!("{:?}", mem.form_factor()), // Ex: DIMM
                        configured_voltage: format!("{:?}", mem.configured_voltage()), // Ex: 1.2V
                    });
                }
                _ => {}
            }
        }
    }

    MotherboardInfo {
        manufacturer,  // Ex: Gigabyte, Asus, MSI
        product,       // Ex: B450M DS3H
        version,       // Ex: F1
        serial_number, // Ex: 123456789
        asset_tag,     // Ex: 123456789
        bios,          // Ex: BiosInfo
        memory_slots,  // Ex: Vec<MemorySlot>
    }
}

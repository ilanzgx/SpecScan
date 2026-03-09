use serde::Serialize;


use smbioslib::DefinedStruct;

#[derive(Serialize)]
pub struct MotherboardInfo {
    pub manufacturer: String,
    pub product: String,
    pub version: String,
}

#[tauri::command]
pub fn get_motherboard_info() -> MotherboardInfo {
    let data_result = smbioslib::table_load_from_device();

    if let Ok(data) = data_result {
        for structure in data.iter() {
            if let DefinedStruct::BaseBoardInformation(board) = structure.defined_struct() {
                return MotherboardInfo {
                    manufacturer: board.manufacturer().to_string(),
                    product: board.product().to_string(),
                    version: board.version().to_string(),
                };
            }
        }
    }

    MotherboardInfo {
        manufacturer: "Unknown".into(),
        product: "Unknown".into(),
        version: "Unknown".into(),
    }
}
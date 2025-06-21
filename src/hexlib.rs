pub mod hex_manager;
pub mod memory_map;
pub mod range;
pub mod intel_hex;
pub mod utility;

use std::path::PathBuf;

pub struct HexLib {
    pub data: hex_manager::HexManager,
}

enum FileType {
    Unknown,
    IntelHex,
    MotorolaSRecord,
    Binary,
}

impl HexLib {
    pub fn new() -> HexLib {
        HexLib {
            data: hex_manager::HexManager::new(),
        }
    }

    fn get_file_type(file_path: &str) -> FileType {
        let path = PathBuf::from(file_path);
        match path.extension().and_then(|s| s.to_str()) {
            Some("hex") => FileType::IntelHex,
            Some("srec") => FileType::MotorolaSRecord,
            Some("bin") => FileType::Binary,
            _ => FileType::Unknown,
        }
    }

    pub fn read_file(&mut self, file_path: &str) -> Result<(), String> {
        match Self::get_file_type(file_path) {
            FileType::IntelHex => intel_hex::read_intelhex_file(file_path, &mut self.data),
            FileType::MotorolaSRecord => Err("Motorola S-Record format not supported yet.".to_string()),
            FileType::Binary => Err("Binary format not supported yet.".to_string()),
            FileType::Unknown => Err("Unknown file format.".to_string()),
        }
    }

    pub fn write_file(&self, file_path: &str, ranges: &Vec<range::AddressRange>) -> Result<(), String> {
        match Self::get_file_type(file_path) {
            FileType::IntelHex => intel_hex::write_intelhex_file(file_path, &self.data, &ranges),
            FileType::MotorolaSRecord => Err("Motorola S-Record format not supported yet.".to_string()),
            FileType::Binary => Err("Binary format not supported yet.".to_string()),
            FileType::Unknown => Err("Unknown file format.".to_string()),
        }
    }

    pub fn show_hex_data(&self, start: u32, end: u32) {
        let data = self.data.get_data(start, (end - start) as usize);
        for (i, byte) in data.iter().enumerate() {
            if i % 16 == 0 {
                print!("{:08X}: ", start + i as u32);
            }
            print!("{:02X} ", byte);
            if i % 16 == 15 {
                println!();
            }
        }
        if data.len() % 16 != 0 {
            println!();
        }
    }

    pub fn show_hex_range(&self) {
        let ranges = self.data.get_address_range();
        println!("Start Address - End Address");
        println!("-------------------------");
        for range in ranges {
            println!("{:08X} - {:08X}", range.start, range.end);
        }
    }
}
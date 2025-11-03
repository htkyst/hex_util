pub mod data_type;
pub mod file_io;
pub mod utility;

use utility::data_buffer::DataBuffer;
use data_type::range::AddressRange;
use std::path::PathBuf;

enum FileType {
    Unknown,
    IntelHex,
    MotorolaSRecord,
    Binary,
}

pub struct HexLib {
    pub data_buffer: DataBuffer,
}

impl HexLib {
    /**
     * Create a new HexLib instance
     */
    pub fn new() -> HexLib {
        HexLib {
            data_buffer: DataBuffer::new(),
        }
    }

    /**
     * Write data to the buffer
     */
    pub fn write_buffer(&mut self, address: u32, data: Vec<u8>) {
        self.data_buffer.set_data(address, data)
    }

    /**
     * Read data from the buffer
     */
    pub fn read_buffer(&self, address: u32, size: usize) -> Vec<u8> {
        self.data_buffer.get_data(address, size)
    }

    fn get_file_type(file_path: &str) -> FileType {
        let path = PathBuf::from(file_path);
        match path.extension().and_then(|s| s.to_str()) {
            Some("hex") => FileType::IntelHex,
            Some("srec") => FileType::MotorolaSRecord,
            Some("mot") => FileType::MotorolaSRecord,
            Some("bin") => FileType::Binary,
            _ => FileType::Unknown,
        }
    }

    pub fn read_file(&mut self, file_path: &str) -> Result<(), String> {
        match Self::get_file_type(file_path) {
            FileType::IntelHex => file_io::intel_hex::read_intelhex_file(file_path, &mut self.data_buffer),
            FileType::MotorolaSRecord => file_io::srecord::read_srecord_file(file_path, &mut self.data_buffer),
            FileType::Binary => Err("Binary format not supported yet.".to_string()),
            FileType::Unknown => Err("Unknown file format.".to_string()),
        }
    }

    pub fn write_file(&self, file_path: &str, ranges: &Vec<AddressRange>) -> Result<(), String> {
        match Self::get_file_type(file_path) {
            FileType::IntelHex => file_io::intel_hex::write_intelhex_file(file_path, &self.data_buffer, &ranges),
            FileType::MotorolaSRecord => file_io::srecord::write_srecord_file(file_path, &self.data_buffer, &ranges),
            FileType::Binary => Err("Binary format not supported yet.".to_string()),
            FileType::Unknown => Err("Unknown file format.".to_string()),
        }
    }

    pub fn show_hex_data(&self, start: u32, end: u32) {
        let data = self.data_buffer.get_data(start, (end - start) as usize);
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
        let ranges = self.data_buffer.get_data_ranges();
        println!("Start Address - End Address");
        println!("-------------------------");
        for range in ranges {
            println!("{:08X} - {:08X}", range.start, range.end);
        }
    }
}
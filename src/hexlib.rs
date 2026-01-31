pub mod fileio;
pub mod memory;
pub mod utility;

use memory::range::AddressRange;
use memory::memory_map::MemoryMap;
use std::path::PathBuf;

pub struct HexDataBuffer {
    pub memmap: MemoryMap,
    pub ranges: Vec<AddressRange>,
}

impl HexDataBuffer {
    /**
     * Create a new HexDataBuffer instance
     */
    pub fn new() -> HexDataBuffer {
        HexDataBuffer {
            memmap: MemoryMap::new(4 * 1024 * 1024 * 1024, 4 * 1024), // 4GB size, 4KB sector
            ranges: Vec::new(),
        }
    }

    pub fn get_memory_map(&self) -> &MemoryMap {
        &self.memmap
    }

    pub fn get_address_ranges(&self) -> &Vec<AddressRange> {
        &self.ranges
    }

    pub fn take_address_ranges(&mut self) -> Vec<AddressRange> {
        std::mem::take(&mut self.ranges)
    }
}

enum FileType {
    Unknown,
    IntelHex,
    MotorolaSRecord,
    Binary,
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

pub fn read_file(file_path: &str, offset: usize, buffer: &mut HexDataBuffer) -> Result<(), String> {
    match get_file_type(file_path) {
        FileType::IntelHex => fileio::intel_hex::read_intelhex_file(file_path, &mut buffer.memmap, &mut buffer.ranges),
        FileType::MotorolaSRecord => fileio::srecord::read_srecord_file(file_path, &mut buffer.memmap, &mut buffer.ranges),
        FileType::Binary => fileio::raw_binary::read_binary_file(file_path, offset, &mut buffer.memmap, &mut buffer.ranges),
        FileType::Unknown => Err("Unknown file format.".to_string()),
    }
}

pub fn write_file(file_path: &str, memory_map: &MemoryMap, write_ranges: &Vec<AddressRange>) -> Result<(), String> {
    match get_file_type(file_path) {
        FileType::IntelHex => fileio::intel_hex::write_intelhex_file(file_path, memory_map, write_ranges),
        FileType::MotorolaSRecord => fileio::srecord::write_srecord_file(file_path, memory_map, write_ranges),
        FileType::Binary => fileio::raw_binary::write_binary_file(file_path, memory_map, write_ranges.first().unwrap()),
        FileType::Unknown => Err("Unknown file format.".to_string()),
    }
}

pub fn show_hex_data(memory_map: &MemoryMap, start: u32, end: u32) {
    let data = memory_map.get_bytes(start, (end - start) as usize);
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

pub fn show_hex_range(address_ranges: &Vec<AddressRange>) {
    println!("Start Address - End Address");
    println!("-------------------------");
    for range in address_ranges {
        println!("{:08X} - {:08X}", range.start, range.end);
    }
}
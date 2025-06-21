use crate::hexlib::range::AddressRange;
use crate::hexlib::memory_map::MemoryMap;

pub struct HexManager {
    address_ranges: Vec<AddressRange>,
    memory_map: MemoryMap,
}

impl HexManager {
    pub fn new() -> HexManager {
        let size: usize = 4 * 1024 * 1024 * 1024;   // 4GB
        let sector_size = 1024 * 4;                 // 4KB
        let address_ranges: Vec<AddressRange> = Vec::new();
        let memory_map = MemoryMap::new(size, sector_size);

        HexManager {
            address_ranges,
            memory_map,
        }
    }

    pub fn set_data(&mut self, start_addr: u32, data: Vec<u8>) {
        let end_addr = start_addr + data.len() as u32 - 1;
        self.address_ranges.push(AddressRange::new(
            start_addr,
            end_addr,
        ));
        self.memory_map.set_bytes(start_addr, data);
    }

    pub fn get_data(&self, start_addr: u32, size: usize) -> Vec<u8> {
        self.memory_map.get_bytes(start_addr, size)
    }

    pub fn get_address_range(&self) -> &Vec<AddressRange> {
        &self.address_ranges
    }
}

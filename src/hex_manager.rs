#[path = "range.rs"]
mod range;
#[path = "memory_map.rs"]
mod memory_map;

use range::Range;
use memory_map::MemoryMap;

pub struct HexManager {
    address_ranges: Vec<Range>,
    memory_map: MemoryMap,
}

impl HexManager {
    pub fn new() -> HexManager {
        let size: usize = 4 * 1024 * 1024 * 1024;   // 4GB
        let sector_size = 1024 * 4;                 // 4KB
        let address_ranges: Vec<Range> = Vec::new();
        let memory_map = MemoryMap::new(size, sector_size);

        HexManager {
            address_ranges,
            memory_map,
        }
    }

    pub fn set_data(&mut self, start_addr: u32, data: Vec<u8>) {
        let end_addr = start_addr + data.len() as u32 - 1;
        self.address_ranges.push(Range::new(
            start_addr,
            end_addr,
        ));
        self.memory_map.set_multi_byte(start_addr, data);
    }

    pub fn get_data(&mut self, start_addr: u32, size: usize) -> Vec<u8> {
        self.memory_map.get_multi_byte(start_addr, size)
    }
}

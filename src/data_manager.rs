use crate::address_range::AddressRange;
use crate::memory_map::MemoryMap;

pub struct HexDataManager {
    address_range: Vec<AddressRange>,
    memory_map: MemoryMap,
}

impl HexDataManager {
    pub fn new() -> HexDataManager {
        let size: usize = 4 * 1024 * 1024 * 1024;
        let sector_size = 1024 * 4;
        let address_range: Vec<AddressRange> = Vec::new();
        let memory_map = MemoryMap::new(size, sector_size);

        HexDataManager {
            address_range,
            memory_map,
        }
    }

    pub fn set_data(&mut self, start_addr: u32, data: Vec<u8>) {
        self.address_range.push(AddressRange::new(
            start_addr,
            start_addr + data.len() as u32,
        ));
        self.memory_map.set_multi_byte(start_addr, data);
    }

    pub fn get_data(&mut self, start_addr: u32, size: usize) -> Vec<u8> {
        self.memory_map.get_multi_byte(start_addr, size)
    }
}

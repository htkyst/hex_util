use crate::hexlib::data_type::memory_map::MemoryMap;
use crate::hexlib::data_type::range::AddressRange;

pub struct DataBuffer {
    data_ranges: Vec<AddressRange>,
    memory_map: MemoryMap,
}

impl DataBuffer {
    /**
     * Create a new DataBuffer instance
     */
    pub fn new() -> DataBuffer {
        let size: usize = 4 * 1024 * 1024 * 1024; // Maximum memory size: 4GB
        let sector_size = 1024 * 4; // Sector size: 4KB
        let data_ranges: Vec<AddressRange> = Vec::new();
        let memory_map = MemoryMap::new(size, sector_size);

        DataBuffer {
            data_ranges,
            memory_map,
        }
    }

    /**
     * Set data at the specified start address
     *
     * @param start_addr Start address to set data
     * @param data Byte datas to set
     */
    pub fn set_data(&mut self, start_addr: u32, data: Vec<u8>) {
        let end_addr = start_addr + data.len() as u32 - 1;
        self.data_ranges.push(AddressRange::new(start_addr, end_addr));
        self.memory_map.set_bytes(start_addr, data);
    }

    /**
     * Get data from the specified start address
     *
     * @param start_addr Start address to get data
     * @param size Number of bytes to get
     */
    pub fn get_data(&self, start_addr: u32, size: usize) -> Vec<u8> {
        self.memory_map.get_bytes(start_addr, size)
    }

    pub fn get_data_ranges(&self) -> &Vec<AddressRange> {
        &self.data_ranges
    }
}

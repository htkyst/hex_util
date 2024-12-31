#[derive(Debug)]
pub struct MemoryMap {
    data: Vec<Vec<u8>>,
    size: usize,
    sector_size: usize,
    sector_num: usize,
}

impl MemoryMap {
    pub fn new(size: usize, sector_size: usize) -> MemoryMap {
        let sector_num: usize = size / sector_size;
        let mut data: Vec<Vec<u8>> = Vec::new();

        for _i in 0..sector_size {
            data.push(Vec::new());
        }

        MemoryMap {
            data,
            size,
            sector_size,
            sector_num,
        }
    }

    fn get_sector_index(&self, address: u32) -> usize {
        let index = address as usize / self.sector_size;
        return index;
    }

    fn get_sector_offset(&self, address: u32) -> usize {
        let offset = address as usize % self.sector_size;
        return offset;
    }

    pub fn set_byte(&mut self, address: u32, data: u8) {
        let index = self.get_sector_index(address);
        let offset = self.get_sector_offset(address);

        match self.data.get_mut(index) {
            Some(elem) => {
                // if not allocate memory
                if elem.is_empty() {
                    elem.resize(self.sector_size, 0xFF);
                }
                elem[offset] = data;
            }
            None => println!("不正なインデックス"),
        }
    }

    pub fn set_multi_byte(&mut self, address: u32, data: Vec<u8>) {
        let index = self.get_sector_index(address);
        let offset = self.get_sector_offset(address);

        match self.data.get_mut(index) {
            Some(elem) => {
                if elem.is_empty() {
                    elem.resize(self.sector_size, 0xFF);
                }
                for i in 0..data.len() {
                    elem[offset + i] = data[i];
                }
            }
            None => println!("不正なインデックス"),
        }
    }

    pub fn get_byte(&mut self, address: u32) -> u8 {
        let index = self.get_sector_index(address);
        let offset = self.get_sector_offset(address);

        match self.data.get(index) {
            Some(elem) => {
                return elem[offset];
            }
            None => {
                println!("不正なインデックス");
                return 0xFF;
            }
        }
    }

    pub fn get_multi_byte(&mut self, address: u32, size: usize) -> Vec<u8> {
        let index = self.get_sector_index(address);
        let offset = self.get_sector_offset(address);

        match self.data.get(index) {
            Some(elem) => {
                return elem[offset..offset + size].to_vec();
            }
            None => {
                println!("不正なインデックス");
                return Vec::new();
            }
        }
    }
}

#[cfg(test)]
mod memory_map_tests {
    use super::*;

    #[test]
    fn set_get_byte_normal() {
        let mut mem_map = MemoryMap::new(0x1000, 0x100);
        mem_map.set_byte(0, 0xAA);
        mem_map.set_byte(0x100, 0xBB);

        assert_eq!(mem_map.get_byte(0x0), 0xAA);
        assert_eq!(mem_map.get_byte(0x100), 0xBB);

        // 値の上書き
        mem_map.set_byte(0, 0xBB);
        mem_map.set_byte(0x100, 0xAA);

        assert_eq!(mem_map.get_byte(0x0), 0xBB);
        assert_eq!(mem_map.get_byte(0x100), 0xAA);
    }

    #[test]
    fn set_get_multi_byte_normal() {
        let mut mem_map = MemoryMap::new(0x1000, 0x100);
        let data: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];

        mem_map.set_multi_byte(0x0, data.clone());
        mem_map.set_multi_byte(0x100, data.clone());

        assert_eq!(mem_map.get_multi_byte(0x0, 8), data.clone());
        assert_eq!(mem_map.get_multi_byte(0x100, 8), data.clone());
    }
}

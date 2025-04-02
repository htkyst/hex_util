#[derive(Debug)]
pub struct MemoryMap {
    data: Vec<Vec<u8>>,
    sector_size: usize,
    sector_num: usize,
}

impl MemoryMap {
    /**
     * MemoryMapの新規作成
     * 
     * @param size メモリサイズ
     * @param sector_size セクタサイズ
     * @return MemoryMap
     */
    pub fn new(size: usize, sector_size: usize) -> MemoryMap {
        // セクタ数の計算
        let sector_num: usize = if sector_size != 0 {
            size / sector_size
        } else {
            size
        };

        let mut data: Vec<Vec<u8>> = Vec::new();
        // セクタ数分のメモリを確保
        for _i in 0..sector_num {
            data.push(Vec::new());
        }

        MemoryMap {
            data,
            sector_size,
            sector_num,
        }
    }

    /**
     * アドレスからセクタインデックスを取得
     */
    fn get_sector_index(&self, address: u32) -> usize {
        let index = address as usize / self.sector_size;
        return index;
    }

    /**
     * アドレスからセクタ内オフセットを取得
     */
    fn get_sector_offset(&self, address: u32) -> usize {
        let offset = address as usize % self.sector_size;
        return offset;
    }

    /**
     * 1バイトデータをセット
     * 
     * @param address アドレス
     * @param data データ
     */
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

    /**
     * 複数バイトデータをセット
     * 
     * @param address アドレス
     * @param data データ
     */
    pub fn set_bytes(&mut self, address: u32, data: Vec<u8>) {
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

    /**
     * 1バイトデータを取得
     * 
     * @param address アドレス
     * @return u8
     */
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

    /**
     * 複数バイトデータを取得
     * 
     * @param address アドレス
     * @param size サイズ
     * @return Vec<u8>
     */
    pub fn get_bytes(&mut self, address: u32, size: usize) -> Vec<u8> {
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
    fn new_test() {
        let mem_map = MemoryMap::new(0x1000, 0x100);

        assert_eq!(mem_map.sector_size, 0x100);
        assert_eq!(mem_map.sector_num, 16);
    }

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

        mem_map.set_bytes(0x0, data.clone());
        mem_map.set_bytes(0x100, data.clone());

        assert_eq!(mem_map.get_bytes(0x0, 8), data.clone());
        assert_eq!(mem_map.get_bytes(0x100, 8), data.clone());
    }
}

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
        let sector_num: usize = if sector_size != 0 { size / sector_size } else { 1 };

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

    fn is_valid_address(&self, address: u32) -> bool {
        return (address as usize) < (self.sector_num * self.sector_size);
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
        if !self.is_valid_address(address) {
            panic!("Invalid address access");
        }

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
            None => {
                panic!("Invalid address access");
            }
        }
    }

    /**
     * 複数バイトデータをセット
     *
     * @param address アドレス
     * @param data データ
     */
    pub fn set_bytes(&mut self, address: u32, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.set_byte(address + i as u32, *byte);
        }
    }

    /**
     * 1バイトデータを取得
     *
     * @param address アドレス
     * @return u8
     */
    pub fn get_byte(&self, address: u32) -> u8 {
        if !self.is_valid_address(address) {
            panic!("Invalid address access");
        }

        let index = self.get_sector_index(address);
        let offset = self.get_sector_offset(address);

        match self.data.get(index) {
            Some(elem) => {
                if elem.get(offset).is_none() {
                    return 0xFF;
                }
                return elem[offset];
            }
            None => {
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
    pub fn get_bytes(&self, address: u32, size: usize) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for i in 0..size {
            data.push(self.get_byte(address + i as u32));
        }
        data
    }
}

#[cfg(test)]
mod memory_map_tests {
    use super::*;

    #[test]
    fn set_get_byte_normal() {
        let mut mem_map = MemoryMap::new(0x1000, 0x100);

        // 正常系 : 書き込みと読み出し
        mem_map.set_byte(0, 0xAA);
        mem_map.set_byte(0x100, 0xBB);

        assert_eq!(mem_map.get_byte(0x0), 0xAA);
        assert_eq!(mem_map.get_byte(0xFF), 0xFF);
        assert_eq!(mem_map.get_byte(0x100), 0xBB);
        assert_eq!(mem_map.get_byte(0x1FF), 0xFF);
        assert_eq!(mem_map.get_byte(0x200), 0xFF);
        assert_eq!(mem_map.get_byte(0xFFF), 0xFF);

        // 正常系 : 上書き
        mem_map.set_byte(0, 0xBB);
        mem_map.set_byte(0x100, 0xAA);

        assert_eq!(mem_map.get_byte(0x0), 0xBB);
        assert_eq!(mem_map.get_byte(0x100), 0xAA);
    }

    #[test]
    #[should_panic]
    fn set_get_byte_abnormal() {
        let mut mem_map = MemoryMap::new(0x1000, 0x100);

        // 異常系 : 不正なアドレスアクセス
        mem_map.get_byte(0x1000);
    }

    #[test]
    fn set_get_multi_byte_normal() {
        let mut mem_map = MemoryMap::new(0x1000, 0x100);
        let data: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let empty_data: Vec<u8> = vec![0xFF; 8];

        mem_map.set_bytes(0x0, data.clone());
        mem_map.set_bytes(0x100, data.clone());

        assert_eq!(mem_map.get_bytes(0x0, 8), data.clone());
        assert_eq!(mem_map.get_bytes(0x100, 8), data.clone());
        assert_eq!(mem_map.get_bytes(0x200, 8), empty_data.clone());
    }

    #[test]
    #[should_panic]
    fn set_get_multi_byte_abnormal() {
        let mut mem_map = MemoryMap::new(0x1000, 0x100);

        // 異常系 : 不正なアドレスアクセス
        mem_map.get_bytes(0xFFFF, 2);
    }
}

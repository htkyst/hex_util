use crate::hexlib::memory::range::AddressRange;
use crate::hexlib::memory::memory_map::MemoryMap;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter, Write};

pub fn read_binary_file(file_path: &str, offset: usize, memory_map: &mut MemoryMap, address_ranges: &mut Vec<AddressRange>) -> Result<(), String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(&mut file);

    let mut current_address = offset as u32;
    loop {
        let mut buf = vec![0u8; 4096];
        let buf_size = reader.read(&mut buf).map_err(|e| e.to_string())?;
        if buf_size == 0 {
            break;
        }

        buf.truncate(buf_size);
        memory_map.set_bytes(current_address, buf);
        address_ranges.push(AddressRange::new(current_address, current_address + buf_size as u32 - 1));

        current_address += buf_size as u32;
    }

    Ok(())
}

pub fn write_binary_file(file_path: &str, memory_map: &MemoryMap, write_range: &AddressRange) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(&mut file);

    let data = memory_map.get_bytes(write_range.start, write_range.size() as usize);
    writer.write_all(&data).map_err(|e| e.to_string())?;

    Ok(())
}
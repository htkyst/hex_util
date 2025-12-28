use crate::hexlib::utility::data_buffer::DataBuffer;
use crate::hexlib::data_type::range::AddressRange;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter, Write};

pub fn read_binary_file(file_path: &str, offset: usize, buffer: &mut DataBuffer) -> Result<(), String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(&mut file);

    let mut current_address = offset as u32;
    loop {
        let mut chunk = vec![0u8; 4096];
        let bytes_read = reader.read(&mut chunk).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            break;
        }
        chunk.truncate(bytes_read);
        buffer.set_data(current_address, chunk);
        current_address += bytes_read as u32;
    }

    Ok(())
}

pub fn write_binary_file(file_path: &str, buffer: &DataBuffer, ranges: &Vec<AddressRange>) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(&mut file);

    for range in ranges {
        let data = buffer.get_data(range.start, (range.end - range.start + 1) as usize);
        if data.is_empty() {
            continue;
        }
        writer.write_all(&data).map_err(|e| e.to_string())?;
    }

    Ok(())
}
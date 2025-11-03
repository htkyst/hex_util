use crate::hexlib::utility::data_buffer::DataBuffer;
use crate::hexlib::data_type::range::AddressRange;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn read_binary_file(file_path: &str, buffer: &mut DataBuffer) -> Result<(), String> {

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
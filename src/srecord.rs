use crate::data_manager::HexDataManager;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct SRecordData {
    address: u16,
    data: Vec<u8>,
    checksum: u8,
}

fn parse_hex_line(line: &str) -> Result<SRecordData, String> {
    if !line.starts_with('S') {
        return Err("Invalid start code".to_string());
    }

    let bytes = hex::decode(&line[2..]).map_err(|e| e.to_string())?;
    const REQUIRED_DATA_NUM: usize = 4;
    if bytes.len() <= REQUIRED_DATA_NUM {
        return Err("Line too short".to_string());
    }

    const MIN_BYTE_NUM: usize = 3;
    let data_len = bytes[0] as usize - MIN_BYTE_NUM;

    let address = u16::from_be_bytes([bytes[1], bytes[2]]);
    let data = bytes[3..3 + data_len].to_vec();
    let checksum = bytes[3 + data_len];

    let mut sum: u16 = 0;
    for i in 0..3 + data_len {
        sum += bytes[i] as u16;
    }
    sum = (!sum & 0xFF) + 1;
    if checksum != sum as u8 {
        return Err("Invalid chechsum".to_string());
    }

    Ok(SRecordData {
        address,
        data,
        checksum,
    })
}

pub fn read_srecord_file(file_path: &str, data_mgr: &mut HexDataManager) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| e.to_string());
    let reader = BufReader::new(file?);

    for l in reader.lines() {
        let line = l.map_err(|e| e.to_string()).unwrap();
        if line.is_empty() {
            continue;
        }

        match parse_hex_line(&line) {
            Ok(record) => {
                data_mgr.set_data(record.address as u32, record.data);
            }
            Err(_err) => {
                return Err("Error: Parse S record".to_string());
            }
        }
    }

    Ok(())
}

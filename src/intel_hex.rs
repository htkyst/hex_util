use crate::data_manager::HexDataManager;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct IntelHexRecordData {
    address: u16,
    data: Vec<u8>,
    checksum: u8,
}

fn parse_hex_line(line: &str) -> Result<IntelHexRecordData, String> {
    if !line.starts_with(':') {
        return Err("Invalid start code".to_string());
    }

    // 16進文字列を数値に変換
    let bytes = hex::decode(&line[1..]).map_err(|e| e.to_string())?;
    const REQUIRED_DATA_NUM: usize = 4; // Byte count(1) + address(2) + record type(1) + checksum(1)
    if bytes.len() <= REQUIRED_DATA_NUM {
        return Err("Line too short".to_string());
    }

    let data_len = bytes[0] as usize;
    if bytes.len() != REQUIRED_DATA_NUM + data_len {
        return Err("Data length mismatch".to_string());
    }

    let address = u16::from_be_bytes([bytes[1], bytes[2]]);
    let data = bytes[4..4 + data_len].to_vec();
    let checksum = bytes[4 + data_len];

    // Calc checksum
    let mut sum: u16 = 0;
    for i in 0..4 + data_len {
        sum += bytes[i] as u16;
    }
    sum = (!sum & 0xFF) + 1;
    if checksum != sum as u8 {
        return Err("Invalid checksum".to_string());
    }

    Ok(IntelHexRecordData {
        address,
        data,
        checksum,
    })
}

pub fn read_intelhex_file(file_path: &str, data_mgr: &mut HexDataManager) -> Result<(), String> {
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
                return Err("Error: Parse intel hex record".to_string());
            }
        }
    }

    Ok(())
}

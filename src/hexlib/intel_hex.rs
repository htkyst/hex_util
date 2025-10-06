use crate::hexlib::hex_manager::HexManager;
use crate::hexlib::range::AddressRange;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
struct IntelHexRecordData {
    record_type: u32,
    address: u16,
    data: Vec<u8>,
}

// Calculate checksum byte for Intel HEX record
fn calc_checksum_byte(bytes: &[u8]) -> u8 {
    let mut sum: u16 = 0;
    for i in bytes {
        sum += *i as u16;
    }
    return ((!sum & 0xFF) + 1) as u8;
}

/**
 * Parse a line of Intel HEX file and return the record data
 *
 * @param line A line from the Intel HEX file
 */
fn parse_hex_line(line: &str) -> Result<IntelHexRecordData, String> {
    if !line.starts_with(':') {
        return Err("Invalid start code".to_string());
    }

    // 16進文字列を数値に変換
    let bytes = hex::decode(&line[1..]).map_err(|e| e.to_string())?;
    const REQUIRED_DATA_NUM: usize = 5; // Byte count(1) + address(2) + record type(1) + checksum(1)
    if bytes.len() < REQUIRED_DATA_NUM {
        return Err("Record line too short".to_string());
    }

    let data_len = bytes[0] as usize;
    if bytes.len() != REQUIRED_DATA_NUM + data_len {
        return Err("Data length mismatch".to_string());
    }

    let address = u16::from_be_bytes([bytes[1], bytes[2]]);
    let record_type = bytes[3] as u32;
    let data = bytes[4..4 + data_len].to_vec();
    let checksum = bytes[4 + data_len];

    // Calc checksum
    if checksum != calc_checksum_byte(&bytes[0..bytes.len() - 1]) {
        return Err("Invalid checksum".to_string());
    }

    Ok(IntelHexRecordData {
        record_type,
        address,
        data,
    })
}

pub fn read_intelhex_file(file_path: &str, manager: &mut HexManager) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| e.to_string());
    let reader = BufReader::new(file?);

    for l in reader.lines() {
        let line = l.map_err(|e| e.to_string()).unwrap();
        if line.is_empty() {
            continue;
        }

        match parse_hex_line(&line) {
            Ok(record) => {
                match record.record_type {
                    0x00 => {
                        // Data record
                        manager.set_data(record.address as u32, record.data);
                    }
                    0x01 => {
                        // End of file record
                        // No action needed, just continue to the next line
                        break;
                    }
                    _ => {
                        // Unsupported record type
                        return Err(format!("Unsupported record type: {}", record.record_type));
                    }
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}

pub fn write_intelhex_file(file_path: &str, manager: &HexManager, ranges: &Vec<AddressRange>) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(&mut file);

    for range in ranges {
        let data = manager.get_data(range.start, (range.end - range.start + 1) as usize);
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(data.len() as u8);
        bytes.push(((range.start >> 8) & 0xFF) as u8);
        bytes.push((range.start & 0xFF) as u8);
        bytes.push(0); // Record type

        for d in data {
            bytes.push(d);
        }

        bytes.push(calc_checksum_byte(bytes.as_slice()));

        let hex_line = format!(":{}\n", hex::encode_upper(bytes));
        writer.write(hex_line.as_bytes()).unwrap();
    }

    Ok(())
}

use crate::hexlib::utility::data_buffer::DataBuffer;
use crate::hexlib::data_type::range::AddressRange;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
struct SRecordData {
    record_type: u32,
    address: u32,
    data: Vec<u8>,
}

// Calculate checksum byte for Intel HEX record
fn calc_checksum_byte(bytes: &[u8]) -> u8 {
    let mut sum: u16 = 0;
    for i in bytes {
        sum += *i as u16;
    }
    return !((sum & 0xFF) as u8);
}

/**
 * Parse a line of S-record file and return the record data
 *
 * @param line A line from the S-record file
 */
fn parse_hex_line(line: String) -> Result<SRecordData, String> {
    // Check if the line starts with 'S'
    if !line.starts_with('S') {
        return Err("Invalid start code".to_string());
    }

    let chars: Vec<char> = line.chars().collect();
    if chars.len() % 2 != 0 {
        return Err("Invalid S-record line length".to_string());
    }
    if chars.len() < 4 {
        // At least 'S' + record type(1) + byte length(2) characters
        return Err("Invalid S-record line length".to_string());
    }

    let record_type = chars[1].to_digit(10).unwrap();
    let bytes = hex::decode(&line[2..]).map_err(|e| e.to_string())?;
    if bytes.len() < 4 {
        // At least record type(1) + byte length(1) + address(2) bytes
        return Err("Record line too short".to_string());
    }

    let data_len = bytes[0] as usize; // address(2) + data(n) + checksum(1)
    if data_len != bytes.len() - 1 {
        // 1 = record type(1)
        return Err("Data length mismatch".to_string());
    }

    let mut address_len = 0;
    let address = match record_type {
        0 | 1 | 5 | 9 => {
            address_len = 2;
            u32::from_be_bytes([0x00, 0x00, bytes[1], bytes[2]])
        }
        2 | 8 => {
            address_len = 3;
            u32::from_be_bytes([0x00, bytes[1], bytes[2], bytes[3]])
        }
        3 | 7 => {
            address_len = 4;
            u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]])
        }
        _ => return Err("Invalid S-record type".to_string()),
    };

    // Data
    let data_start = address_len + 1; // +1 for byte length
    let data_end = bytes.len() - 1; // -1 for checksum
    let data = bytes[data_start..data_end].to_vec();

    // Checksum
    let checksum = bytes[data_end];
    if checksum != calc_checksum_byte(&bytes[0..bytes.len() - 1]) {
        return Err("Invalid chechsum".to_string());
    }

    Ok(SRecordData {
        record_type,
        address,
        data,
    })
}

/**
 * Read S-record file and load data into DataBuffer
 *
 * @param file_path Path to the S-record file
 * @param buffer DataBuffer to load data into
 */
pub fn read_srecord_file(file_path: &str, buffer: &mut DataBuffer) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| e.to_string());
    let reader = BufReader::new(file?);

    let mut line_number = 0;
    for l in reader.lines() {
        let line = l.map_err(|e| e.to_string()).unwrap();
        if line.is_empty() {
            continue;
        }

        match parse_hex_line(line) {
            Ok(record) => match record.record_type {
                1 | 2 | 3 => {
                    buffer.set_data(record.address, record.data);
                }
                _ => {
                    continue;
                }
            },
            Err(err) => {
                return Err(line_number.to_string() + ": " + &err);
            }
        }

        line_number += 1;
    }

    Ok(())
}

/**
 * Write DataBuffer content into S-record file
 *
 * @param file_path Path to the S-record file
 * @param buffer DataBuffer containing data to write
 * @param ranges Address ranges to write
 */
pub fn write_srecord_file(file_path: &str, buffer: &DataBuffer, ranges: &Vec<AddressRange>) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(&mut file);

    for range in ranges {
        let data = buffer.get_data(range.start, (range.end - range.start + 1) as usize);
        if data.is_empty() {
            continue;
        }

        // Determine record type based on address length
        let address_len = if range.start <= 0xFFFF {
            2
        } else if range.start <= 0xFFFFFF {
            3
        } else {
            4
        };
        let record_type = match address_len {
            2 => 'S',
            3 => 'S',
            4 => 'S',
            _ => return Err("Invalid address length".to_string()),
        };

        // Create S-record line
        let mut bytes = vec![data.len() as u8 + address_len as u8 + 1]; // +1 for checksum
        bytes.push(record_type as u8 - '0' as u8); // Record type
        bytes.extend_from_slice(&range.start.to_be_bytes()[..address_len]); // Address
        bytes.extend_from_slice(&data); // Data

        // Calculate checksum
        bytes.push(calc_checksum_byte(bytes.as_slice()));

        // Write to file
        let hex_line = format!("S{}\n", hex::encode_upper(bytes));
        writer.write(hex_line.as_bytes()).map_err(|e| e.to_string())?;
    }

    Ok(())
}

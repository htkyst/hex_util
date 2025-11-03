use crate::hexlib::utility::data_buffer::DataBuffer;
use crate::hexlib::data_type::range::AddressRange;
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
fn parse_hex_line(line: String) -> Result<IntelHexRecordData, String> {
    if !line.starts_with(':') {
        return Err("Invalid start code".to_string());
    }

    // Convert hex string to numeric values
    let byte_line = hex::decode(&line[1..]).map_err(|e| e.to_string())?;

    const REQUIRED_DATA_NUM: usize = 5; // Byte count(1) + address(2) + record type(1) + checksum(1)
    if byte_line.len() < REQUIRED_DATA_NUM {
        return Err("Record line too short".to_string());
    }

    let data_len = byte_line[0] as usize;
    if byte_line.len() != REQUIRED_DATA_NUM + data_len {
        return Err("Data length mismatch".to_string());
    }

    let address = u16::from_be_bytes([byte_line[1], byte_line[2]]);
    let record_type = byte_line[3] as u32;
    let data = byte_line[4..4 + data_len].to_vec();
    let checksum = byte_line[4 + data_len];

    // Calc checksum
    if checksum != calc_checksum_byte(&byte_line[0..byte_line.len() - 1]) {
        return Err("Mismatch checksum".to_string());
    }

    Ok(IntelHexRecordData {
        record_type,
        address,
        data,
    })
}

/**
 * Read Intel HEX file 
 *
 * @param file_path Path to the Intel HEX file
 * @param buffer Data buffer to store read data
 */
pub fn read_intelhex_file(file_path: &str, buffer: &mut DataBuffer) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| e.to_string());
    let reader = BufReader::new(file?);

    let mut extend_address: u32 = 0;
    for l in reader.lines() {
        let line = l.map_err(|e| e.to_string()).unwrap();
        if line.is_empty() {
            continue;
        }

        match parse_hex_line(line) {
            Ok(record) => {
                match record.record_type {
                    0x00 => {
                        // Data record: Record type for actual data content
                        let full_address = extend_address + record.address as u32;
                        buffer.set_data(full_address, record.data);
                    }
                    0x01 => {
                        // End of file record: Indicates the end of the Intel HEX file
                        // No action needed, just exit the loop
                        break;
                    }
                    0x02 => {
                        // Extended segment address record: 
                        extend_address = ((record.data[0] as u32) << 8 | (record.data[1] as u32)) << 4;
                    }
                    0x04 => {
                        // Extended linear address record: 
                        extend_address = ((record.data[0] as u32) << 8 | (record.data[1] as u32)) << 16;
                    }
                    _ => {
                        // Unsupported record type: Any other record types not handled
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

/**
 * Create an Intel HEX record line
 *
 * @param count Byte count
 * @param address Address
 * @param record_type Record type
 * @param data Data bytes
 */
fn make_hex_record_line(count: u8, address: u16, record_type: u8, data: &Vec<u8>) -> String {
    let mut byte_line: Vec<u8> = Vec::new();
    byte_line.push(count);                         // Byte count
    byte_line.push(((address >> 8) & 0xFF) as u8); // Address high byte
    byte_line.push((address & 0xFF) as u8);        // Address low byte    
    byte_line.push(record_type);                    // Record type

    for d in data {
        byte_line.push(*d);
    }

    byte_line.push(calc_checksum_byte(byte_line.as_slice()));

    format!(":{}\n", hex::encode_upper(byte_line))
}

/**
 * Write Intel HEX file 
 *
 * @param file_path Path to the Intel HEX file
 * @param buffer Data buffer to write
 */
pub fn write_intelhex_file(file_path: &str, buffer: &DataBuffer, write_ranges: &Vec<AddressRange>) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(&mut file);

    let mut extend_addr: u32 = 0;
    const LINE_DATA_SIZE: usize = 16; // Number of data bytes per line

    for range in write_ranges {
        // Align start address to LINE_DATA_SIZE boundary
        let start_addr = if range.start % LINE_DATA_SIZE as u32 != 0 {
            range.start - (range.start % LINE_DATA_SIZE as u32)
        } else {
            range.start
        };

        // Write extended linear address record if needed
        let upper_addr = (start_addr >> 16) & 0xFFFF;
        let is_chaned_extended_address = upper_addr != 0 && upper_addr != extend_addr;
        if is_chaned_extended_address {
            let hex_line = make_hex_record_line(2, 0, 4, vec![((upper_addr >> 8) & 0xFF) as u8, (upper_addr & 0xFF) as u8].as_ref());
            writer.write(hex_line.as_bytes()).unwrap();

            extend_addr = upper_addr;
        }

        // Write data in chunks of LINE_DATA_SIZE
        for base_addr in (start_addr..range.end + 1).step_by(LINE_DATA_SIZE) {
            let data = buffer.get_data(base_addr, LINE_DATA_SIZE);
            let hex_line = make_hex_record_line(data.len() as u8, base_addr as u16, 0, &data);

            writer.write(hex_line.as_bytes()).unwrap();
        }
    }

    Ok(())
}

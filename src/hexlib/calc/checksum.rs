
pub fn calc_checksum_u8(bytes: &[u8]) -> u8 {
    let mut sum: u16 = 0;
    for i in bytes {
        sum += *i as u16;
    }
    return !((sum & 0xFF) as u8);
}
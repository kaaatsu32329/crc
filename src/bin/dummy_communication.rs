use crc::crc::CRC;
use crc::crc16::CRC16;
use crc::crc32::CRC32;

fn main() {
    let header = "kMvcpBVJv6px";
    let data = "oVPx67sRNfmtnDsAqJU7LNNmN1oG3y";

    let crc16 = CRC16::default();
    let crc32 = CRC32::default();

    let checked16 = crc16.calculate(header.as_bytes(), header.len());

    let checked32 = crc32.calculate(data.as_bytes(), data.len());

    println!("CRC16: {checked16:x}");
    println!("CRC32: {checked32:x}");

    let error_header = error_occured(header.as_bytes());
    let error_data = error_occured(data.as_bytes());

    let receiver_check_header = crc16.calculate(&error_header, error_header.len());
    let receiver_check_data = crc32.calculate(&error_data, error_data.len());

    println!("{:x}->{:x}", checked16, receiver_check_header);
    println!("{:x}->{:x}", checked32, receiver_check_data);
}

fn error_occured(data: &[u8]) -> Vec<u8> {
    let len = data.len();
    let mut error = data.to_vec();
    let weak_rand = data[0] as usize;
    error[weak_rand % len] = 0;

    error
}

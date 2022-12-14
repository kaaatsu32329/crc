use crc::crc::CRC;
use crc::crc16::CRC16;
use crc::crc32::CRC32;

fn main() {
    let buf = "123456789";
    let len = buf.len();

    println!("{}", buf);

    let mut crc16 = CRC16::new();
    crc16.make_table();

    let crc16_result = crc16.calculate(buf.as_bytes(), len);

    println!("0x{:x}", crc16_result);

    let mut crc32 = CRC32::new();
    crc32.make_table();

    let crc32_result = crc32.calculate(buf.as_bytes(), len);

    println!("0x{:x}", crc32_result);
}

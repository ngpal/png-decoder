use std::{
    fs::File,
    io::{self, stdin, stdout, Read, Write},
};

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
fn is_png(content: &mut File) -> io::Result<bool> {
    // Check file signature
    let mut signature = [0u8; 8];
    content.read(&mut signature)?;

    Ok(signature == PNG_SIGNATURE)
}

struct CrcManager {
    table: Option<[u32; 256]>,
}

const CRC_POLYNOMIAL: u32 = 0xedb88320;

impl CrcManager {
    fn make_crc_table() -> [u32; 256] {
        let mut table = [0u32; 256];
        for i in 0..256 {
            let mut c = i;
            for _ in 0..8 {
                if c & 1 == 1 {
                    c = CRC_POLYNOMIAL ^ (c >> 1);
                } else {
                    c >>= 1;
                }
            }

            table[i as usize] = c;
        }

        table
    }

    fn update_crc(&mut self, crc: u32, buf: Vec<u8>) -> u32 {
        if self.table.is_none() {
            self.table = Some(Self::make_crc_table());
        }

        let mut c = crc;
        for byte in buf {
            c = self.table.unwrap()[((c ^ byte as u32) & 0xff) as usize] ^ (c >> 8);
        }

        c
    }

    fn crc(&mut self, buf: Vec<u8>) -> u32 {
        self.update_crc(0xFFFFFFFF, buf) ^ 0xFFFFFFFF
    }
}

fn main() -> io::Result<()> {
    print!("Enter file name: ");
    stdout().flush().unwrap();

    let mut filename = String::new();
    stdin().read_line(&mut filename)?;
    filename = filename.trim().to_string();

    let mut file = File::open(filename)?;
    if is_png(&mut file)? {
        println!("This is a PNG file");
    } else {
        println!("This is not a PNG file");
    }

    // Check crc of IHDR chunk
    let mut crc_manager = CrcManager { table: None };

    // Get size of IHDR chunk

    let mut ihdr_size_bytes = [0u8; 4];
    file.read(&mut ihdr_size_bytes)?;
    let ihdr_size = u32::from_be_bytes(ihdr_size_bytes);

    let mut chunk_type_bytes = [0u8; 4];
    file.read(&mut chunk_type_bytes)?;
    let chunk_type = String::from_utf8(chunk_type_bytes.into()).unwrap();

    if chunk_type != "IHDR" {
        panic!("IHDR chunk not fonud");
    } else {
        println!("IHDR chunk found of size {}", ihdr_size);
    }

    // Read data
    let mut data = Vec::new();
    for _ in 0..ihdr_size {
        let mut buf = [0u8];
        file.read(&mut buf)?;
        data.push(buf[0]);
    }

    let mut crc_buf = Vec::from(chunk_type_bytes);
    crc_buf.extend(data);

    let crc = crc_manager.crc(crc_buf);
    println!("CRC calculated is: {:x}", crc);

    let mut file_crc_bytes = [0u8; 4];
    file.read(&mut file_crc_bytes)?;

    println!("CRC found is: {:x}", u32::from_be_bytes(file_crc_bytes));

    Ok(())
}

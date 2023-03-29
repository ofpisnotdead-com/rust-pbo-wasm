use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

// ineffective custom read_until, since it is not available in wasm env
fn read_until_exact(reader: &mut impl Read, delim: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
    let mut total = 0;
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        total += 1;
        if byte[0] == delim {
            buf.push(byte[0]);
            break;
        }
        buf.push(byte[0]);

        // super simple infinte loop break
        if total > 1000 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Filename length too long, is this valid PBO entry?"))
        }
    }
    Ok(total)
}

fn read_stringz(reader: &mut impl Read) -> io::Result<String> {
    let mut buffer = Vec::new();
    read_until_exact(reader, 0u8, &mut buffer)?;
    buffer.pop(); // remove null terminator from the buffer

    match String::from_utf8(buffer) {
        Ok(s) => Ok(s),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

fn read_string(reader: &mut impl Read, length: usize) -> io::Result<String> {
    let mut buffer = vec![0u8; length];
    reader.read_exact(&mut buffer)?;

    match String::from_utf8(buffer) {
        Ok(s) => Ok(s),
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

fn read_u32(reader: &mut impl Read) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    let value = (buffer[0] as u32)
        | ((buffer[1] as u32) << 8)
        | ((buffer[2] as u32) << 16)
        | ((buffer[3] as u32) << 24);
    Ok(value)
}

#[derive(Debug)]
struct PboEntry {
    filename: String,
    packaging_method: String,
    original_size: u32,
    reserved: u32,
    timestamp: u32,
    data_size: u32
}

fn read_pbo_entry(reader: &mut impl Read) -> io::Result<PboEntry> {
    let filename = read_stringz(reader).unwrap();
    let packaging_method = read_string(reader, 4).unwrap();
    let original_size = read_u32(reader)?;
    let reserved = read_u32(reader)?;
    let timestamp = read_u32(reader)?;
    let data_size = read_u32(reader)?;

    let entry = PboEntry {
        filename: filename,
        packaging_method: packaging_method,
        original_size: original_size,
        reserved: reserved,
        timestamp: timestamp,
        data_size: data_size
    };

    Ok(entry)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);

    let mut entries = Vec::new();
    let mut headers = HashMap::new();

    loop {
        let entry = read_pbo_entry(&mut reader).unwrap();
        if entry.filename == "" {
            if entry.packaging_method == "sreV" {
                loop {
                    let key = read_stringz(&mut reader).unwrap();
                    if key == ""  {
                        break;
                    }
                    let value = read_stringz(&mut reader).unwrap();
                    headers.insert(key, value);
                }
            } else {
                break;
            }
        } else {
            entries.push(entry);
        }
    }

    println!("Headers: {:#?}", headers);
    for entry in entries.iter() {
        println!("{:#?}", entry);
    }

    Ok(())
}

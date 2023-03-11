use std::io::{Read};
use std::io::{self};
use wasm_bindgen::prelude::*;
use wasm_bindgen_file_reader::WebSysFile;
use js_sys::Array;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
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

#[wasm_bindgen]
pub fn read_pbo_entries(file: web_sys::File) -> Array {
    let mut wf = WebSysFile::new(file);
    let mut entries = Vec::new();

    loop {
        let entry = read_pbo_entry(&mut wf).unwrap();
        if entry.filename == "" {
            break;
        } else {
            entries.push(entry);
        }
    }

    let arr = Array::new_with_length(entries.len() as u32);
    for (i, entry) in entries.iter().enumerate() {
        arr.set(i as u32, serde_wasm_bindgen::to_value(entry).unwrap())
    }

    arr
}

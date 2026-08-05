use std::{
    fs::{self, File},
    io::{Error, prelude::*},
    path::Path,
};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use log::{error, info};

/// Reads a file as an array of bytes.
pub async fn read_bytes(path: &str) -> Option<Vec<u8>> {
    let p = Path::new(path);

    let mut file = match File::open(p) {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening {path}: {}", e);
            return None;
        }
    };

    let mut file_bytes: Vec<u8> = Vec::new();
    let _size = match file.read_to_end(&mut file_bytes) {
        Ok(s) => s,
        Err(e) => {
            error!("Error reading {path}: {}", e);
            return None;
        }
    };

    Some(file_bytes)
}

/// Reads a compressed map list file.
pub async fn read_gzip<T: prost::Message + Default>(path: &str) -> Option<T> {
    let file_bytes = match read_bytes(path).await {
        Some(bytes) => bytes,
        None => return None,
    };

    let size = file_bytes.len();

    // Decompress file, then write as bytes
    let mut d = GzDecoder::new(&file_bytes[..size]);
    let mut buffer: Vec<u8> = Vec::new();
    let buf_size = d.read_to_end(&mut buffer).unwrap();

    Some(T::decode(&buffer[..buf_size]).unwrap())
}

/// Writes bytes to a file.
pub async fn write_bytes(bytes: Vec<u8>, path: &str) -> Result<usize, Error> {
    let len = bytes.len();

    match fs::write(path, bytes) {
        Ok(_) => info!("Successfully wrote bytes to {}", path),
        Err(e) => {
            error!("{:?}", e);
            return Err(e);
        }
    }

    Ok(len)
}

/// Compresses bytes to GZIP format.
pub async fn compress_bytes_gz(bytes: Vec<u8>) -> Vec<u8> {
    let buf = Vec::new();

    let mut gz = GzEncoder::new(buf, Compression::default());
    let _res = gz.write_all(&bytes);

    gz.finish().unwrap()
}

/// Writes the cache (as bytes) to a compressed Protobuf file.
pub async fn write_cache(map_list_bytes: Vec<u8>, path: &str) -> Result<usize, Error> {
    let compressed = compress_bytes_gz(map_list_bytes).await;

    write_bytes(compressed, path).await
}

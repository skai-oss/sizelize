
use flate2::Compression;
use flate2::write::GzEncoder;

use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use byte_unit::Byte;

pub fn compress(path: &str) -> Byte {
    let mut input = BufReader::new(File::open(path).unwrap());
    let output = Vec::new();
    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    return Byte::from_bytes(u128::try_from(output.len()).unwrap());
}
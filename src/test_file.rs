use byte_unit::Byte;

use crate::compressor::compress;
use crate::messages::print_test_result;

pub fn test (path: &str, max_size: &String) -> bool {
    let max_size_in_bytes = Byte::from_str(max_size).unwrap();
    let result_size = compress(path);
    print_test_result(max_size_in_bytes, result_size, path);
    if result_size > max_size_in_bytes {
        return false;
    } else {
        return true
    };
}

use colored::*;
use byte_unit::Byte;
extern crate exitcode;

pub fn print_test_result( max_size: Byte, compression_size: Byte, path:&str) {

    let adjusted_compression_size = compression_size.get_appropriate_unit(false);
    let adjusted_max_size = max_size.get_appropriate_unit(false);
    if compression_size > max_size {
        println!("{}      {}","[FAIL]".red(), path);
        println!("            {} > {}", adjusted_compression_size.format(2).bold() , adjusted_max_size);
    } else {
        println!("{}   {}","[SUCCESS]".green(), path);
        println!("            {} <= {}", adjusted_compression_size.format(2).bold() , adjusted_max_size);
    }
}

pub fn print_process_result(res: bool) -> () {
    println!("");
    if res == true {
        println!("{}", "[Test successful]".green());
        ::std::process::exit(exitcode::OK);
    } else {
        println!("{}", "[Test failed]".red());
        ::std::process::exit(exitcode::DATAERR);
    }
}
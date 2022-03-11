use crate::scanner::Result;
use byte_unit::Byte;
use colored::*;

fn print_test_result(max_size: Byte, compression_size: Byte, path: &str) {
    let adjusted_compression_size = compression_size.get_appropriate_unit(false);
    let adjusted_max_size = max_size.get_appropriate_unit(false);
    if compression_size > max_size {
        println!("{}      {}", "[FAIL]".red(), path);
        println!(
            "            {} > {}",
            adjusted_compression_size.format(2).bold(),
            adjusted_max_size
        );
    } else {
        println!("{}   {}", "[SUCCESS]".green(), path);
        println!(
            "            {} <= {}",
            adjusted_compression_size.format(2).bold(),
            adjusted_max_size
        );
    }
}

pub fn print_process_result(results: Vec<Result>) -> () {
    let number_of_files = results.len();
    results.iter().for_each(|result| {
        print_test_result(result.max_size, result.compressed_size, &result.path);
    });

    println!("");
    println!("{}", "Summary".underline());
    println!("");

    let failures: Vec<Result> = results
        .into_iter()
        .filter(|result| result.compressed_size > result.max_size)
        .collect();
    let number_of_failures = failures.len();
    if number_of_failures == 0 {
        println!(
            "{} {} {}",
            "All".green(),
            number_of_files.to_string().green(),
            "tests were successful.".green()
        );
        ::std::process::exit(0);
    } else {
        println!(
            "{} {} {} {}",
            number_of_failures.to_string().red(),
            "out of".red(),
            number_of_files.to_string().red(),
            "test failed".red()
        );
        ::std::process::exit(1);
    }
}

use structopt::StructOpt;

use crate::config_reader::read_config;
use crate::messages::print_process_result;
use crate::scanner::scan_files;

mod compressor;
mod config_reader;
mod config_struct;
mod messages;
mod scanner;

const DEFAULT_PACKAGE_JSON: &str = "./package.json";

#[derive(StructOpt, Debug)]
#[structopt(name = "sizelize")]
struct Opt {
    #[structopt(short, long, default_value = DEFAULT_PACKAGE_JSON)]
    path: String,
}

fn main() {
    let opt = Opt::from_args();
    let config = read_config(opt.path).unwrap();
    let results = scan_files(&config);
    print_process_result(results)
}

use structopt::StructOpt;


use crate::config_reader::read_config;
use crate::scanner::scan_files;
use crate::messages::print_process_result;
use crate::test_file::test;


mod config_struct;
mod config_reader;
mod compressor;
mod scanner;
mod messages;
mod test_file;

const DEFAULT_PACKAGE_JSON: &str = "./package.json";

#[derive(StructOpt, Debug)]
#[structopt(name = "sizelize")]
struct Opt {
    #[structopt(short, long, default_value = DEFAULT_PACKAGE_JSON)]
    path: String
}

fn main() {
    let opt = Opt::from_args();
    let config = read_config(opt.path).unwrap();
    let res = scan_files(&config, test);
    print_process_result(res)
}
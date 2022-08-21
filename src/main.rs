use tc_save_monger::get_data_from_file;
use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("you must provide the path to file");
    let circuit = get_data_from_file(path);
    println!("{:?}", circuit.header);
}

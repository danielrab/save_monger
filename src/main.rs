use tc_save_monger::{parse, Circuit};
use std::{env, fs};

fn parse_from_file(path: &str) -> Circuit {
    let bytes = fs::read(path).expect("error reading file");
    parse(&bytes)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("you must provide the path to file");
    let circuit = parse_from_file(path);
    println!("{:?}", circuit.header);
}

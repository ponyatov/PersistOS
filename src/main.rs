#![deny(unsafe_code)]

use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let srcfile = &argv[1];
    let src = fs::read_to_string(srcfile);
    println!("{:?}/{:?} -> {:?}", argc, argv, srcfile);
}

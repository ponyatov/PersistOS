use std::env;
use std::fs;

mod forth;
use crate::forth::config::*;
use crate::forth::memory::*;
use crate::forth::rstack::R;
use crate::forth::stack::D;

// https://github.com/icorderi/rust-pmem/tree/master/sys/pmem-sys

// use crate::pmem;
mod pmem;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let srcfile = &argv[1];
    let _src = fs::read_to_string(srcfile);
    println!("#{:?}: {:?} -> {:?}", argc, argv, srcfile);
    unsafe {
        println!("D/{:?}/ = {:?}", Dsz, D);
        println!("R/{:?}/ = {:?}", Rsz, R);
        println!("M/{:?}/ = {:?}", Msz, M);
    }
    println!("{}", dump(0, Msz));
}

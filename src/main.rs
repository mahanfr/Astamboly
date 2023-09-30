use std::{fs, env::args};

mod elf;
mod instruct_table;

fn main() {
    let mut args = args();
    args.next().unwrap();
    let source = fs::read_to_string(args.next().unwrap()).expect("can not Read the file");
    println!("{}",source.len());
}

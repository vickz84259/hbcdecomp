use std::fs;

use nom::Offset;

mod bytecode_file_format;
mod bytecode_parser;

use bytecode_parser::bytecode_file_parser;

fn main() {
    let bytes_vec = fs::read("target/test.hbc").expect("Unable to read file");
    let bytes = bytes_vec.as_slice();

    let (bytes_remaining, _bytecode_file) = bytecode_file_parser(bytes).unwrap();

    println!("{:?}", bytes.offset(bytes_remaining));
}

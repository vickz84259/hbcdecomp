use std::fs;

use bitfield::bitfield;
use nom::{
    bytes::complete::take,
    combinator::{map, verify},
    multi::count,
    number::complete::{le_u32, le_u64, le_u8},
    sequence::{terminated, tuple},
    IResult,
};

const MAGIC: u64 = 0x1F1903C103BC1FC6;
const SHA1_NUM_BYTES: usize = 20;
const HEADER_PADDING: usize = 31; // bytes

bitfield! {
    struct ByteCodeOptions(u8);
    impl Debug;
    static_builtins, _: 0;
    cjs_modules_statically_resolved, _: 1;
}

#[derive(Debug)]
struct FileHeader<'a> {
    magic: u64,
    version: u32, // Bytecode version number
    source_hash: &'a [u8],
    file_length: u32, // File size in bytes
    global_code_index: u32,
    function_count: u32,        // Number of functions
    string_kind_count: u32,     // Number of string kind entries
    identifier_count: u32,      // Number of strings which are identifiers
    string_count: u32,          // Number of strings in the string table
    overflow_string_count: u32, // Number of strings in overflow table
    string_storage_size: u32,   // Bytes in the blob of string contents
    reg_exp_count: u32,
    reg_exp_storage_size: u32,
    array_buffer_size: u32,
    obj_key_buffer_size: u32,
    obj_value_buffer_size: u32,
    cjs_module_offset: u32, // The starting module ID in this segment
    cjs_module_count: u32,  // Number of modules
    debug_info_offset: u32,
    bytecode_options: ByteCodeOptions,
}

fn magic_parser(input: &[u8]) -> IResult<&[u8], u64> {
    verify(le_u64, |b: &u64| *b == MAGIC)(input)
}

fn hash_parser(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(SHA1_NUM_BYTES)(input)
}

fn entries_parser(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
    // Number of entries in FileHeader after source_hash excluding bytecode_options
    let entries_count = 16;
    count(le_u32, entries_count)(input)
}

fn options_parser(input: &[u8]) -> IResult<&[u8], ByteCodeOptions> {
    map(le_u8, |result: u8| ByteCodeOptions(result))(input)
}

fn padding(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(HEADER_PADDING)(input)
}

fn header(input: &[u8]) -> IResult<&[u8], FileHeader> {
    terminated(
        map(
            tuple((
                magic_parser,
                le_u32,
                hash_parser,
                entries_parser,
                options_parser,
            )),
            |result: (u64, u32, &[u8], Vec<u32>, ByteCodeOptions)| {
                let (magic, version, source_hash, entries, bytecode_options) = result;
                FileHeader {
                    magic,
                    version,
                    source_hash,
                    file_length: entries[0],
                    global_code_index: entries[1],
                    function_count: entries[2],
                    string_kind_count: entries[3],
                    identifier_count: entries[4],
                    string_count: entries[5],
                    overflow_string_count: entries[6],
                    string_storage_size: entries[7],
                    reg_exp_count: entries[8],
                    reg_exp_storage_size: entries[9],
                    array_buffer_size: entries[10],
                    obj_key_buffer_size: entries[11],
                    obj_value_buffer_size: entries[12],
                    cjs_module_offset: entries[13],
                    cjs_module_count: entries[14],
                    debug_info_offset: entries[15],
                    bytecode_options,
                }
            },
        ),
        padding,
    )(input)
}

fn main() {
    let bytes = fs::read("target/test.hbc").expect("Unable to read file");

    let file_header = header(&bytes[..]).unwrap().1;
    println!("{:X?}", file_header);
}

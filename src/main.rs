use std::fs;

use nom::{
    bytes::complete::take,
    combinator::{map, verify},
    multi::count,
    number::complete::{le_u128, le_u32, le_u64, le_u8},
    sequence::{terminated, tuple},
    IResult, Offset,
};

use bytecode_format::{
    ByteCodeOptions, FileHeader, FunctionHeader, StringKind, BYTECODE_ALIGNMENT, MAGIC,
    SHA1_NUM_BYTES,
};

mod bytecode_format;

const HEADER_PADDING: usize = 31; // bytes

trait Align {
    fn align(self, alignment: usize, other: Self) -> Self;
}

impl<'a> Align for &'a [u8] {
    fn align(self, alignment: usize, other: Self) -> Self {
        // Necessary??
        // assert!(alignment > 0 && alignment <= 8 && ((alignment & (alignment - 1)) == 0))

        let bytes_read = self.offset(other);
        match bytes_read % alignment {
            0 => other,
            result => &other[(alignment - result)..],
        }
    }
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

fn get_func_headers_parser<'a>(
    bytes: &'a [u8],
    func_count: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<FunctionHeader>> {
    move |input| {
        let input = bytes.align(BYTECODE_ALIGNMENT, input);
        count(map(le_u128, |result| FunctionHeader(result)), func_count)(input)
    }
}

fn get_string_kinds_parser<'a>(
    bytes: &'a [u8],
    kinds_count: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<StringKind>> {
    move |input| {
        let input = bytes.align(BYTECODE_ALIGNMENT, input);
        count(map(le_u32, |result| StringKind::new(result)), kinds_count)(input)
    }
}

fn main() {
    let bytes_vec = fs::read("target/test.hbc").expect("Unable to read file");
    let bytes = bytes_vec.as_slice();

    let (bytes_remaining, file_header) = header(bytes).unwrap();
    let func_count = file_header.function_count as usize;

    let func_headers_parser = get_func_headers_parser(bytes, func_count);
    let (bytes_remaining, func_headers) = func_headers_parser(bytes_remaining).unwrap();

    let kinds_count = file_header.string_kind_count as usize;
    let string_kinds_parser = get_string_kinds_parser(bytes, kinds_count);
    let string_kinds = string_kinds_parser(bytes_remaining).unwrap().1;

    println!("{:X?}", string_kinds);
}

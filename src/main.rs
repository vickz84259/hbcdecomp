use std::fs;

use nom::{
    bytes::complete::take,
    combinator::{map, verify},
    multi::count,
    number::complete::{le_u128, le_u32, le_u64, le_u8},
    sequence::{terminated, tuple},
    IResult, Offset,
};

use bytecode_file_format::{
    ByteCodeOptions, BytecodeFile, CjsModuleTableEntry, FileHeader, FunctionHeader,
    OverflowStringTableEntry, RegExpTableEntry, SmallStringTableEntry, StringKind,
    BYTECODE_ALIGNMENT, MAGIC, SHA1_NUM_BYTES,
};

mod bytecode_file_format;

type ParserResult<'a, O> = IResult<&'a [u8], O>;

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

fn magic_parser(input: &[u8]) -> ParserResult<u64> {
    verify(le_u64, |b: &u64| *b == MAGIC)(input)
}

fn hash_parser(input: &[u8]) -> ParserResult<&[u8]> {
    take(SHA1_NUM_BYTES)(input)
}

fn entries_parser(input: &[u8]) -> ParserResult<Vec<u32>> {
    // Number of entries in FileHeader after source_hash excluding bytecode_options
    let entries_count = 16;
    count(le_u32, entries_count)(input)
}

fn options_parser(input: &[u8]) -> ParserResult<ByteCodeOptions> {
    map(le_u8, |result: u8| ByteCodeOptions(result))(input)
}

fn padding(input: &[u8]) -> ParserResult<&[u8]> {
    take(HEADER_PADDING)(input)
}

fn file_header(input: &[u8]) -> ParserResult<FileHeader> {
    terminated(
        map(
            tuple((
                magic_parser,
                le_u32,
                hash_parser,
                entries_parser,
                options_parser,
            )),
            |(magic, version, source_hash, entries, bytecode_options)| FileHeader {
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
                regexp_count: entries[8],
                regexp_storage_size: entries[9],
                array_buffer_size: entries[10],
                obj_key_buffer_size: entries[11],
                obj_value_buffer_size: entries[12],
                cjs_module_offset: entries[13],
                cjs_module_count: entries[14],
                debug_info_offset: entries[15],
                bytecode_options,
            },
        ),
        padding,
    )(input)
}

fn multi_count_parser<'a, F, O>(
    bytes: &'a [u8],
    count: u32,
    func: &'a F,
) -> impl Fn(&'a [u8]) -> ParserResult<Vec<O>>
where
    F: Fn(&'a [u8]) -> ParserResult<O>,
{
    move |input| {
        let input = bytes.align(BYTECODE_ALIGNMENT, input);
        nom::multi::count(func, count as usize)(input)
    }
}

fn function_header(input: &[u8]) -> ParserResult<FunctionHeader> {
    map(le_u128, |result| FunctionHeader(result))(input)
}

fn string_kind(input: &[u8]) -> ParserResult<StringKind> {
    map(le_u32, |result| StringKind::new(result))(input)
}

fn string_table_entry(input: &[u8]) -> ParserResult<SmallStringTableEntry> {
    map(le_u32, |result| SmallStringTableEntry(result))(input)
}

fn overflow_table_entry(input: &[u8]) -> ParserResult<OverflowStringTableEntry> {
    map(tuple((le_u32, le_u32)), |(offset, length)| {
        OverflowStringTableEntry { offset, length }
    })(input)
}

fn multi_take_parser<'a>(bytes: &'a [u8], size: u32) -> impl Fn(&'a [u8]) -> ParserResult<&[u8]> {
    move |input| {
        let input = bytes.align(BYTECODE_ALIGNMENT, input);
        take(size)(input)
    }
}

fn regexp_table_entry(input: &[u8]) -> ParserResult<RegExpTableEntry> {
    map(tuple((le_u32, le_u32)), |(offset, length)| {
        RegExpTableEntry { offset, length }
    })(input)
}

fn cjs_module_table_entry(input: &[u8]) -> ParserResult<CjsModuleTableEntry> {
    map(tuple((le_u32, le_u32)), |(first, second)| {
        CjsModuleTableEntry(first, second)
    })(input)
}

fn bytecode_file_parser(input: &[u8]) -> ParserResult<BytecodeFile> {
    let (bytes, header) = file_header(input).unwrap();

    let (
        remaining_bytes,
        (
            function_headers,
            string_kinds,
            identifier_hashes,
            small_string_table,
            overflow_string_table,
            string_storage,
            array_buffer,
            obj_key_buffer,
            obj_value_buffer,
            regexp_table,
            regexp_storage,
            cjs_module_table,
        ),
    ) = tuple((
        multi_count_parser(input, header.function_count, &function_header),
        multi_count_parser(input, header.string_kind_count, &string_kind),
        multi_count_parser(input, header.identifier_count, &le_u32),
        multi_count_parser(input, header.string_count, &string_table_entry),
        multi_count_parser(input, header.overflow_string_count, &overflow_table_entry),
        multi_take_parser(input, header.string_storage_size),
        multi_take_parser(input, header.array_buffer_size),
        multi_take_parser(input, header.obj_key_buffer_size),
        multi_take_parser(input, header.obj_value_buffer_size),
        multi_count_parser(input, header.regexp_count, &regexp_table_entry),
        multi_take_parser(input, header.regexp_storage_size),
        multi_count_parser(input, header.cjs_module_count, &cjs_module_table_entry),
    ))(bytes)
    .unwrap();

    let bytecode_file = BytecodeFile {
        header,
        function_headers,
        string_kinds,
        identifier_hashes,
        small_string_table,
        overflow_string_table,
        string_storage,
        array_buffer,
        obj_key_buffer,
        obj_value_buffer,
        regexp_table,
        regexp_storage,
        cjs_module_table,
    };

    Ok((remaining_bytes, bytecode_file))
}

fn main() {
    let bytes_vec = fs::read("target/test.hbc").expect("Unable to read file");
    let bytes = bytes_vec.as_slice();

    let (bytes_remaining, _bytecode_file) = bytecode_file_parser(bytes).unwrap();

    println!("{:?}", bytes.offset(bytes_remaining));
}

use bitfield::bitfield;

pub const MAGIC: u64 = 0x1F1903C103BC1FC6;
pub const SHA1_NUM_BYTES: usize = 20;

pub const BYTECODE_ALIGNMENT: usize = 4; // bytes

bitfield! {
    pub struct ByteCodeOptions(u8);
    impl Debug;
    pub static_builtins, _: 0;
    pub cjs_modules_statically_resolved, _: 1;
}

#[derive(Debug)]
pub struct FileHeader<'a> {
    pub magic: u64,
    pub version: u32, // Bytecode version number
    pub source_hash: &'a [u8],
    pub file_length: u32, // File size in bytes
    pub global_code_index: u32,
    pub function_count: u32,        // Number of functions
    pub string_kind_count: u32,     // Number of string kind entries
    pub identifier_count: u32,      // Number of strings which are identifiers
    pub string_count: u32,          // Number of strings in the string table
    pub overflow_string_count: u32, // Number of strings in overflow table
    pub string_storage_size: u32,   // Bytes in the blob of string contents
    pub regexp_count: u32,
    pub regexp_storage_size: u32,
    pub array_buffer_size: u32,
    pub obj_key_buffer_size: u32,
    pub obj_value_buffer_size: u32,
    pub cjs_module_offset: u32, // The starting module ID in this segment
    pub cjs_module_count: u32,  // Number of modules
    pub debug_info_offset: u32,
    pub bytecode_options: ByteCodeOptions,
}

#[derive(Debug)]
pub enum Prohibit {
    ProhibitCall,
    ProhibitConstruct,
    ProhibitNone,
}

impl From<u8> for Prohibit {
    fn from(item: u8) -> Self {
        match item {
            0 => Prohibit::ProhibitCall,
            1 => Prohibit::ProhibitConstruct,
            2 => Prohibit::ProhibitNone,
            _ => panic!("This shouldn't happen"),
        }
    }
}

bitfield! {
    pub struct FunctionHeaderFlag(u8);
    impl Debug;
    pub into Prohibit, prohibit_invoke, _: 1, 0;
    pub strict_mode, _: 2;
    pub has_exception_handler, _: 3;
    pub has_debug_info, _: 4;
    pub overflowed, _: 5;
}

impl From<u8> for FunctionHeaderFlag {
    fn from(item: u8) -> Self {
        Self(item)
    }
}

bitfield! {
    pub struct FunctionHeader(u128);
    impl Debug;
    u32;
    pub offset, _: 24, 0; // 25 bits
    pub param_count, _: 31, 25; // 7 bits

    pub bytecode_size_in_bytes, _: 46, 32; // 15 bits
    pub function_name, _: 63, 47; // 17 bits

    pub info_offset, _: 88, 64; // 25 bits
    pub frame_size, _: 95, 89; // 7 bits

    pub u8, environment_size, _: 103, 96; // 8 bits
    pub u8, highest_read_cache_index, _: 111, 104; // 8 bits
    pub u8, highest_write_cache_index, _: 119, 112; // 8 bits
    pub u8, into FunctionHeaderFlag, flags, _: 127, 120; // 8 bits
}

const COUNT_BITS: u32 = 31;
const MAX_COUNT: u32 = (1 << COUNT_BITS) - 1;

const STRING_KIND: u32 = 0 << COUNT_BITS;
const IDENTIFIER_KIND: u32 = 1 << COUNT_BITS;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Kind {
    String = STRING_KIND,
    Identifier = IDENTIFIER_KIND,
}

impl Kind {
    fn new(value: u32) -> Self {
        match value {
            STRING_KIND => Kind::String,
            IDENTIFIER_KIND => Kind::Identifier,
            _ => panic!("Unknown Kind"),
        }
    }
}

#[derive(Debug)]
pub struct StringKind {
    pub kind: Kind,
    pub count: u32,
}

impl StringKind {
    pub fn new(value: u32) -> Self {
        let count = value & MAX_COUNT;
        let kind = Kind::new(value & !MAX_COUNT);

        // assert!((kind as u32 & MAX_COUNT) == 0, "Kind overlapping with count storage.");
        // assert!(1 <= count && count <= MAX_COUNT, "Count out of bounds");

        Self { count, kind }
    }
}

bitfield! {
    pub struct SmallStringTableEntry(u32);
    impl Debug;

    pub is_utf_16, _: 0; // 1 bit
    pub offset, _: 23, 1; // 23 bits
    pub length, _: 31, 24; // 7 bits
}

#[derive(Debug)]
pub struct OverflowStringTableEntry {
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug)]
pub struct RegExpTableEntry {
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug)]
pub struct CjsModuleTableEntry(pub u32, pub u32);

#[derive(Debug)]
pub struct BytecodeFile<'a> {
    pub header: FileHeader<'a>,
    pub function_headers: Vec<FunctionHeader>,
    pub string_kinds: Vec<StringKind>,
    pub identifier_hashes: Vec<u32>,
    pub small_string_table: Vec<SmallStringTableEntry>,
    pub overflow_string_table: Vec<OverflowStringTableEntry>,
    pub string_storage: &'a [u8],
    pub array_buffer: &'a [u8],
    pub obj_key_buffer: &'a [u8],
    pub obj_value_buffer: &'a [u8],
    pub regexp_table: Vec<RegExpTableEntry>,
    pub regexp_storage: &'a [u8],
    pub cjs_module_table: Vec<CjsModuleTableEntry>,
}

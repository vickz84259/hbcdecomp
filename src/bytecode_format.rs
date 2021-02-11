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
    pub reg_exp_count: u32,
    pub reg_exp_storage_size: u32,
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

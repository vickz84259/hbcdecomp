use super::{Register, StringIndex};

#[derive(Debug)]
#[repr(u8)]
pub enum BufferIndex {
    Word(u16),
    Dword(u32),
}

#[derive(Debug)]
pub struct NewObjectExpression {
    no_of_static_elements: u16,
    key_index: BufferIndex,
    value_index: BufferIndex,
    parent: Option<Register>,
}

#[derive(Debug)]
pub struct NewArrayExpression {
    no_of_static_elements: u16,
    array_index: Option<BufferIndex>,
}

#[derive(Debug)]
#[repr(u8)]
pub enum EnvIndex {
    Byte(u8),
    Word(u16),
}

#[derive(Debug)]
pub struct EnvExpression {
    index: EnvIndex,
    value: Option<Register>,
}

#[derive(Debug)]
pub enum ArrayIndex {
    Byte(u8),
    Dword(u32),
}

#[derive(Debug)]
pub enum Property {
    String(StringIndex),
    Index(ArrayIndex),
    Register(Register),
}

#[derive(Debug)]
pub enum ObjectExpKind {
    Delete,
    Get,
    Set {
        value: Register,
        enumerable: bool,
    },
    Define {
        getter: Register,
        setter: Register,
        enumerable: bool,
    },
}

#[derive(Debug)]
pub enum ObjectType {
    Normal,
    Array,
    Global,
    This,
    Environment { id: EnvIndex },
}

pub struct ObjectExpression {
    object: Register,
    obj_type: ObjectType,
    property: Property,
    kind: ObjectExpKind,
}

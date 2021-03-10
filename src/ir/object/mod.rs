mod new_array;
mod new_object;
mod object_expression;

pub use new_array::*;
pub use new_object::*;
pub use object_expression::*;

#[derive(Debug)]
pub enum BufferIndex {
    Word(u16),
    Dword(u32),
}

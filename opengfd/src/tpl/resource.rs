use cpp_types::msvc::string::String as CppString;

#[repr(C)]
#[derive(Debug)]
pub struct Resource {
    this: *mut Resource,
    owner: usize,
    field10: i32,
    field14: i32,
    field18: i32,
    field1c: i32,
    filename: CppString,
    stream: *mut u8,
    stream_owner: usize,
    size: usize,
    field58: usize,
    field60: i32
}

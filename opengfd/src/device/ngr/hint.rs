#[repr(C)]
#[derive(Debug)]
pub struct MemHint {
    _cpp_vtable: usize,
    data: [u8; 8]
}

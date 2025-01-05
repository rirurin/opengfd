#[repr(C)]
#[derive(Debug)]
pub struct MemHint {
    _cpp_vtable: *const u8,
    data: [u8; 8]
}

impl MemHint {
    pub fn new() -> Self {
        Self {
            _cpp_vtable: std::ptr::null(),
            data: [0; 8]
        }
    }
}

use crate::globals;

#[repr(C)]
#[derive(Debug)]
pub struct MemHint {
    _cpp_vtable: *const u8,
    hint: u32,
}

impl MemHint {
    pub fn new() -> Self {
        Self::new_value(0)
    }
    pub fn new_value(hint: u32) -> Self {
        Self {
            _cpp_vtable: match globals::get_ngr_memhint_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            hint
        }
    }
}

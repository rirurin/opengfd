use crate::device::ngr::hint::MemHint;

#[repr(C)]
#[derive(Debug)]
pub struct Allocator {
    _cpp_vtable: usize,
    // Methods:
    // drop
    // malloc
    // malloc2
    // free
    // realloc
    // msize
    // always returns 0
    _platform: usize,
    _hint: MemHint,
    data: [u8; 0x28]
}

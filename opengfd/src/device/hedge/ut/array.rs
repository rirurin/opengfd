use std::ptr::NonNull;

#[repr(C)]
pub struct Array<T, S> {
    buffer: Option<NonNull<T>>,
    length: S,
    capacity: S,
    _allocator: *const u8
}

type IndexType = u32;

#[repr(C)]
pub struct InplaceArray<T, const R: usize> {
    _super: Array<T, IndexType>,
    reserved: [T; R]
}
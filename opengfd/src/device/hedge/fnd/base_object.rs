use std::ptr::NonNull;
use crate::device::hedge::fnd::allocator::IAllocator;

#[repr(C)]
pub struct BaseObject {
    _cpp_vtable: *const u8,
    allocator: NonNull<IAllocator>,
}

#[repr(C)]
pub struct ReferencedObject {
    _super: BaseObject,
    is_ref_counted: bool,
    ref_count: u32
}
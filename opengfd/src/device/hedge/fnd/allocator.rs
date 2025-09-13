use std::ptr::NonNull;
use crate::device::hedge::fnd::{
    mutex::Mutex,
    tlsf_heap::TlsfHeapTemplate
};

#[repr(C)]
pub struct IAllocator(*const u8);

// vtable:
// 0x0: ~IAllocator
// 0x8: malloc(size, align)
// 0x10: free(ptr)

// See https://github.com/angryzor/rangers-api/blob/main/rangers-api/Hedgehog/Base/System/MemoryRouter.h

// 0x14252c3a0 in P5R
#[repr(C)]
pub struct MemoryRouterAllocator {
    _super: IAllocator,
    heap: Option<NonNull<TlsfHeapTemplate<Mutex>>>,
    unk1: usize
}

#[repr(C)]
pub struct MemoryRouter {
    module: NonNull<MemoryRouterAllocator>,
    debug: NonNull<MemoryRouterAllocator>,
}
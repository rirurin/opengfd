use allocator_api2::alloc::{AllocError, Allocator as AllocatorTrait};

use crate::device::ngr::hint::MemHint;
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug)]
pub struct AllocatorPlatformVtable {
    // FUN_1410e2380
    drop: fn(&mut Allocator, usize) -> *mut u8,
    // FUN_1410e2320
    malloc: fn(&Allocator, usize) -> Option<NonNull<u8>>,
    // FUN_1410e2330
    malloc2: fn(&mut Allocator, usize, i32) -> Option<NonNull<u8>>,
    // FUN_1410e2340
    free: fn(&Allocator, NonNull<u8>) -> (),
    // FUN_1410e2350
    realloc: fn(&mut Allocator, NonNull<u8>, usize) -> Option<NonNull<u8>>,
    // FUN_1410e2360
    msize: fn(&mut Allocator, NonNull<u8>) -> usize,
    // FUN_1410e2370
    func6: fn(&mut Allocator) -> usize
}

#[repr(C)]
#[derive(Debug)]
pub struct Allocator {
    _cpp_vtable: usize,
    _platform: &'static AllocatorPlatformVtable,
    _hint: MemHint,
    data: [u8; 0x28]
}

unsafe impl AllocatorTrait for Allocator {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, allocator_api2::alloc::AllocError> { 
        match (self._platform.malloc)(self, layout.size()) {
            Some(n) => Ok(NonNull::slice_from_raw_parts(n, layout.size())),
            None => Err(AllocError)
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: std::alloc::Layout) {
        (self._platform.free)(self, ptr)
    }
}

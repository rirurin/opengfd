use allocator_api2::alloc::{AllocError, Allocator as AllocatorTrait};

use crate::{
    device::ngr::hint::MemHint,
    globals
};
use std::{
    ffi::c_void,
    ptr::NonNull
};

#[repr(C)]
#[derive(Debug)]
pub struct AllocatorPlatformVtable {
    // FUN_1410e2380
    drop: fn(&mut Allocator, usize) -> *mut u8,
    // FUN_1410e2320 (1410ebd40)
    malloc: fn(&Allocator, usize) -> Option<NonNull<u8>>,
    // FUN_1410e2330
    malloc2: fn(&mut Allocator, usize, i32) -> Option<NonNull<u8>>,
    // FUN_1410e2340 (1410ebd60)
    free: fn(&Allocator, NonNull<u8>) -> (),
    // FUN_1410e2350
    realloc: fn(&mut Allocator, NonNull<u8>, usize) -> Option<NonNull<u8>>,
    // FUN_1410e2360
    msize: fn(&mut Allocator, NonNull<u8>) -> usize,
    // FUN_1410e2370
    func6: fn(&mut Allocator) -> usize
}

impl AllocatorPlatformVtable {
    pub unsafe fn malloc(&self, alloc: usize) -> Option<NonNull<u8>> {
        let out = libc::malloc(alloc);
        if out.is_null() { None } else { Some(NonNull::new_unchecked(out as *mut u8))}
    }
    pub unsafe fn malloc2(&self, _alloc: usize, a3: i32) -> Option<NonNull<u8>> {
        self.malloc(a3 as usize)
    }
    pub unsafe fn free(&self, ptr: NonNull<u8>) {
        libc::free(ptr.as_ptr() as *mut c_void)
    }
    pub unsafe fn realloc(&self, ptr: NonNull<u8>, new_alloc: usize) -> Option<NonNull<u8>> {
        let out = libc::realloc(ptr.as_ptr() as *mut c_void, new_alloc);
        if out.is_null() { None } else { Some(NonNull::new_unchecked(out as *mut u8))}
    }
    pub unsafe fn msize(&self, ptr: NonNull<u8>) -> usize {
        libc::_msize(ptr.as_ptr() as *mut c_void)
    }
    pub unsafe fn func6(&self) -> usize { 0 }
}

#[repr(C)]
#[derive(Debug)]
pub struct Allocator {
    _cpp_vtable: *const u8,
    _platform: *const &'static AllocatorPlatformVtable,
    _hint: MemHint,
    data: [u8; 0x28]
}

unsafe impl AllocatorTrait for Allocator {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        match (unsafe { (*self._platform).malloc })(self, layout.size()) {
            Some(n) => Ok(NonNull::slice_from_raw_parts(n, layout.size())),
            None => Err(AllocError)
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: std::alloc::Layout) {
        ((*self._platform).free)(self, ptr)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AllocatorHook;

unsafe impl AllocatorTrait for AllocatorHook {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { globals::get_ngr_allocator_unchecked().allocate(layout) }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: std::alloc::Layout) {
        globals::get_ngr_allocator_unchecked().deallocate(ptr, layout)
    }
}

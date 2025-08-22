use allocator_api2::alloc::{ AllocError, Allocator };
use std::{
    ffi::c_void,
    ptr::NonNull
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GfdAllocator;

unsafe impl Allocator for GfdAllocator {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let out = libc::malloc(layout.size());
            if out.is_null() { 
                Err(AllocError) 
            } else {
                let not_null = NonNull::new(out as *mut u8).unwrap();
                Ok(NonNull::slice_from_raw_parts(not_null, layout.size()))
            }
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: std::alloc::Layout) { 
        libc::free(ptr.as_ptr() as *mut c_void)
    }
}

#![allow(dead_code)]
use std::{
    mem::size_of,
    sync::atomic::Ordering
};

#[cfg(feature = "v1-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CmdBuffer {
    buffer: [*mut u8; 2],
    buffer_ptr: *mut u8,
    buffer_head: *mut u8,
    buffer_tail: *mut u8,
    buffer_size: u32
}

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CmdBuffer {
    buffer: [*mut u8; 3],
    ptr: std::sync::atomic::AtomicUsize
}

unsafe impl Send for CmdBuffer {}
unsafe impl Sync for CmdBuffer {}

impl CmdBuffer {
    /// Allocate memory in the command buffer, returning the previous value. Specified size is in
    /// bytes, and is aligned to 0x10. Note that the memory *is not cleared*, it's the
    /// responsibility of the caller to ensure that the memory represents some valid state!
    /// (Original function: gfdCmdBufferAlloc)
    pub unsafe fn alloc(&mut self, size: i32) -> *mut u8 {
        self.ptr.fetch_add((size as usize) + 0xf & !0xf, Ordering::Relaxed) as *mut u8
    }
    /// Similar to `alloc`, but allocates according to the size of the provided type. Similarly to
    /// `alloc`, the memory returned is uninitialized (I recommend calling this inside of a new()
    /// function, and then clear memory and set initial values there)
    pub unsafe fn alloc_ex<T>(&mut self) -> &mut T {
        &mut *(self.alloc(size_of::<T>() as i32) as *mut T)
    }
    pub unsafe fn alloc_type<T>(&mut self, extra: usize) -> *mut T {
        self.alloc((size_of::<T>() + extra) as i32) as *mut T
    }
}
/*
impl Drop for CmdBuffer {

}
*/
/*
#[allow(non_snake_case)]
pub mod ffi {
    use super::CmdBuffer;
    #[no_mangle]
    pub unsafe extern "C" fn gfdCmdBufferAlloc(pCmdBuffer: *mut CmdBuffer, size: i32) -> usize {
        pCmdBuffer.as_mut().unwrap().alloc(size) as usize
    }
}
*/
#[cfg(test)]
pub mod tests {

}

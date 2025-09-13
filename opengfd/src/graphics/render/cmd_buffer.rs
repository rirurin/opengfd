use crate::kernel::global::RENDER_LISTS;
use std::{
    ffi::c_void,
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

pub trait CmdBufferInterface {
    /// Allocate memory in the command buffer, returning the previous value. Specified size is in
    /// bytes, and is aligned to 0x10. Note that the memory *is not cleared*, it's the
    /// responsibility of the caller to ensure that the memory represents some valid state!
    /// (Original function: gfdCmdBufferAlloc for CmdBuffer, ngrCmdBufferAlloc for
    /// PlatformCmdBuffer)
    unsafe fn alloc(&mut self, size: i32) -> *mut u8;
    /// Similar to `alloc`, but allocates according to the size of the provided type. Similarly to
    /// `alloc`, the memory returned is uninitialized (I recommend calling this inside of a new()
    /// function, and then clear memory and set initial values there)
    unsafe fn alloc_ex<T>(&mut self) -> &mut T {
        &mut *(self.alloc(size_of::<T>() as i32) as *mut T)
    }
    unsafe fn alloc_type<T>(&mut self, extra: usize) -> *mut T {
        self.alloc((size_of::<T>() + extra) as i32) as *mut T
    }
    unsafe fn copy_from_slice<T>(&mut self, data: &[T]) -> &[T] {
        let out = self.alloc((data.len() * size_of::<T>()) as i32);
        std::ptr::copy_nonoverlapping(data.as_ptr(), out as *mut T, data.len());
        std::slice::from_raw_parts(out as *mut T, data.len())
    }
    unsafe fn alloc_zeroed(&mut self, size: i32) -> *mut u8 {
        let out = self.alloc(size);
        libc::memset(out as *mut c_void, 0, size as usize);
        out
    }
    unsafe fn alloc_ex_zeroed<T>(&mut self) -> *mut T {
        let out = self.alloc_ex::<T>();
        libc::memset(&raw mut *out as *mut c_void, 0, size_of::<T>());
        out
    }
    unsafe fn alloc_to_slice<T>(&mut self, count: usize) -> &[T] {
        let alloc = self.alloc((size_of::<T>() * count) as i32) as *mut T;
        std::slice::from_raw_parts(alloc, count)
    }
    unsafe fn alloc_to_slice_mut<T>(&mut self, count: usize) -> &mut [T] {
        let alloc = self.alloc((size_of::<T>() * count) as i32) as *mut T;
        std::slice::from_raw_parts_mut(alloc, count)
    }
}

pub const DEFAULT_CMD_BUFFER_SIZE: usize = 0x2000000; // ( 32 MB )

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CmdBuffer {
    buffer: [*mut u8; RENDER_LISTS],
    ptr: std::sync::atomic::AtomicUsize
}

unsafe impl Send for CmdBuffer {}
unsafe impl Sync for CmdBuffer {}

impl CmdBufferInterface for CmdBuffer {
    // For GFD command buffer: has to be allocated to nearest 0x10!
    unsafe fn alloc(&mut self, size: i32) -> *mut u8 {
        self.ptr.fetch_add((size as usize) + 0xf & !0xf, Ordering::Relaxed) as *mut u8
    }
}
impl CmdBuffer {
    pub fn get_buffer_val(&self, index: usize) -> *mut u8 {
        self.buffer[index]
    }
    pub fn get_ptr_val(&self) -> usize {
        unsafe { *self.ptr.as_ptr() }
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

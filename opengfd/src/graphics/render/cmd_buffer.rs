#![allow(dead_code)]

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
    /// (Original function: gfdCmdBufferAlloc)
    unsafe fn alloc(&mut self, size: i32) -> usize {
        self.ptr.fetch_add((size as usize) + 0xf & !0xf, std::sync::atomic::Ordering::Relaxed)
    }
}
/*
impl Drop for CmdBuffer {

}
*/

#[allow(non_snake_case)]
pub mod ffi {
    use super::CmdBuffer;
    #[no_mangle]
    pub unsafe extern "C" fn gfdCmdBufferAlloc(pCmdBuffer: *mut CmdBuffer, size: i32) -> usize {
        pCmdBuffer.as_mut().unwrap().alloc(size)
    }
}

#[cfg(test)]
pub mod tests {

}

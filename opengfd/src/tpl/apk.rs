#[repr(C)]
pub struct ApkTextureStream {
    free_cb: unsafe extern "C" fn(*mut u8),
    stream: *mut u8
}

impl ApkTextureStream {
    pub fn get_free_cb(&self) -> unsafe extern "C" fn (*mut u8) {
        self.free_cb
    }
    pub fn get_stream(&self) -> *mut u8 {
        self.stream
    }
    
    pub fn new(free_cb: unsafe extern "C" fn(*mut u8), stream: *mut u8) -> Self {
        Self { free_cb, stream }
    }
}
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug)]
pub struct ResBuffer {
    pub ptr: NonNull<u8>,
    pub offset: NonNull<u8>,
    pub size: usize,
    pub owner: Option<NonNull<Resources>>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Resources {
}

#[cfg(feature = "v1-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CullObject {
    todo: [u8; 0x20]
}

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CullObject {
    todo: [u8; 0x10]
}

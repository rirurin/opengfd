// see https://github.com/angryzor/rangers-api/blob/main/rangers-api/Hedgehog/Needle/SurfaceBase.h

#[repr(u32)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SurfaceType {
    NONE,
    UNK1,
    UNK2,
    UNK3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SurfaceFormat {
    UNKNOWN,
    R8G8B8A8,
}

#[repr(C)]
pub struct SurfaceBase {
    
}
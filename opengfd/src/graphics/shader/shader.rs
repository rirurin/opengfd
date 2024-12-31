#[cfg(feature = "v1-core")]
use super::flag::{ Flags0, Flags1, Flags2, Flags3 };

#[cfg(feature = "v2-core")]
use super::flag::{ Flags0, Flags1, Flags2 };

#[cfg(feature = "v1-core")]
#[repr(C)]
pub struct ShaderID {
    pub id: i16,
    pub flags0: Flags0,
    pub flags1: Flags1,
    pub flags2: Flags2,
    pub flags3: Flags3,
    pub texcoord0: i32,
    pub texcoord1: i32,
}

#[cfg(feature = "v2-core")]
#[repr(C)]
pub struct ShaderID {
    pub id: i16,
    pub flags0: Flags0,
    pub flags1: Flags1,
    pub flags2: Flags2,
    pub texcoord0: i32,
    pub texcoord1: i32,
    pub vertex_attrib: i32,
}

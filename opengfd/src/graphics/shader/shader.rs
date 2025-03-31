use crate::utility::reference::Reference;

#[cfg(feature = "v1-core")]
use super::flag::{ Flags0, Flags1, Flags2, Flags3 };

#[cfg(feature = "v2-core")]
use super::flag::{ Flags0, Flags1, Flags2 };

use std::ops::BitOrAssign;

#[cfg(feature = "v1-core")]
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ShaderID {
    id: i16,
    flags: ShaderFlags,
    texcoord0: i32,
    texcoord1: i32,
    vertex_attrib: i32,
}

impl ShaderID {
    pub fn get_texcoord_in(&self) -> i32 {
        self.texcoord0
    }
    pub fn get_texcoord_out(&self) -> i32 {
        self.texcoord1
    }
}

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderFlags {
    flag0: Flags0,
    flag1: Flags1,
    flag2: Flags2
}

impl ShaderFlags {
    pub fn reset_flag0(&mut self, val: Flags0) {
        self.flag0 = val;
    }
    pub fn reset_flag1(&mut self, val: Flags1) {
        self.flag1 = val;
    }
    pub fn reset_flag2(&mut self, val: Flags2) {
        self.flag2 = val;
    }
}

impl BitOrAssign<Flags0> for ShaderFlags {
    fn bitor_assign(&mut self, rhs: Flags0) {
        self.flag0 |= rhs;
    }
}

impl BitOrAssign<Flags1> for ShaderFlags {
    fn bitor_assign(&mut self, rhs: Flags1) {
        self.flag1 |= rhs;
    }
}

impl BitOrAssign<Flags2> for ShaderFlags {
    fn bitor_assign(&mut self, rhs: Flags2) {
        self.flag2 |= rhs;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderSource {
    code: *mut u8,
    size: u32,
    ref_: Reference
}

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use crate::utility::reference::Reference;

#[cfg(feature = "v1-core")]
use super::flag::{ Flags0, Flags1, Flags2, Flags3 };

#[cfg(feature = "v2-core")]
use super::flag::{ Flags0, Flags1, Flags2 };

use std::ops::{BitOrAssign, Deref};
use allocator_api2::alloc::Allocator;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

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

pub(crate) const TEX_BIT_SIZE: usize = 3;
pub(crate) const TEX_UV_ID_MAX: usize = 2;
pub(crate) const TEX_BIT_MAX: usize = (1 << TEX_BIT_SIZE) - 1;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct TexcoordID(u32);

#[derive(Debug)]
pub enum TexcoordError {
    ParseFailed((usize, u32)),
    InvalidTextureId(usize)
}
// pub struct TexcoordError((usize, u32));

impl Error for TexcoordError {}
impl Display for TexcoordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFailed((tex_id, slot)) => write!(f, "Texcoord Error on Texture {}: Slot {}", *tex_id, *slot),
            Self::InvalidTextureId(id) => write!(f, "Tried getting slot for invalid texture ID {}", *id),
        }
    }
}

impl TryFrom<u32> for TexcoordID {
    type Error = TexcoordError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        for i in 0..10 {
            let tex = Self::get_static(value, i);
            if tex > 2 && tex != 7 {
                return Err(TexcoordError::ParseFailed((i, tex)));
            }
        }
        Ok(TexcoordID(value))
    }
}

impl Deref for TexcoordID {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TexcoordID {
    fn get_inner(&self, tex_id: usize) -> u32 {
        (**self as usize >> (tex_id * TEX_BIT_SIZE) & TEX_BIT_MAX) as u32
    }

    fn get_static(value: u32, tex_id: usize) -> u32 {
        (value as usize >> (tex_id * TEX_BIT_SIZE) & TEX_BIT_MAX) as u32
    }

    pub fn get_slot(&self, tex_id: usize) -> Result<u32, TexcoordError> {
        if tex_id > 9 {
            return Err(TexcoordError::InvalidTextureId(tex_id));
        }
        Ok(self.get_inner(tex_id))
    }
}

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ShaderID {
    id: i16,
    flags: ShaderFlags,
    texcoord0: TexcoordID,
    texcoord1: TexcoordID,
    vertex_attrib: u32,
}

impl ShaderID {
    pub fn get_texcoord_in(&self) -> TexcoordID {
        self.texcoord0
    }
    pub fn get_texcoord_out(&self) -> TexcoordID {
        self.texcoord1
    }
    pub fn get_vertex_attribute(&self) -> u32 {
        self.vertex_attrib
    }
    pub fn get_id(&self) -> i16 {
        self.id
    }
    pub fn get_flags(&self) -> &ShaderFlags {
        &self.flags
    }
}

#[cfg(all(feature = "serialize", feature = "v2-core"))]
impl<AStream, T> GfdSerialize<AStream, T> for ShaderID
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ShaderID = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(all(feature = "serialize", feature = "v2-core"))]
impl ShaderID {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self.id = stream.read_u16()? as i16;
        self.texcoord0 = stream.read_u32()?.try_into()?;
        self.texcoord1 = stream.read_u32()?.try_into()?;
        Ok(())
    }
}

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
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
    pub fn get_flag0(&self) -> Flags0 {
        self.flag0
    }
    pub fn get_flag1(&self) -> Flags1 {
        self.flag1
    }
    pub fn get_flag2(&self) -> Flags2 {
        self.flag2
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

impl Default for ShaderFlags {
    fn default() -> Self {
        Self {
            flag0: Flags0::empty(),
            flag1: Flags1::empty(),
            flag2: Flags2::empty(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderSource {
    code: *mut u8,
    size: u32,
    ref_: Reference
}

#![allow(dead_code, unused_variables)]
//! (Original file: gfdName.c)
use allocator_api2::alloc::{ Allocator, Global };
use bitflags::bitflags;
use crc32fast::Hasher;
// use riri_mod_tools_proc::ensure_layout;
use std::{
    alloc::Layout,
    fmt::{ Display, Debug },
    // hash::{ Hash, Hasher },
    mem::align_of,
    ptr::NonNull
};
use std::error::Error;
use std::fmt::Formatter;
use std::io::{Read, Seek, Write};
use glam::Mat4;
use crate::kernel::version::GfdVersion;
#[cfg(feature = "serialize")]
use crate::utility::stream::{DeserializationStack, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NameFlags: u32 {
        const CalculatedCrc = 1 << 0;
        const CalculatedLength = 1 << 1;
    }
}

/// An immutable, UTF-8 string used in GFD to name particular objects, such as nodes or effect 
/// leaves. The target allocator is customizable to allow for better interoperation with
/// precompiled code (e.g Injecting a custom gfdName into Persona 5 Royal to rename a node).
/// Strings up to i32::MAX in length are supported. When using GFD's serializer, only names up to
/// i16::MAX are supported, however larger strings can be stored when using serde.
#[repr(C)]
pub struct Name<A = Global>
where A: Allocator + Clone
{
    /// Stores current state of the name instance
    pub(crate) flags: NameFlags,
    /// Pointer to the string buffer. How characters are stored depends on the target charset
    /// (either UTF-8 or P5 encoding)
    pub string: Option<NonNull<u8>>,
    /// The length of the string. Any value above i32::MAX will panic.
    pub length: i32,
    /// the CRC32 hash for the string
    pub hash: u32,
    pub(crate) _allocator: A,
}

// TODO: Serialization

impl<A> Name<A>
where A: Allocator + Clone
{
    pub fn empty_in(alloc: A) -> Self {
        let flags = NameFlags::empty();
        Self { flags, length: 0, string: None, hash: 0, _allocator: alloc }
    }
    /// (Original function: gfdNameSet)
    pub fn new_in(text: &str, alloc: A) -> Self {
        let flags = NameFlags::CalculatedLength | NameFlags::CalculatedCrc;
        let length: i32 = text.len().try_into().unwrap();
        let mut hasher = Hasher::new_with_initial(!text.len() as u32);
        hasher.update(text.as_bytes());
        let hash = !hasher.finalize(); 
        let string = Some(alloc.allocate(Self::get_layout_sized(length)).unwrap().cast());
        unsafe { text.as_ptr().copy_to_nonoverlapping(string.unwrap().as_ptr(), length as usize); }
        Self {
            flags, length, string, hash,
            _allocator: alloc,
        }
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    /// (Original function: gfdNameGetLength)
    pub fn len(&self) -> i32 { self.length }
    fn get_layout(&self) -> Layout { Self::get_layout_sized(self.len()) }
    fn get_layout_sized(len: i32) -> Layout {
        // SAFETY: Alignment is fixed to alignof(u8), size cannot exceed isize since length is
        // stored as i32 (max sound size on 32-bit)
        // (on serialized gfdName, only i16 is possible!)
        unsafe { Layout::from_size_align_unchecked(len as usize, align_of::<u8>()) }
    }
    pub fn get_string(&self) -> Option<&str> {
        self.string.map(|v| {
            // SAFETY: self.string ptr is valid for lifetime of self
            let slice = unsafe { std::slice::from_raw_parts(v.as_ptr(), self.len() as usize) };
            // SAFETY: self.string is always UTF-8 (except in FFI), return value can only live as long as self
            unsafe { std::str::from_utf8_unchecked(slice) }
        })
    }
    pub fn get_hash(&self) -> u32 { self.hash }
}

impl Name<Global> {
    pub fn new(text: &str) -> Self { Self::new_in(text, Global) }
}

impl<A> Display for Name<A> 
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self.get_string() {
            Some(v) => v,
            None => "NULL"
        };
        write!(f, "{}", fmt)
    }
}

impl<A> Debug for Name<A> 
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self.get_string() {
            Some(v) => v,
            None => "NULL"
        };
        write!(f, "GfdName {{ string: {}, hash: {} }}", fmt, self.hash)
    }
}

impl<A> Clone for Name<A> 
where A: Allocator + Clone
{
    /// (Original function: gfdNameCopy)
    fn clone(&self) -> Self {
        // allocator API creates a byte slice, but we only need the buffer component since we store
        // length elsewhere
        let string = Some(self._allocator.allocate(self.get_layout()).unwrap().cast());
        unsafe { self.string.unwrap().copy_to_nonoverlapping(string.unwrap(), self.len() as usize); }
        Self {
            flags: self.flags,
            string,
            length: self.length,
            hash: self.hash,
            _allocator: self._allocator.clone()
        }
    }
}

impl<A> Drop for Name<A> 
where A: Allocator + Clone
{
    /// (Original function: gfdNameClear)
    fn drop(&mut self) {
        // SAFETY: This is the last time that self.string is referred to
        if let Some(s) = self.string {
            unsafe { self._allocator.deallocate(s, self.get_layout()); }
        }
    }
}

impl<A> PartialEq for Name<A> 
where A: Allocator + Clone
{
    /// (Original function: gfdNameEqual)
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<A> Eq for Name<A> 
where A: Allocator + Clone
{}

impl<A> PartialOrd for Name<A> 
where A: Allocator + Clone
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A> Ord for Name<A> 
where A: Allocator + Clone
{
    /// (Original function: gfdNameCompare)
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hash == other.hash { return std::cmp::Ordering::Equal; }
        let self_slice = unsafe { std::slice::from_raw_parts(self.string.unwrap().as_ptr(), self.len() as usize) };
        let othr_slice = unsafe { std::slice::from_raw_parts(other.string.unwrap().as_ptr(), other.len() as usize) };
        self_slice.cmp(othr_slice)
    }
}

impl From<&str> for Name<Global> {
    fn from(value: &str) -> Self {
        Self::new_in(value, Global)
    }
}

impl<A> PartialEq<str> for Name<A> 
where A: Allocator + Clone
{
    fn eq(&self, other: &str) -> bool {
        match self.get_string() {
            Some(v) => v == other,
            None => false
        }
    }
}

impl<A> PartialOrd<str> for Name<A> 
where A: Allocator + Clone
{
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.get_string().map(|v| v.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum NameError {
    HashMismatch
}

impl Error for NameError {}
impl Display for NameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "serialize")]
pub trait NameSerializationTechnique<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&self, stream: &mut Stream<AStream, T>, alloc: AObject) -> Result<Name<AObject>, Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug;
}

#[cfg(feature = "serialize")]
pub struct NameSerializationContext<A, D>
where A: Allocator + Clone,
      D: NameSerializationTechnique<A>
{
    allocator: A,
    technique: D
}

#[cfg(feature = "serialize")]
impl<A, D> NameSerializationContext<A, D>
where A: Allocator + Clone,
      D: NameSerializationTechnique<A>
{
    pub(crate) fn new(allocator: A, technique: D) -> Self {
        Self { allocator, technique }
    }
}

#[cfg(feature = "serialize")]
impl<A, D> GfdSerializationUserData<A> for NameSerializationContext<A, D>
where A: Allocator + Clone,
      D: NameSerializationTechnique<A> {
    fn get_heap_allocator(&self) -> Option<A> {
        Some(self.allocator.clone())
    }
}

#[cfg(feature = "serialize")]
pub struct NameSerializationNoHash;
impl<AObject> NameSerializationTechnique<AObject> for NameSerializationNoHash
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&self, stream: &mut Stream<AStream, T>, alloc: AObject) -> Result<Name<AObject>, Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        Name::stream_read_string(stream, alloc)
    }
}

#[cfg(feature = "serialize")]
pub struct NameSerializationHash;
impl<AObject> NameSerializationTechnique<AObject> for NameSerializationHash
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&self, stream: &mut Stream<AStream, T>, alloc: AObject) -> Result<Name<AObject>, Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        let name = Name::stream_read_string(stream, alloc)?;
        if stream.has_feature(GfdVersion::NameContainsHash).is_some() {
            let serial_hash = stream.read_u32()?;
            if name.get_hash() != serial_hash {
                return Err(Box::new(NameError::HashMismatch));
            }
        }
        Ok(name)
    }
}

#[cfg(feature = "serialize")]
pub struct NameSerializationHashGAP;
impl<AObject> NameSerializationTechnique<AObject> for NameSerializationHashGAP
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&self, stream: &mut Stream<AStream, T>, alloc: AObject) -> Result<Name<AObject>, Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        let name = Name::stream_read_string(stream, alloc)?;
        if stream.has_feature(GfdVersion::NameContainsHash).is_some() {
            if cfg!(feature = "cfb_gap") {
                let _ = stream.read_u8()?;
            }
            let serial_hash = stream.read_u32()?;
            if name.get_hash() != serial_hash {
                // Used by Catherine: Full Body and Metaphor Refantazio
                return Err(Box::new(NameError::HashMismatch));
            }
        }
        Ok(name)
    }
}

// Called from stream_read
#[cfg(feature = "serialize")]
impl<AObject> Name<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_string<T, AStream>(stream: &mut Stream<AStream, T>, alloc: AObject) -> Result<Self, Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug {
        let mut name = Vec::with_capacity( stream.read_u16()? as usize);
        let slice = unsafe { std::slice::from_raw_parts_mut(name.as_mut_ptr(), name.capacity()) };
        stream.read_u8_slice(slice)?;
        Ok(Self::new_in(unsafe { std::str::from_utf8_unchecked(slice) }, alloc))
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, D, T> GfdSerialize<AStream, T, AObject, DeserializationStack<Self>, NameSerializationContext<AObject, D>> for Name<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone,
      D: NameSerializationTechnique<AObject>
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut NameSerializationContext<AObject, D>) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        Ok(param.technique.stream_read_inner(stream, param.get_heap_allocator().unwrap())?.into())
    }
}

#[allow(non_snake_case)]
pub mod ffi {
    // use allocator_api2::alloc::Allocator;
    use std::ffi::{ c_char, CStr };
    /// P5 2014 EBOOT: 0x878fa4
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameClear(pName: *mut super::Name) {
        let _ = Box::from_raw(pName);
    }
    /// P5 2014 EBOOT: 0x878f0c
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameCompare(p0: *const super::Name, p1: *const super::Name) -> i32 {
        p0.as_ref().unwrap().cmp(p1.as_ref().unwrap()) as i32
    }
    /// P5 2014 EBOOT: 0x878ee0
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameCompareString(p0: *const super::Name, p1: *const c_char) -> i32 {
        p0.as_ref().unwrap().partial_cmp(CStr::from_ptr(p1).to_str().unwrap()).unwrap() as i32
    }
    /*
    /// P5 2014 EBOOT: 0x87948c
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameCopy(pDst: *mut super::Name, pSrc: *const super::Name) -> i32 {
        let dst = pDst.as_mut().unwrap();
        let src = pSrc.as_ref().unwrap();
        if let Some(n) = dst.string {
            // unsafe { dst._allocator.deallocate(dst.string.unwrap(), dst.get_layout()); }
        }
    }
    */
    /// P5 2014 EBOOT: 0x878ffc
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameEqual(p0: *const super::Name, p1: *const super::Name) -> bool {
        let r0 = p0.as_ref().unwrap();
        let r1 = p1.as_ref().unwrap();
        if r0.hash != r1.hash { return false; }
        r0.cmp(r1) == std::cmp::Ordering::Equal
    }
    /// P5 2014 EBOOT: 0x878e6c
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameEqualStringHash(p0: *const super::Name, p1: *const c_char, hash: u32) -> bool {
        let r0 = p0.as_ref().unwrap();
        if let None = r0.string { return false; }
        let cstr = CStr::from_ptr(p1).to_str().unwrap();
        if r0.hash == hash || !r0.flags.contains(super::NameFlags::CalculatedCrc) {
            r0.partial_cmp(cstr) == Some(std::cmp::Ordering::Equal)
        } else {
            false
        }
    }
    /// P5 2014 EBOOT: 0x878f3c
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameGetLength(pName: *const super::Name) -> i32 {
        let name = pName.as_ref().unwrap();
        if name.flags.contains(super::NameFlags::CalculatedLength) {
            name.len()
        } else {
            CStr::from_ptr(name.string.unwrap().as_ptr() as *const c_char).to_bytes().len().try_into().unwrap()
        }
    }
    /// P5 2014 EBOOT: 0x8791ac
    #[no_mangle]
    pub unsafe extern "C" fn gfdNameSet(pName: *mut std::mem::MaybeUninit<super::Name>, pStr: *const c_char) {
        let cstr = CStr::from_ptr(pStr).to_str().unwrap();
        // (*pName).
        // *pName = super::Name::new_in(cstr, allocator_api2::alloc::Global);
        // pName.as_ref().
    }
}

#[cfg(test)]
pub mod tests {
    use super::Name;
    use std::error::Error;
    type TestReturn = Result<(), Box<dyn Error>>;
    #[test]
    fn create_gfd_name_from_string_slice() -> TestReturn {
        // RootNode (48E1B0E5) [48E1B0E5]
        assert_eq!(Name::new("RootNode").get_hash(), 0x48E1B0E5);
        // COMMON/init/gfdDefaultEnv.dds (BB4AC992) [BB4AC992]
        assert_eq!(Name::new("COMMON/init/gfdDefaultEnv.dds").get_hash(), 0xBB4AC992);
        Ok(())
    }
}

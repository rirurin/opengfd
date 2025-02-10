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
    pub fn get_string(&self) -> &str {
        // SAFETY: self.string ptr is valid for lifetime of self
        let slice = unsafe { std::slice::from_raw_parts(self.string.unwrap().as_ptr(), self.len() as usize) };
        // SAFETY: self.string is always UTF-8 (except in FFI), return value can only live as long as self
        unsafe { std::str::from_utf8_unchecked(slice) }
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
        write!(f, "{}", self.get_string())
    }
}

impl<A> Debug for Name<A> 
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GfdName {{ string: {}, hash: {} }}", self.get_string(), self.hash)
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
        unsafe { self._allocator.deallocate(self.string.unwrap(), self.get_layout()); }
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
        self.get_string() == other
    }
}

impl<A> PartialOrd<str> for Name<A> 
where A: Allocator + Clone
{
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        Some(self.get_string().cmp(other))
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

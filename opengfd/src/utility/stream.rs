//! Original file: gfdStream.c

#![allow(dead_code, unused_imports)]
use allocator_api2::{
    alloc::{Allocator, Global},
    boxed::Box as ABox,
    vec::Vec as AVec
};
use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt, NativeEndian};
use half::f16;
use std::{
    alloc::Layout,
    error::Error,
    fmt::{ Debug, Display },
    marker::PhantomData,
    mem::size_of,
    io::{ self, Read },
    // mem::ManuallyDrop,
    ptr::NonNull
};
use std::fmt::Formatter;
use std::io::{Seek, SeekFrom, Write};
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use rkyv::rancor::OptionExt;
use crate::kernel::{
    allocator::GfdAllocator,
    version::GfdVersion
};
use crate::kernel::chip::ChipAllocator;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AccessType: u16 {
        const Read = 1 << 0;
        const Write = 1 << 1;
        const Append = 1 << 2;
        const Create = 1 << 3;
    }
}

#[cfg(target_endian = "little")]
pub type ReverseEndian = BigEndian;
#[cfg(target_endian = "big")]
pub type ReverseEndian = LittleEndian;

type SizeType = u32;

const GFS0_MAGIC: u32 = 0x47465330; // GFS0
const GFS0_MAGIC_REVERSE: u32 = 0x30534647; // 0SFG

pub unsafe trait DeserializationStrategy<T, A, P>
where T: Sized,
      A: Allocator + Clone,
      P: GfdSerializationUserData<A> {
    fn uninit(param: &mut P) -> Self;
    fn zeroed(param: &mut P) -> Self;
}
#[repr(transparent)]
pub struct DeserializationStack<T>(T) where T: Sized;
impl<T> Deref for DeserializationStack<T> where T: Sized {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<T, A, P> DeserializationStrategy<T, A, P> for DeserializationStack<T>
where T: Sized,
      A: Allocator + Clone,
      P: GfdSerializationUserData<A>
{
    fn uninit(_: &mut P) -> Self {
        Self(unsafe { MaybeUninit::uninit().assume_init() })
    }
    fn zeroed(_: &mut P) -> Self {
        Self(unsafe { MaybeUninit::zeroed().assume_init() })
    }
}

impl<T> From<T> for DeserializationStack<T> where T: Sized {
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T> DeserializationStack<T> where T: Sized {
    pub fn into_raw(self) -> T {
        self.0
    }
}

#[repr(C)]
pub struct DeserializationHeap<T, A>
where T: Sized, A: Allocator + Clone{
    instance: ABox<T, A>,
    allocator: A
}

impl<T, A> Deref for DeserializationHeap<T, A>
where T: Sized,
      A: Allocator + Clone {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.instance.as_ref()
    }
}

impl<T, A> DerefMut for DeserializationHeap<T, A>
where T: Sized,
      A: Allocator + Clone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.instance.as_mut()
    }
}

unsafe impl<T, A, P> DeserializationStrategy<T, A, P> for DeserializationHeap<T, A>
where T: Sized,
      A: Allocator + Clone,
      P: GfdSerializationUserData<A>
{
    fn uninit(param: &mut P) -> Self {
        let allocator = param.get_heap_allocator().unwrap();
        Self {
            instance: unsafe { ABox::new_uninit_in(allocator.clone()).assume_init() },
            allocator
        }
    }

    fn zeroed(param: &mut P) -> Self {
        let allocator = param.get_heap_allocator().unwrap();
        Self {
            instance: unsafe { ABox::new_zeroed_in(allocator.clone()).assume_init() },
            allocator
        }
    }
}

impl<T, A> DeserializationHeap<T, A>
where T: Sized,
      A: Allocator + Clone
{
    pub fn into_raw(self) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(ABox::into_raw(self.instance)) }
    }
}

pub trait GfdSerializationUserData<A>
where A: Allocator + Clone {
    fn get_heap_allocator(&self) -> Option<A> where A: Allocator + Clone;
}

impl<A> GfdSerializationUserData<A> for ()
where A: Allocator + Clone {
    fn get_heap_allocator(&self) -> Option<A> where A: Allocator + Clone {
        None
    }
}

pub struct SerializationSingleAllocator<A>(A) where A: Allocator + Clone;
impl<A> SerializationSingleAllocator<A> where A: Allocator + Clone {
    pub fn new(alloc: A) -> Self {
        Self(alloc)
    }
}
impl<A> GfdSerializationUserData<A> for SerializationSingleAllocator<A>
where A: Allocator + Clone {
    fn get_heap_allocator(&self) -> Option<A> {
        Some(self.0.clone())
    }
}

// GFD Serialization
pub trait GfdSerialize<AStream, T, AObject = GfdAllocator, D = DeserializationStack<Self>, P = ()>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone,
      D: DeserializationStrategy<Self, AObject, P>,
      P: GfdSerializationUserData<AObject>,
      Self: Sized
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut P) -> Result<D, Box<dyn Error>>;
    fn stream_write(&self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamError {
    Unknown = 0,
    InvalidGfsSignature,
    InvalidPlatform,
    InvalidChunkId,
    ReadNotAllowed,
    WriteNotAllowed,
    BufferOverflow,
    InvalidEnumValue
}

impl Error for StreamError {}
impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GFD Stream Error: {:?}", *self)
    }
}


#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamType {
    FileDescriptor = 1,
    Filename = 2,
    MemoryStream = 3,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum DevicePlatform {
    P5_PC = 1,
    P5_PS3 = 2,
    PDANCE_VITA = 4,
    P5R_PS4 = 6,
    P5R_DX11 = 7,
    METAPHOR = 8 // (and P5R Switch)
}

impl TryFrom<u32> for DevicePlatform {
    type Error = StreamError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::P5_PC),
            2 => Ok(Self::P5_PS3),
            4 => Ok(Self::PDANCE_VITA),
            6 => Ok(Self::P5R_PS4),
            7 => Ok(Self::P5R_DX11),
            8 => Ok(Self::METAPHOR),
            _ => Err(StreamError::InvalidPlatform)
        }
    }
}

#[repr(C)]
// #[derive(Debug, Clone)]
#[derive(Clone)]
pub struct StreamHeader {
    magic: u32,
    version: u32,
    platform: DevicePlatform,
    #[allow(dead_code)]
    _reserve: u32
}

impl StreamHeader {
    pub(crate) fn is_endian_reversed(&self) -> bool {
        self.magic == GFS0_MAGIC_REVERSE
    }
    pub(crate) fn is_endian_reversed_static(magic: u32) -> bool {
        magic == GFS0_MAGIC_REVERSE
    }
}

impl Default for StreamHeader {
    fn default() -> Self {
        Self {
            magic: 0,
            version: GfdVersion::current() as u32,
            platform: DevicePlatform::METAPHOR,
            _reserve: 0
        }
    }
}

impl Debug for StreamHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StreamHeader {{ version: 0x{:x}, platform: {:?} }}", self.version, self.platform)
    }
}

impl StreamHeader {
    pub fn read<R>(reader: &mut R) -> Result<Self, Box<dyn Error>>
    where R: Read + Seek {
        let magic = reader.read_u32::<NativeEndian>()?;
        if magic != GFS0_MAGIC && magic != GFS0_MAGIC_REVERSE {
            return Err(Box::new(StreamError::InvalidGfsSignature));
        }
        let endian_reversed = Self::is_endian_reversed_static(magic);
        let version = match endian_reversed {
            false => reader.read_u32::<NativeEndian>()?,
            true => reader.read_u32::<ReverseEndian>()?
        };
        let platform = (match endian_reversed {
            false => reader.read_u32::<NativeEndian>()?,
            true => reader.read_u32::<ReverseEndian>()?
        }).try_into()?;
        // Skip reserve field
        reader.seek(SeekFrom::Current(size_of::<u32>() as i64))?;
        Ok(Self {
            magic,
            version,
            platform,
            _reserve: 0
        })
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ChunkType {
    EOF,
    Model              = 0x00010003,
    ExtraProperties    = 0x000100F8,
    PhysicsDictionary  = 0x000100F9,
    MaterialDictionary = 0x000100FB,
    TextureDictionary  = 0x000100FC,
    AnimationPack      = 0x000100FD,
    ChunkType000100FE  = 0x000100FE
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ChunkError {
    UnknownChunkType(u32)
}
impl Error for ChunkError {}
impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownChunkType(c) => write!(f, "Unknown chunk type 0x{:x}", *c),
            _ => write!(f, "ChunkError: {:?}", self)
        }
    }
}

impl TryFrom<u32> for ChunkType {
    type Error = ChunkError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EOF),
            0x00010003 => Ok(Self::Model),
            0x000100F8 => Ok(Self::ExtraProperties),
            0x000100F9 => Ok(Self::PhysicsDictionary),
            0x000100FB => Ok(Self::MaterialDictionary),
            0x000100FC => Ok(Self::TextureDictionary),
            0x000100FD => Ok(Self::AnimationPack),
            0x000100FE => Ok(Self::ChunkType000100FE),
            v => Err(ChunkError::UnknownChunkType(v))
        }
    }
}

#[repr(C)]
// #[derive(Debug, Clone)]
#[derive(Clone)]
pub struct ChunkHeader {
    version: u32,
    chunk_id: ChunkType,
    length: u32,
    #[allow(dead_code)]
    _reserve: u32
}

impl Default for ChunkHeader {
    fn default() -> Self {
        Self {
            version: GfdVersion::current() as u32,
            chunk_id: ChunkType::EOF,
            length: 0,
            _reserve: 0
        }
    }
}

impl ChunkHeader {
    pub fn get_version(&self) -> u32 {
        self.version
    }
    pub fn get_chunk_id(&self) -> ChunkType {
        self.chunk_id
    }
    pub fn get_length(&self) -> u32 {
        self.length
    }
}

impl<A, T> GfdSerialize<A, T> for ChunkHeader
where T: Debug + Read + Write + Seek + StreamIODevice,
      A: Allocator + Clone + Debug
{
    fn stream_read(stream: &mut Stream<A, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let version = stream.read_u32()?;
        let chunk_id = stream.read_u32()?.try_into()?;
        let length = stream.read_u32()?;
        stream.seek(SeekFrom::Current(size_of::<u32>() as i64))?;
        Ok(Self { version, chunk_id, length, _reserve: 0 }.into())
    }
}

impl Debug for ChunkHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkHeader {{ version: 0x{:x}, chunk_id: {:?}, length: 0x{:x} }}", self.version, self.chunk_id, self.length)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Stream<A = GfdAllocator, T = StreamMemory<A>>
where T: Debug + Read + Write + Seek,
      // E: ByteOrder,
      A: Allocator + Clone + Debug
{
    header: StreamHeader,
    stream_type: StreamType,
    access_type: AccessType,
    device: T,
    _allocator: A,
    // _endian: PhantomData<E>
}

#[derive(Debug, Clone, Copy)]
pub struct StreamFactory<A: Allocator + Clone + Debug>(A);
impl<A> StreamFactory<A>
where A: Allocator + Clone + Debug {

    pub fn new(allocator: A) -> Self {
        Self(allocator)
    }

    pub fn read_from_memory(&self, buf: &[u8]) -> Result<Stream<A, StreamMemory<A>>, Box<dyn Error>> {
        let mut device = StreamMemory::new_reallocate(buf, self.0.clone());
        let header = StreamHeader::read(&mut device)?;
        Ok(Stream {
            header,
            stream_type: StreamType::MemoryStream,
            access_type: AccessType::Write,
            device,
            _allocator: self.0.clone()
        })
    }
}

/*
impl StreamFactory<Global> {
    // Convenience function for setting up stream from std::fs::read()
    pub fn read_from_global_vec(&self, buf: Vec<u8>) -> Stream<Global, StreamMemory<Global>> {

    }
}
*/

impl<A, T> Stream<A, T>
where T: Debug + Read + Write + Seek + StreamIODevice,
      A: Allocator + Clone + Debug
{

    pub fn get_header_version(&self) -> u32 {
        self.header.version
    }

    pub fn has_feature(&self, feature: GfdVersion) -> Option<()> {
        match self.get_header_version() >= feature as u32 {
            true => Some(()),
            false => None
        }
    }

    /*
    pub fn get_allocator(&self) -> A {
        self._allocator.clone()
    }
    */

    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.device.read_u8()
    }

    pub fn read_u8_slice(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let _ = self.device.read(buf)?;
        Ok(())
    }

    pub fn read_u8_owned(&mut self, len: usize) -> io::Result<AVec<u8, A>> {
        let mut out = AVec::with_capacity_in(len, self._allocator.clone());
        unsafe { out.set_len(out.capacity()) };
        self.read_u8_slice(&mut out)?;
        Ok(out)
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u16::<NativeEndian>()?,
            true => self.device.read_u16::<ReverseEndian>()?,
        })
    }

    pub fn read_u16_slice(&mut self, buf: &mut [u16]) -> io::Result<()> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u16_into::<NativeEndian>(buf)?,
            true => self.device.read_u16_into::<ReverseEndian>(buf)?,
        })
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u32::<NativeEndian>()?,
            true => self.device.read_u32::<ReverseEndian>()?,
        })
    }

    pub fn read_u32_slice(&mut self, buf: &mut [u32]) -> io::Result<()> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u32_into::<NativeEndian>(buf)?,
            true => self.device.read_u32_into::<ReverseEndian>(buf)?,
        })
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u64::<NativeEndian>()?,
            true => self.device.read_u64::<ReverseEndian>()?,
        })
    }

    pub fn read_u64_slice(&mut self, buf: &mut [u64]) -> io::Result<()> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u64_into::<NativeEndian>(buf)?,
            true => self.device.read_u64_into::<ReverseEndian>(buf)?,
        })
    }

    pub fn read_f32(&mut self) -> io::Result<f32> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_f32::<NativeEndian>()?,
            true => self.device.read_f32::<ReverseEndian>()?,
        })
    }

    pub fn read_f32_slice(&mut self, buf: &mut [f32]) -> io::Result<()> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_f32_into::<NativeEndian>(buf)?,
            true => self.device.read_f32_into::<ReverseEndian>(buf)?,
        })
    }

    pub fn read_f64(&mut self) -> io::Result<f64> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_f64::<NativeEndian>()?,
            true => self.device.read_f64::<ReverseEndian>()?,
        })
    }

    pub fn read_f64_slice(&mut self, buf: &mut [f64]) -> io::Result<()> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_f64_into::<NativeEndian>(buf)?,
            true => self.device.read_f64_into::<ReverseEndian>(buf)?,
        })
    }
}

impl<A, T> Seek for Stream<A, T>
where T: Debug + Read + Write + Seek + StreamIODevice,
      A: Allocator + Clone + Debug
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.device.seek(pos)
    }
}

/*
impl<A, T> StreamIORead for Stream<A, T>
where T: Debug + Read + Write + Seek + StreamIODevice,
      A: Allocator + Clone + Debug
{
    fn read_u32(&mut self) -> io::Result<u32> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u32::<NativeEndian>()?,
            true => self.device.read_u32::<ReverseEndian>()?,
        })
    }

    fn read_u32_slice(&mut self, buf: &mut [u32]) -> io::Result<()> {
        Ok(match self.header.is_endian_reversed() {
            false => self.device.read_u32_into::<NativeEndian>(buf)?,
            true => self.device.read_u32_into::<ReverseEndian>(buf)?,
        })
    }
}
*/

/*
pub trait StreamIORead {
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_u32_slice(&mut self, buf: &mut [f32]) -> io::Result<()>;
    /*
    fn read_float_slice(&mut self, buf: &mut [f32]);
    fn read_int16(&mut self) -> i16;
    fn read_int16_slice(&mut self, buf: &mut [i16]);
    fn read_int32(&mut self) -> i32;
    fn read_int32_slice(&mut self, buf: &mut [i32]);
    fn read_int8(&mut self) -> i8;
    fn read_uint16(&mut self) -> u16;
    fn read_uint32(&mut self) -> u32;
    fn read_uint8(&mut self) -> u8;

    // OpenGFD Extensions
    fn read_half_float(&mut self) -> f16;
    fn read_double(&mut self) -> f64;
    */
}

pub trait StreamIOWrite {

}

// GFD Serialization
pub trait StreamSerializable<T: StreamIORead> {
    fn stream_read(stream: &mut T) -> Self;
}

pub trait StreamDeserializable<T: StreamIOWrite> {
    fn stream_write(&self, stream: &mut T) -> SizeType;
}
*/

pub trait StreamIODevice {
    fn make_buffer_slice(&mut self, len: SizeType) -> Option<&[u8]>;
}

#[repr(C)]
#[derive(Debug)]
pub struct StreamMemory<A = GfdAllocator>
where A: Allocator + Clone {
    position: SizeType,
    size: SizeType,
    buffer: Option<NonNull<u8>>,
    _allocator: A
}

impl<A> StreamMemory<A>
where A: Allocator + Clone {

    fn get_layout(len: usize) -> Layout {
        unsafe { Layout::from_size_align_unchecked(len, size_of::<usize>()) }
    }

    pub(crate) fn new_reallocate(buf: &[u8], allocator: A) -> Self {
        match buf.len() {
            0 => Self::new_blank(allocator),
            i => {
                let buffer = allocator.allocate(Self::get_layout(i)).unwrap().cast();
                unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), buffer.as_ptr(), buf.len()) };
                Self {
                    position: 0,
                    size: i as SizeType,
                    buffer: Some(buffer),
                    _allocator: allocator
                }
            }
        }
    }

    pub(crate) fn new_transfer_ownership(buf: AVec<u8, A>) -> Self {
        match buf.len() {
            0 => Self::new_blank(buf.allocator().clone()),
            _ => {
                let (buffer, _, size, _allocator) = buf.into_raw_parts_with_alloc();
                let buffer = NonNull::new(buffer);
                Self {
                    position: 0,
                    size: size as SizeType,
                    buffer,
                    _allocator
                }
            }
        }
    }

    pub(crate) fn new_blank(allocator: A) -> Self {
        Self {
            position: 0,
            size: 0,
            buffer: None,
            _allocator: allocator
        }
    }
}

impl<A> StreamIODevice for StreamMemory<A>
where A: Allocator + Clone {
    fn make_buffer_slice(&mut self, len: SizeType) -> Option<&[u8]> {
        self.buffer.and_then(|buf| {
            if self.size - self.position < len {
                None
            } else {
                let value = unsafe { std::slice::from_raw_parts(buf.as_ptr().add(self.position as usize), len as usize) };
                self.position += len;
                Some(value)
            }
        })
    }
}

impl<A> Drop for StreamMemory<A>
where A: Allocator + Clone {
    fn drop(&mut self) {
        if let Some(buf) = self.buffer {
            let layout = Self::get_layout(self.size as usize);
            unsafe { self._allocator.deallocate(buf, layout) };
        }
    }
}

impl<A> Read for StreamMemory<A>
where A: Allocator + Clone {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.make_buffer_slice(buf.len() as SizeType) {
            Some(s) => {
                buf.copy_from_slice(s);
                Ok(buf.len())
            },
            None => Err(io::Error::from(io::ErrorKind::UnexpectedEof))
        }
    }
}

impl<A> Seek for StreamMemory<A>
where A: Allocator + Clone {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let target = match pos {
            SeekFrom::Start(p) => {
                if p as u32 > self.size {
                    return Err(io::Error::from(io::ErrorKind::InvalidInput));
                }
                p as i64
            },
            SeekFrom::Current(p) => {
                let (v, o) = (self.position as i64).overflowing_add(p);
                if o || v as u32 > self.size {
                    return Err(io::Error::from(io::ErrorKind::InvalidInput));
                }
                v
            },
            SeekFrom::End(p) => {
                let (v, o) = (self.size as i64).overflowing_sub(p);
                if o {
                    return Err(io::Error::from(io::ErrorKind::InvalidInput));
                }
                v
            }
        };
        if target > 0 {
            self.position = target as u32;
        } else {
            self.position = -target as u32;
        }
        Ok(self.position as u64)
    }
}

impl<A> StreamMemory<A>
where A: Allocator + Clone
{
    fn resize_to(&mut self, new_size: usize) -> io::Result<()> {
        if new_size > SizeType::MAX as usize {
            return Err(io::Error::from(io::ErrorKind::FileTooLarge));
        }
        let new = self._allocator.allocate(Self::get_layout(new_size)).unwrap().cast();
        if let Some(b) = self.buffer {
            unsafe {
                std::ptr::copy_nonoverlapping(b.as_ptr(), new.as_ptr(), self.size as usize);
                self._allocator.deallocate(b, Self::get_layout(self.size as usize));
            }
        }
        self.buffer = Some(new);
        Ok(())
    }
    fn resize_auto(&mut self) -> io::Result<()> {
        self.resize_to(self.buffer.map_or(0x100, |_| self.size as usize * 2))
    }
}

impl<A> Write for StreamMemory<A>
where A: Allocator + Clone
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Reallocate
        if self.position as usize + buf.len() > self.size as usize {
            self.resize_auto()?;
        }
        unsafe { std::ptr::copy(buf.as_ptr(), self.buffer.unwrap().as_ptr().add(self.position as usize), buf.len()) };
        self.position += buf.len() as SizeType;
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct StreamFile<A = GfdAllocator>
where A: Allocator + Clone {
    fd: i32, // Unix file descriptor (see https://doc.rust-lang.org/std/os/fd/struct.OwnedFd.html)
    _padding: [u8; 0xc], // make Stream<StreamFile> the same size as Stream<StreamMemory>
    _allocator: PhantomData<A>
}

pub mod ffi {

}
#[cfg(test)]
pub mod tests {

}

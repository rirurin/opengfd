//! Original file: gfdStream.c

#![allow(dead_code, unused_imports)]
use bitflags::bitflags;
use byteorder::{ ByteOrder, LittleEndian, BigEndian };
use half::f16;
use std::{
    error::Error,
    fmt::Display,
    mem::size_of,
    io::{ self, Read },
    // mem::ManuallyDrop,
    ptr::NonNull
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StreamType: u16 {
        const Read = 1 << 0;
        const Write = 1 << 1;
    }
}

type SizeType = u32;

const GFS0_MAGIC: u32 = 0x47465330; // GFS0
const GFS0_MAGIC_REVERSE: u32 = 0x30534647; // 0SFG

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessType {
    FileDescriptor = 2,
    MemoryStream = 3,
    Rkyv = 4
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StreamHeader {
    magic: u32,
    version: u32,
    platform: u32,
    #[allow(dead_code)]
    _reserve: u32
}

impl StreamHeader {
    pub(crate) fn check_reverse_endian(&self) -> bool {
        self.magic == GFS0_MAGIC_REVERSE
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ChunkHeader {
    version: u32,
    chunk_id: u32,
    length: u32,
    #[allow(dead_code)]
    _reserve: u32
}

// #[cfg(target_endian = "little")]

#[repr(C)]
// #[derive(Debug, Clone)]
pub struct Stream<
    #[cfg(feature = "v1-core")] // P5R Default
    E: ByteOrder = BigEndian, 
    #[cfg(not(feature = "v1-core"))]
    E: ByteOrder = LittleEndian, 
    T: StreamIORead<E> + StreamIOWrite<E> = StreamMemory<E>
>
    // where E: ByteOrder
{
    header: StreamHeader,
    stream_type: StreamType,
    access_type: AccessType,
    position: SizeType,
    _endian: E,
    device: T
    // device: StreamState 
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamErrorReason {
    Unknown = 0,
    ReadNotAllowed,
    WriteNotAllowed,
    BufferOverflow
}

#[derive(Debug)]
pub struct StreamError(StreamErrorReason);
impl Error for StreamError {}
impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GFD Stream Error: {:?}", self.0)
    }
}

impl Stream {
    /* 
    fn read_bytes(&mut self, len: SizeType) -> Result<&[u8], StreamError> {
        if self.stream_type.contains(StreamType::Read) {
            Ok(self.device.read_bytes(len))
        } else {
            Err(StreamError(StreamErrorReason::ReadNotAllowed))
        }
    }
    */
}

pub trait StreamIORead<E: ByteOrder> {
    // Use read() method in Read instead
    // fn read_bytes(&mut self, len: SizeType) -> &[u8];
    // fn read_chunk_header(&mut self) -> &ChunkHeader;
    fn read_float(&mut self) -> f32;
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
}
pub trait StreamIOWrite<E: ByteOrder> {

}

// GFD Serialization
pub trait StreamSerializable<E, T>
    where E: ByteOrder,
    T: StreamIORead<E> 
{
    fn stream_read(stream: &mut T) -> Self;
}

pub trait StreamDeserializable<E, T>
    where E: ByteOrder,
    T: StreamIOWrite<E> 
{
    fn stream_write(&self, stream: &mut T) -> SizeType;
}

#[repr(C)]
#[derive(Debug)]
pub struct StreamMemory<
    #[cfg(feature = "v1-core")] // P5R Default
    E: ByteOrder = BigEndian,
    #[cfg(not(feature = "v1-core"))]
    E: ByteOrder = LittleEndian,
> {
    position: SizeType,
    size: SizeType,
    buffer: NonNull<u8>,
    _endian: E
}

impl<E: ByteOrder> StreamMemory<E> {
    fn make_buffer_slice(&mut self, len: SizeType) -> &[u8] {
        self.position += len;
        unsafe { std::slice::from_raw_parts(self.buffer.as_ptr().add((self.position - len) as usize), len as usize) }
    }
}

impl<E: ByteOrder> StreamIORead<E> for StreamMemory<E> {
    /*
    fn read_bytes(&mut self, len: SizeType) -> &[u8] {
        self.make_buffer_slice(len)
    }
    */
    fn read_float(&mut self) -> f32 {
        E::read_f32(self.make_buffer_slice(size_of::<f32>() as SizeType))
    }

    fn read_float_slice(&mut self, buf: &mut [f32]) {
        E::read_f32_into(self.make_buffer_slice((buf.len() * size_of::<f32>()) as SizeType), buf)
    }
    
    fn read_int16(&mut self) -> i16 {
        E::read_i16(self.make_buffer_slice(size_of::<i16>() as SizeType))
    }

    fn read_int16_slice(&mut self, buf: &mut [i16]) {
        E::read_i16_into(self.make_buffer_slice((buf.len() * size_of::<i16>()) as SizeType), buf)
    }

    fn read_int32(&mut self) -> i32 {
        E::read_i32(self.make_buffer_slice(size_of::<i32>() as SizeType))
    }

    fn read_int32_slice(&mut self, buf: &mut [i32]) {
        E::read_i32_into(self.make_buffer_slice((buf.len() * size_of::<i32>()) as SizeType), buf)
    }

    fn read_int8(&mut self) -> i8 {
        self.read_uint8() as i8
    }

    fn read_uint16(&mut self) -> u16 {
        E::read_u16(self.make_buffer_slice(size_of::<u16>() as SizeType))
    }

    fn read_uint32(&mut self) -> u32 {
        E::read_u32(self.make_buffer_slice(size_of::<u32>() as SizeType))
    }

    fn read_uint8(&mut self) -> u8 {
        let res = unsafe { *self.buffer.as_ptr().add(self.position as usize) };
        self.position += 1;
        res
    }

    fn read_half_float(&mut self) -> f16 {
        unsafe { std::mem::transmute::<u16, f16>(E::read_u16(self.make_buffer_slice(size_of::<f16>() as SizeType))) }
    }
    fn read_double(&mut self) -> f64 {
        E::read_f64(self.make_buffer_slice(size_of::<f64>() as SizeType))
    }
}

impl<E: ByteOrder> StreamIOWrite<E> for StreamMemory<E> {

}

impl<E: ByteOrder> Read for StreamMemory<E> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() > SizeType::MAX as usize {
            return Err(io::Error::from(io::ErrorKind::FileTooLarge));
        }
        let slice = self.make_buffer_slice(buf.len() as SizeType);
        buf.copy_from_slice(slice);
        Ok(buf.len())
    }
}

/*
// TODO
#[repr(C)]
#[derive(Debug)]
pub struct StreamFile<
    #[cfg(feature = "v1-core")] // P5R Default
    E: ByteOrder = BigEndian,
    #[cfg(not(feature = "v1-core"))]
    E: ByteOrder = LittleEndian,
> {
    fd: i32, // Unix file descriptor (see https://doc.rust-lang.org/std/os/fd/struct.OwnedFd.html)
    _padding: [u8; 0xc], // make Stream<StreamFile> the same size as Stream<StreamMemory>
    _endian: E
}
impl<E: ByteOrder> StreamIO<E> for StreamFile<E> {
    fn read_bytes(&mut self, len: SizeType) -> &[u8] {
        // std::slice::
        // Box::new()
        // libc::read(self.fd, )
    }
}
*/

#[allow(nonstandard_style)]
pub enum GfdVersion {
    NameContainsHash = 0x1080001,
    MaterialBlendAddMultiplyMode = 0x1080004,
    EnvFogMode = 0x1102000,
    GeometryAddGeomType = 0x1103021,
    MaterialBlendAsU8 = 0x1103041, // also adds mat constant field
    MaterialFlagsAllowConstantColor = 0x1104000,
    EnvAddHeightFog = 0x1104021,
    EnvColorCorrectionScreenBurn = 0x1104141,
    CameraAddRoll = 0x1104061,
    LightAddFlags = 0x1104191,
    EnvAddIndependenceLight = 0x1104241,
    EnvLightingStarParameters = 0x1104301,
    MaterialAddSecondFlags = 0x1104801,
    // xrd757_p4d
    GameRelease_P4D = 0x1105030,
    EplAddP5RField80 = 0x1105061,
    // xrd664
    GameRelease_Persona5 = 0x1105070,
    // xrd757
    GameRelease_P5D_P3D = 0x1105090,
    // xrd744
    GameRelease_Persona5Royal = 0x1105100, // also CFB
    MaterialUseParameterSet = 0x2000000,
    MaterialParameter0AddMultiAlpha = 0x2000004,
    EnvColorCorrectionScreenHSL = 0x2000005,
    MaterialParameterToonSetP12 = 0x2010000,
    EnvAddTemperareSection = 0x2020001,
    MaterialParameterAddBloomIntensity = 0x2030001,
    EnvAddCloudsSection = 0x2060000,
    MaterialParameterToonAddSpecularThreshold = 0x2090001,
    EnvColorCorrectionFieldModelSpecific = 0x2092001,
    MaterialParameterToonAddEdgeRemoveYAxisFactor = 0x2094001,
    EnvLightMetaphorField0 = 0x2098001,
    EnvAddSSAOSection = 0x2099001,
    EnvAddToneMapSection = 0x2102001,
    EnvToneMapAddFilmAlpha = 0x2103001,
    EnvFieldLightMetaphorField1 = 0x2104001,
    MaterialParameterToonAddP17 = 0x2109501,
    MaterialParameterToonAddP20 = 0x2109601,
    EnvFogColorMultiplyFactor = 0x2101001,
    EnvCloudsAddCloudColor = 0x2110011,
    MaterialParameter0AddFlags = 0x2110041,
    CameraAddUnkMetaphor = 0x2110051,
    EnvFogColorParameterSky = 0x2110111,
    EnvFogHeightColorSky = 0x2110120,
    MaterialAddField6C = 0x2110161,
    EnvAddLUTRecolorWeighting = 0x2110173,
    EnvHeightFogSkyStartEnd = 0x2110174,
    EnvFogColorParameterToon = 0x2110175, // env height color toon, fog dist color toon
    MaterialParameter3AddBloomIntensity = 0x2110184,
    EnvAddEnvironmentColorsSection = 0x2110185,
    MaterialParameterWaterAddFlags = 0x2110188, // env fog parameters sky
    LightAddMetaphorField98 = 0x2110191,
    EnvFogDistanceColorSky = 0x2110194,
    MaterialParameterToonAddMatRoughness = 0x2110198,
    EnvLightingAdaptedLumAdjust = 0x2110201,
    EnvLightingPbrIntensity = 0x2110202,
    LightAddMetaphorField9C = 0x2110203,
    MaterialParameterToonAddFittingTile = 0x2110204, // add P4_7
    EnvAddInfiniteOcean_LUTRecolorParams = 0x2110205,
    GeometryAddStrideType = 0x2110206,
    EplAddMetaphorField60 = 0x2110208,
    MaterialParameterToonAddMultiFittingTile = 0x2110210,
    GeometryAddMetaphorUnkVertexWeightRelated = 0x2110213,
    MaterialParameter3AddP8 = 0x2110218,
    EnvFogExponentialHeightYRate = 0x2110219,
    // xrd759
    GameRelease_MetaphorRefantazio = 0x02110221,
    // OpenGFD Extensions
    OpenGFD_MaterialDefineWaterColor = 0x2110222
}

pub mod ffi {

}
#[cfg(test)]
pub mod tests {

}

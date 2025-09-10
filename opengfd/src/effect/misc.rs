use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use allocator_api2::alloc::Allocator;
use crate::kernel::version::GfdVersion;
use crate::utility::stream::{DeserializationStack, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Adjustment {
    scale: f32,
    speed: f32
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for Adjustment
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Adjustment = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl Adjustment {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        // PARAMS
        self.scale = stream.read_f32()?;
        self.speed = stream.read_f32()?;
        Ok(())
    }
}

impl Adjustment {
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Self {
            scale: 1.,
            speed: 0.
        }
    }
}
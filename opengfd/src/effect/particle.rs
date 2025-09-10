use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::{
    effect::{
        behavior::Behavior,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::resources::ResBuffer,
    utility::misc::RGBA
};
use crate::effect::behavior::{BasicParams, LineParams, RingParams, SmokeParams, SparkParams, SphereParams, SpiralParams};
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParticleEmitterType {
    Smoke = 1,
    Explosion,
    Spiral,
    Ball,
    Circle,
    StraightLine
}

impl TryFrom<u32> for ParticleEmitterType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Smoke),
            2 => Ok(Self::Explosion),
            3 => Ok(Self::Spiral),
            4 => Ok(Self::Ball),
            5 => Ok(Self::Circle),
            6 => Ok(Self::StraightLine),
            v => Err(EplError::InvalidParticleEmitterType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Particle<A = GfdAllocator>
where A: Allocator + Clone {
    emitter_type: ParticleEmitterType,
    number: i32,
    color: RGBA,
    behavior: Option<NonNull<Behavior<A>>>,
    material: Option<NonNull<EPLMaterial<A>>>,
    vertex_buffer: *mut u8,
    command: [Option<NonNull<ResBuffer>>; 2],
    job_data: *mut u8,
    params: NonNull<EPLParameter<BasicParams, A>>,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Particle<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.color = RGBA::from_rgba_u32(u32::MAX);
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Particle<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.emitter_type = stream.read_u32()?.try_into()?;
        if stream.has_feature(GfdVersion::EplBehaviorBlurParams).is_none() {
            stream.seek(SeekFrom::Current(size_of::<u32>() as i64))?;
        }
        self.params = unsafe { NonNull::new_unchecked(match self.emitter_type {
            ParticleEmitterType::Smoke => EPLParameter::<SmokeParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Explosion => EPLParameter::<SparkParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Spiral => EPLParameter::<SpiralParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Ball => EPLParameter::<SphereParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Circle => EPLParameter::<RingParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::StraightLine => EPLParameter::<LineParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
        })};
        self.number = self.get_params().get_max_generate_count().clamp(1, 500);
        Ok(())
    }

    pub fn get_params(&self) -> &EPLParameter<BasicParams, AObject> {
        unsafe { self.params.as_ref() }
    }

    pub fn get_params_mut(&mut self) -> &mut EPLParameter<BasicParams, AObject> {
        unsafe { self.params.as_mut() }
    }
}
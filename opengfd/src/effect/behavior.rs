use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    effect::{
        misc::Adjustment,
        particle::ParticleEmitterType
    },
    graphics::curve::Curve4,
    utility::misc::{ Fade, Range }
};
use glam::{ Vec2, Vec3, Mat4 };
use crate::graphics::curve::Curve2;
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

#[repr(C)]
#[derive(Debug)]
pub struct Behavior<A = GfdAllocator>
where A: Allocator + Clone {
    type_: ParticleEmitterType,
    number: u32,
    num_all: u32,
    generated: u32,
    time: f32,
    refresh: f32,
    scale: f32,
    seed: u32,
    base: u32,
    dirty: u32,
    posture: Option<NonNull<EPLPosture>>,
    physics: Option<NonNull<u8>>,
    params: Option<NonNull<u8>>,
    initial: Option<NonNull<u8>>,
    job_data: Option<NonNull<u8>>,
    _allocator: A
}

#[repr(C)]
#[derive(Debug)]
pub struct EPLPosture {
    pos: Vec3,
    time: f32
}

// <gfdsharp> ParticleEmitter

#[repr(C, packed(4))]
// #[derive(Debug)]
pub struct EmitterParams {
    transform: Mat4,
    life: f32, // despawn_timer
    count: Range, // spawn_choker
    field4c: Range,
    field54: Range
}

impl Debug for EmitterParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmitterParams {{ life: {}, count: {:?}, field4c: {:?}, field54: {:?} }}", self.life, &self.count, &self.field4c, &self.field54)
    }
}

impl Clone for EmitterParams {
    fn clone(&self) -> Self {
        Self {
            transform: unsafe { std::ptr::read_unaligned(&raw const self.transform) },
            life: self.life,
            count: self.count.clone(),
            field4c: self.field4c.clone(),
            field54: self.field54.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ColorParams {
    curve4: Curve4, // color_over_life
    alpha: f32,
    fade: Fade,
    random: f32,
    blend: u32 // draw_queue_id
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScaleParams {
    curve4: Curve4, // size_over_life
    rate: Vec2, // field12c/field130
    field140: Range
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RotateParams {
    start: Range, // spawner_angles
    speed: Range,
    accele: f32,
    rotate_type: u32 // field148
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BlurParams {
    length: u32, // field14c
    interval: f32 // field150
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for BlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: BlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl BlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.length = stream.read_u32()?;
        self.interval = stream.read_f32()?;
        Ok(())
    }
}

impl Default for BlurParams {
    fn default() -> Self {
        Self {
            length: 0,
            interval: 0.
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BasicParamFlags : u32 {
        const Flag0 = 1 << 0;
        const Flag1 = 1 << 1;
        const Flag2 = 1 << 2;
        const Flag3 = 1 << 3;
        const Flag4 = 1 << 4;
        const Flag5 = 1 << 5;
        const Flag6 = 1 << 6;
        const Flag7 = 1 << 7;
        const Flag8 = 1 << 8;
        const Flag9 = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const Flag15 = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const Flag18 = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BasicParams {
    field0: u32, // everything in disassembly is - 0x10, this is handled in the outer function
    _align: [u8; 0xc],
    emitter: EmitterParams,
    color: ColorParams,
    scale: ScaleParams,
    rotate: RotateParams,
    blur: BlurParams,
    adjust: Adjustment,
    flags: BasicParamFlags, // random_spawn_delay
    life: f32, // particle_life
    seed: u32 // angle_seed
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for BasicParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: BasicParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl BasicParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        // PARAMS
        self.field0 = stream
            .has_feature(GfdVersion::MaterialExtensionEdgeBacklight)
            .map_or::<Result<u32, Box<dyn Error>>, _>(
                Ok(1),
                |_| Ok(stream.read_u32()?)
            )?;
        self.flags = BasicParamFlags::from_bits_retain(stream.read_u32()?);
        self.life = stream.read_f32()?;
        self.seed = stream.read_u32()?;
        // EMITTER
        self.emitter.life = stream.read_f32()?;
        self.emitter.count = Range::stream_read(stream, &mut ())?.into_raw();
        self.emitter.field4c = stream
            .has_feature(GfdVersion::EplBehaviorGetField4C)
            .map_or::<Result<Range, Box<dyn Error>>, _>(
                Ok(Range::new(1., 0.)),
                |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
            )?;
        self.emitter.field54 = stream
            .has_feature(GfdVersion::EnvBehaviorSetField54)
            .map_or::<Result<Range, Box<dyn Error>>, _>(
                Ok(Range::new(1., 0.)),
                |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
            )?;
        // COLOR
        self.color.alpha = stream.read_f32()?;
        self.color.blend = stream.read_u32()?;
        self.color.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.color.curve4 = match stream.has_feature(GfdVersion::EplBehaviorUseCurve4ForLife) {
            Some(_) => Ok::<Curve4, Box<dyn Error>>(Curve4::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Curve2::stream_read(stream, &mut ())?.into_raw().into())
        }?;
        self.color.random = stream
            .has_feature(GfdVersion::EplBehaviorRandomColor)
            .map_or::<Result<f32, Box<dyn Error>>, _>(
                Ok(0.),
                |_| Ok(stream.read_f32()?)
            )?;
        // SCALE
        self.scale.curve4 = match stream.has_feature(GfdVersion::EplBehaviorUseCurve4ForLife) {
            Some(_) => Ok::<Curve4, Box<dyn Error>>(Curve4::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Curve2::stream_read(stream, &mut ())?.into_raw().into())
        }?;
        self.scale.rate = stream
            .has_feature(GfdVersion::EplBehaviorRandomColor)
            .map_or::<Result<Vec2, Box<dyn Error>>, _>(
                Ok(Vec2::new(1., 1.)),
                |_| Ok(Vec2::stream_read(stream, &mut ())?.into_raw())
            )?;
        self.scale.field140 = stream
            .has_feature(GfdVersion::EplBehaviorGetField4C)
            .map_or::<Result<Range, Box<dyn Error>>, _>(
                Ok(Range::new(1., 0.)),
                |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
            )?;
        // ROTATE
        match stream.has_feature(GfdVersion::EplBehaviorGetField4C) {
            Some(_) => {
                self.rotate.start = Range::stream_read(stream, &mut ())?.into_raw();
                self.rotate.speed = Range::stream_read(stream, &mut ())?.into_raw();
                self.rotate.accele = stream.read_f32()?;
            },
            None => {
                let value = Curve2::stream_read(stream, &mut ())?.into_raw();
                self.rotate.start = Range::new(value.get_start::<f32>()?, value.get_end::<f32>()?);
                self.rotate.speed = Range::new(0.5, 0.1);
                self.rotate.accele = 0.001;
            }
        }
        self.rotate.rotate_type = stream.read_u32()?;
        // BLUR
        self.blur = stream
            .has_feature(GfdVersion::EplBehaviorBlurParams)
            .map_or::<Result<BlurParams, Box<dyn Error>>, _>(
                Ok(BlurParams::default()),
                |_| Ok(BlurParams::stream_read(stream, &mut ())?.into_raw())
            )?;
        // ADJUSTMENT
        self.adjust = stream
            .has_feature(GfdVersion::EplBehaviorAdjustmentParams)
            .map_or::<Result<Adjustment, Box<dyn Error>>, _>(
                Ok(Adjustment::default()),
                |_| Ok(Adjustment::stream_read(stream, &mut ())?.into_raw())
            )?;
        Ok(())
    }

    // Original function: gfdEPLBehaviorGetMaxGenerateCount
    pub fn get_max_generate_count(&self) -> i32 {
        match self.emitter.life {
            0. => self.life as i32,
            v => ((self.emitter.count.get_datums() + self.emitter.count.get_range()) * v * 60.) as i32
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SmokeParams {
    basic: BasicParams,
    radius: f32, // disperse_dist
    speed: Range, // launch_up_speed, launch_up_speed_rand
    gravity: Range, // gravity_strength, gravity_strength_rand
    amplif_begin: Range, // disperse_start_dist, disperse_start_dist_rand
    amplif_end: Range, // disperse_end_dist, disperse_end_dist_rand
    amplif_speed: Range, // disperse_rotate, disperse_rotate_rand
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SmokeParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SmokeParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SmokeParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = stream.read_f32()?;
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.gravity = match stream.has_feature(GfdVersion::EplBehaviorSmokeGravity) {
            Some(_) => Ok::<Range, Box<dyn Error>>(Range::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Range::new(stream.read_f32()?, 0.))
        }?;
        self.amplif_begin = Range::stream_read(stream, &mut ())?.into_raw();
        self.amplif_end = Range::stream_read(stream, &mut ())?.into_raw();
        self.amplif_speed = Range::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Range,
    spread_xz: Range,
    spread_y: Range,
    speed: Range,
    gravity: Range,
    accele: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SparkParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SparkParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SparkParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        match stream.has_feature(GfdVersion::EplBehaviorSmokeGravity) {
            Some(_) => {
                self.spread_xz = Range::stream_read(stream, &mut ())?.into_raw();
                self.spread_y = Range::stream_read(stream, &mut ())?.into_raw();
                self.gravity = Range::stream_read(stream, &mut ())?.into_raw();
            },
            None => {
                self.spread_xz = Range::new(-1., 2.);
                self.spread_y = Range::new(-1., 2.);
                self.gravity = Range::new(stream.read_f32()?, 0.);
            }
        };
        self.accele = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SpiralParams {
    basic: BasicParams,
    height: f32, // spiral_height
    radius_begin: Range, // spiral_start_radius
    radius_end: Range, // spiral_end_radius
    rotate_speed: Range, // soft_spiral_rotation
    rotate_accele: f32, // hard_spiral_rotation
    rise_speed: Range, // slide_up_speed
    gravity: Range // slide_down_speed
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SpiralParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SpiralParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SpiralParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.height = stream.read_f32()?;
        self.radius_begin = Range::stream_read(stream, &mut ())?.into_raw();
        self.radius_end = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rise_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.gravity = match stream.has_feature(GfdVersion::EplBehaviorSmokeGravity) {
            Some(_) => Ok::<Range, Box<dyn Error>>(Range::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Range::new(stream.read_f32()?, 0.))
        }?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SphereParams {
    basic: BasicParams,
    radius_begin: Range,
    radius_end: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    gravity: Range,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SphereParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SphereParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SphereParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius_begin = Range::stream_read(stream, &mut ())?.into_raw();
        self.radius_end = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.gravity = match stream.has_feature(GfdVersion::EplBehaviorSmokeGravity) {
            Some(_) => Ok::<Range, Box<dyn Error>>(Range::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Range::new(stream.read_f32()?, 0.))
        }?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RingParams {
    basic: BasicParams,
    radius: Range,
    height: Range,
    spread_speed: Range,
    spread_accele: f32,
    rotate_speed: Range,
    rotate_accele: f32,
    gravity: Range,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for RingParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: RingParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl RingParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = Range::stream_read(stream, &mut ())?.into_raw();
        self.spread_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.spread_accele = stream.read_f32()?;
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        match stream.has_feature(GfdVersion::EplBehaviorSmokeGravity) {
            Some(_) => {
                self.height = Range::stream_read(stream, &mut ())?.into_raw();
                self.gravity = Range::stream_read(stream, &mut ())?.into_raw();
            },
            None => {
                self.height = Range::new(0., 0.);
                self.gravity = Range::new(stream.read_f32()?, 0.);
            }
        };
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LineParams {
    basic: BasicParams,
    length: f32,
    speed: Range,
    gravity: Range,
    ampli_begin: Range,
    ampli_end: Range,
    ampli_speed: Range,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for LineParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: LineParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl LineParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.length = stream.read_f32()?;
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.gravity = Range::stream_read(stream, &mut ())?.into_raw();
        self.ampli_begin = Range::stream_read(stream, &mut ())?.into_raw();
        self.ampli_end = Range::stream_read(stream, &mut ())?.into_raw();
        self.ampli_speed = Range::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}
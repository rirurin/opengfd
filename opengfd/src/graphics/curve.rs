use std::error::Error;
use glam::Vec2;
use std::fmt::{ Debug, Formatter };
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::misc::RGBA;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurveType {
    #[default]
    Integer = 0,
    Float = 1,
    Color = 2,
    Vec2 = 3
}

impl TryFrom<u16> for CurveType {
    type Error = EplError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Integer),
            1 => Ok(Self::Float),
            2 => Ok(Self::Color),
            3 => Ok(Self::Vec2),
            v => Err(EplError::InvalidCurveType(v))
        }
    }
}

#[repr(C)]
pub(crate) union CurveTarget {
    integer: i32,
    float: f32,
    color: RGBA,
    range: Vec2
}

impl Default for CurveTarget {
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl Clone for CurveTarget {
    fn clone(&self) -> Self {
        unsafe { std::ptr::read(self) }
    }
}

/// Two-point bezier curve, with a start and end point. Mostly used for particle systems.
#[repr(C)]
#[derive(Default, Clone)]
pub struct Curve2 {
    curve_type: CurveType,
    start: CurveTarget,
    end: CurveTarget,
    point: [Vec2; 2],
    table: [u16; 23]
}

#[cfg(feature = "serialize")]
impl<A, T> GfdSerialize<A, T> for Curve2
where T: Debug + Read + Write + Seek + StreamIODevice,
      A: Allocator + Clone + Debug
{
    fn stream_read(stream: &mut Stream<A, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this = Curve2::default();
        this.curve_type = stream.read_u16()?.try_into()?;
        match this.curve_type {
            CurveType::Integer => {
                this.start.integer = stream.read_u32()? as i32;
                this.end.integer = stream.read_u32()? as i32;
            },
            CurveType::Float => {
                this.start.float = stream.read_f32()?;
                this.end.float = stream.read_f32()?;
            },
            CurveType::Color => {
                this.start.color = RGBA::stream_read(stream, &mut ())?.into_raw();
                this.end.color = RGBA::stream_read(stream, &mut ())?.into_raw();
            },
            CurveType::Vec2 => {
                this.start.range = Vec2::stream_read(stream, &mut ())?.into_raw();
                this.end.range = Vec2::stream_read(stream, &mut ())?.into_raw();
            },
        }
        for i in 0..2 {
            this.point[i] = Vec2::stream_read(stream, &mut ())?.into_raw();
        }
        stream.read_u16_slice(this.table.as_mut_slice())?;
        Ok(this.into())
    }
}

impl Debug for Curve2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Curve2 {{ type: {:?}, points: ({}, {}) }}", self.curve_type, self.point[0], self.point[1])
    }
}

pub trait GetCurveValue where Self: Sized {
    const CURVE_TYPE: CurveType;
    fn get_value(data: &CurveTarget) -> Self;
    fn set_value(data: &mut CurveTarget, value: Self);
}

impl GetCurveValue for i32 {
    const CURVE_TYPE: CurveType = CurveType::Integer;
    fn get_value(data: &CurveTarget) -> Self {
        unsafe { data.integer }
    }
    fn set_value(data: &mut CurveTarget, value: Self) {
        data.integer = value;
    }
}

impl GetCurveValue for f32 {
    const CURVE_TYPE: CurveType = CurveType::Float;
    fn get_value(data: &CurveTarget) -> Self {
        unsafe { data.float }
    }
    fn set_value(data: &mut CurveTarget, value: Self) {
        data.float = value;
    }
}

impl GetCurveValue for RGBA {
    const CURVE_TYPE: CurveType = CurveType::Color;
    fn get_value(data: &CurveTarget) -> Self {
        unsafe { data.color }
    }
    fn set_value(data: &mut CurveTarget, value: Self) {
        data.color = value;
    }
}

impl GetCurveValue for Vec2 {
    const CURVE_TYPE: CurveType = CurveType::Vec2;
    fn get_value(data: &CurveTarget) -> Self {
        unsafe { data.range }
    }
    fn set_value(data: &mut CurveTarget, value: Self) {
        data.range = value;
    }
}

impl Curve2 {
    pub fn get_start<T>(&self) -> Result<T, EplError> where T: GetCurveValue {
        match self.curve_type == T::CURVE_TYPE {
            true => Ok(T::get_value(&self.start)),
            false => Err(EplError::IncorrectCurveType((T::CURVE_TYPE, self.curve_type)))
        }

    }
    pub fn get_end<T>(&self) -> Result<T, EplError> where T: GetCurveValue {
        match self.curve_type == T::CURVE_TYPE {
            true => Ok(T::get_value(&self.end)),
            false => Err(EplError::IncorrectCurveType((T::CURVE_TYPE, self.curve_type)))
        }
    }

    pub unsafe fn reset_as<T>(&mut self) where T: GetCurveValue {
        self.curve_type = T::CURVE_TYPE;
        // Wipe any existing data to prevent possible reading of corrupted data
        // (zero is valid for all types)
        self.start = std::mem::zeroed::<CurveTarget>();
        self.end = std::mem::zeroed::<CurveTarget>();
    }
    pub fn set_start_point(&mut self, value: Vec2) {
        self.point[0] = value;
    }
    pub fn set_end_point(&mut self, value: Vec2) {
        self.point[1] = value;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub struct Curve4Flags : u16 {
        const NO_REBUILDING_POINT_TABLE = 1 << 0;
    }
}

impl Default for Curve4Flags {
    fn default() -> Self {
        Self::empty()
    }
}

/// Four-point bezier curve. Mostly used for particle systems.
#[repr(C)]
#[derive(Default, Clone)]
pub struct Curve4 {
    curve_type: CurveType,
    target: [CurveTarget; 4],
    point: [Vec2; 2],
    table: [u16; 23],
    flags: Curve4Flags
}

impl Debug for Curve4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Curve4 {{ type: {:?}, points: ({}, {}) }}", self.curve_type, self.point[0], self.point[1])
    }
}

#[cfg(feature = "serialize")]
impl<A, T> GfdSerialize<A, T> for Curve4
where T: Debug + Read + Write + Seek + StreamIODevice,
      A: Allocator + Clone + Debug
{
    fn stream_read(stream: &mut Stream<A, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this = Curve4::default();
        this.curve_type = stream.read_u16()?.try_into()?;
        for i in 0..4 {
            match this.curve_type {
                CurveType::Integer => this.target[i].integer =  stream.read_u32()? as i32,
                CurveType::Float => this.target[i].float =  stream.read_f32()?,
                CurveType::Color => this.target[i].color =  RGBA::stream_read(stream, &mut ())?.into_raw(),
                CurveType::Vec2 => this.target[i].range =  Vec2::stream_read(stream, &mut ())?.into_raw(),
            }
        }
        for i in 0..2 {
            this.point[i] = Vec2::stream_read(stream, &mut ())?.into_raw();
        }
        stream.read_u16_slice(this.table.as_mut_slice())?;
        this.flags = stream
            .has_feature(GfdVersion::EplCurve4Field62)
            .map_or::<Result<Curve4Flags, Box<dyn Error>>, _>(
                Ok(Curve4Flags::default()),
                |_| Ok(Curve4Flags::from_bits_retain(stream.read_u16()?))
            )?;
        Ok(this.into())
    }
}

impl From<Curve2> for Curve4 {
    // Original function: gfdEPLCurve4MakeFromCurve
    fn from(value: Curve2) -> Self {
        let curve_type = value.curve_type;
        let table = value.table;
        let point = value.point;
        let target: [CurveTarget; 4] = match curve_type {
            CurveType::Integer => {
                let distance = unsafe { value.end.integer - value.start.integer };
                [
                    CurveTarget { integer: unsafe { value.start.integer } },
                    CurveTarget { integer: unsafe { ((distance as f32 / 3.) + value.start.integer as f32) as i32 } },
                    CurveTarget { integer: unsafe { ((distance as f32 * 2. / 3.) + value.start.integer as f32) as i32 } },
                    CurveTarget { integer: unsafe { value.end.integer } },
                ]
            },
            CurveType::Float => {
                let distance = unsafe { value.end.float - value.start.float };
                [
                    CurveTarget { float: unsafe { value.start.float } },
                    CurveTarget { float: unsafe { distance / 3. + value.start.float } },
                    CurveTarget { float: unsafe { distance * 2. / 3. + value.start.float } },
                    CurveTarget { float: unsafe { value.end.float } },
                ]
            },
            CurveType::Color => {
                let (dr, dg, db, da) = unsafe { (
                    value.end.color.get_red() - value.start.color.get_red(),
                    value.end.color.get_green() - value.start.color.get_red(),
                    value.end.color.get_blue() - value.start.color.get_blue(),
                    value.end.color.get_alpha() - value.start.color.get_alpha(),
                ) };
                let mf = |count: f32, point: f32| -> RGBA {
                    unsafe { RGBA::from_rgba_u8(
                        ((dr as f32 * point / count - 1.) + value.start.color.get_red() as f32) as u8,
                        ((dg as f32 * point / count - 1.) + value.start.color.get_green() as f32) as u8,
                        ((db as f32 * point / count - 1.) + value.start.color.get_blue() as f32) as u8,
                        ((da as f32 * point / count - 1.) + value.start.color.get_alpha() as f32) as u8,
                    ) }
                };

                [
                    CurveTarget { color: unsafe { value.start.color } },
                    CurveTarget { color: mf(4., 1.) },
                    CurveTarget { color: mf(4., 2.) },
                    CurveTarget { color: unsafe { value.end.color } },
                ]
            },
            CurveType::Vec2 => {
                let distance = unsafe { value.end.range - value.start.range };
                [
                    CurveTarget { range: unsafe { value.start.range } },
                    CurveTarget { range: unsafe { distance / 3. + value.start.range } },
                    CurveTarget { range: unsafe { distance * 2. / 3. + value.start.range } },
                    CurveTarget { range: unsafe { value.end.range } },
                ]
            },
        };
        Self {
            curve_type,
            target,
            point,
            table,
            flags: Curve4Flags::default()
        }
    }
}

impl Curve4 {
    pub fn get_target<T>(&self, index: usize) -> Result<T, EplError> where T: GetCurveValue {
        match self.curve_type == T::CURVE_TYPE && index < 4 {
            true => Ok(T::get_value(&self.target[index])),
            false => Err(EplError::IncorrectCurveType((T::CURVE_TYPE, self.curve_type)))
        }
    }

    pub unsafe fn reset_as<T>(&mut self) where T: GetCurveValue {
        self.curve_type = T::CURVE_TYPE;
        // Wipe any existing data to prevent possible reading of corrupted data
        // (zero is valid for all types)
        self.target = std::mem::zeroed::<[CurveTarget; 4]>();
    }

    pub fn set_target<T>(&mut self, value: T, index: usize) -> Result<(), EplError> where T: GetCurveValue {
        match self.curve_type == T::CURVE_TYPE && index < 4 {
            true => Ok(T::set_value(&mut self.target[index], value)),
            false => Err(EplError::IncorrectCurveType((T::CURVE_TYPE, self.curve_type)))
        }
    }

    pub fn set_all_targets<T>(&mut self, value: T) -> Result<(), EplError> where T: GetCurveValue + Copy {
        match self.curve_type == T::CURVE_TYPE {
            true => {
                for i in 0..4 {
                    T::set_value(&mut self.target[i], value);
                }
                Ok(())
            },
            false => Err(EplError::IncorrectCurveType((T::CURVE_TYPE, self.curve_type)))
        }
    }

    pub fn set_start_point(&mut self, value: Vec2) {
        self.point[0] = value;
    }
    pub fn set_end_point(&mut self, value: Vec2) {
        self.point[1] = value;
    }

    pub fn rebuild_point_table(&mut self) {
        /*
            let prog = 1. / 24.;
            for i in &mut self.table {
                *i = match self.flags.contains(Curve4Flags::NO_REBUILDING_POINT_TABLE) {
                    false => {
                        let mut fx = (self.point[0].x + self.point[1].x) * 0.375 + 0.125;
                        let mut fy = (self.point[0].y + self.point[1].y) * 0.375 + 0.125;
                        let mut fvar4 = 0.5;
                        let mut fvar5 = 0.25;
                        while fx != prog && fvar5 >= 0.0001 {
                            if prog <= fx {
                                if prog < fx {
                                    fvar4 -= fvar5;
                                }
                            } else {
                                fvar4 += fvar5;
                            }
                            fx = 1. - fvar4;
                            fvar5 *= 0.5;
                        }
                    },
                    true => 0
                };
            }
        */
        }
}

pub mod ffi {

}

#[cfg(test)]
pub mod tests {

}

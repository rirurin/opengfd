#![allow(dead_code)]
//! NOTE: Float arrays are used instead of glam's vectors to keep alignemnt value as alignof(f32)

use std::ops::Deref;
use bitflags::bitflags;
use glam::{FloatExt, Quat, Vec2, Vec3, Vec4 };
use std::ptr::NonNull;
use half::f16;

pub trait AnimKey {
    fn blend(&self, other: &Self, rate: f32) -> Self;
}

// BASIC TYPES

impl AnimKey for f32 {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        self.lerp(*other, rate)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Float(f32);
impl Deref for Float {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AnimKey for Float {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Self(self.lerp(**other, rate))
    }
}

impl Default for Float {
    fn default() -> Self {
        Self(f32::default())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FloatStep(f32);
impl Deref for FloatStep {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AnimKey for FloatStep {
    fn blend(&self, _other: &Self, _rate: f32) -> Self {
        FloatStep(**self)
    }
}

impl Default for FloatStep {
    fn default() -> Self {
        Self(f32::default())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Alpha {
    interp: TransformType,
    data: f32
}

impl AnimKey for Alpha {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let interp = self.interp;
        match interp {
            TransformType::Step => Self { interp, data: self.data },
            TransformType::Blend => Self { interp, data: self.data.blend(&other.data, rate) },
            TransformType::Cosine => {
                let rate_adj = 1. - (rate * std::f32::consts::FRAC_PI_2).cos();
                Self { interp, data: self.data.blend(&other.data, rate_adj) }
            },
            TransformType::Sine => {
                let rate_adj = (rate * std::f32::consts::FRAC_PI_2).sin();
                Self { interp, data: self.data.blend(&other.data, rate_adj) }
            }
        }
    }
}

impl Default for Alpha {
    fn default() -> Self {
        Self {
            interp: TransformType::Step,
            data: f32::default()
        }
    }
}

// Wrapper type for a 4-byte aligned Quaternion
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion([f32; 4]);

impl AnimKey for Quaternion {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Quaternion(Quat::from_array(self.0.clone()).lerp(Quat::from_array(other.0.clone()), rate).into())
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self([f32::default(); 4])
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyTR {
    t: Vec3,
    r: Quaternion
}

impl AnimKey for KeyTR {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Self {
            t: self.t.lerp(other.t, rate),
            r: self.r.blend(&other.r, rate)
        }
    }
}

impl Default for KeyTR {
    fn default() -> Self {
        Self {
            t: Vec3::default(),
            r: Quaternion::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyCompressedTR {
    t: [f16; 3],
    r: [f16; 4]
}

impl From<KeyCompressedTR> for KeyTR {
    fn from(value: KeyCompressedTR) -> Self {
        let t = Vec3::from_array(std::array::from_fn::<_, 3, _>(|i| value.t[i].into()));
        let r = Quaternion(std::array::from_fn::<_, 4, _>(|i| value.r[i].into()));
        Self { t, r }
    }
}

impl From<KeyTR> for KeyCompressedTR {
    fn from(value: KeyTR) -> Self {
        let t = value.t.to_array();
        let t: [f16; 3] = std::array::from_fn::<_, 3, _>(|i| f16::from_f32(t[i]));
        let r: [f16; 4] = std::array::from_fn::<_, 4, _>(|i| f16::from_f32(value.r.0[i]));
        Self { t, r }
    }
}

impl AnimKey for KeyCompressedTR {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let self_f32: KeyTR = (*self).into();
        let other_f32: KeyTR = (*other).into();
        let out = self_f32.blend(&other_f32, rate);
        out.into()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyCompressedTRS {
    t: [f16; 3],
    r: [f16; 4],
    s: [f16; 3]
}

impl From<KeyCompressedTRS> for KeyTRS {
    fn from(value: KeyCompressedTRS) -> Self {
        let t = Vec3::from_array(std::array::from_fn::<_, 3, _>(|i| value.t[i].into()));
        let r = Quaternion(std::array::from_fn::<_, 4, _>(|i| value.r[i].into()));
        let s = Vec3::from_array(std::array::from_fn::<_, 3, _>(|i| value.s[i].into()));
        Self { t, r, s }
    }
}

impl From<KeyTRS> for KeyCompressedTRS {
    fn from(value: KeyTRS) -> Self {
        let t = value.t.to_array();
        let s = value.s.to_array();
        let t: [f16; 3] = std::array::from_fn::<_, 3, _>(|i| f16::from_f32(t[i]));
        let r: [f16; 4] = std::array::from_fn::<_, 4, _>(|i| f16::from_f32(value.r.0[i]));
        let s: [f16; 3] = std::array::from_fn::<_, 3, _>(|i| f16::from_f32(s[i]));
        Self { t, r, s }
    }
}

impl AnimKey for KeyCompressedTRS {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let self_f32: KeyTRS = (*self).into();
        let other_f32: KeyTRS = (*other).into();
        let out = self_f32.blend(&other_f32, rate);
        out.into()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyTRS {
    t: Vec3,
    r: Quaternion,
    s: Vec3,
}

impl AnimKey for KeyTRS {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Self {
            t: self.t.lerp(other.t, rate),
            r: self.r.blend(&other.r, rate),
            s: self.s.lerp(other.s, rate),
        }
    }
}

impl Default for KeyTRS {
    fn default() -> Self {
        Self {
            t: Vec3::default(),
            r: Quaternion::default(),
            s: Vec3::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyRGB(Vec3);

impl AnimKey for KeyRGB {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Self(self.0.lerp(other.0, rate))    
    }
}

impl Default for KeyRGB {
    fn default() -> Self {
        Self(Vec3::default())
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyRGBA([f32; 4]);

impl AnimKey for KeyRGBA {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Self(Vec4::from_array(self.0.clone()).lerp(Vec4::from_array(other.0.clone()), rate).into())
    }
}

impl Default for KeyRGBA {
    fn default() -> Self {
        Self([f32::default(); 4])
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyUV {
    offset: Vec2,
    tile: Vec2,
    angle: f32
}

impl AnimKey for KeyUV {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        Self {
            offset: self.offset.lerp(other.offset, rate),
            tile: self.tile.lerp(other.tile, rate),
            angle: self.angle.lerp(other.angle, rate)
        }
    }
}

impl Default for KeyUV {
    fn default() -> Self {
        Self {
            offset: Vec2::default(),
            tile: Vec2::default(),
            angle: f32::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyUVStep {
    offset: Vec2,
    tile: Vec2,
    angle: f32
}

impl AnimKey for KeyUVStep {
    fn blend(&self, _other: &Self, _rate: f32) -> Self {
        self.clone()
    }
}

impl Default for KeyUVStep {
    fn default() -> Self {
        Self {
            offset: Vec2::default(),
            tile: Vec2::default(),
            angle: f32::default()
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum TransformType {
    Step = 0,
    Blend = 1,
    Cosine = 2,
    Sine = 3
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyTransform {
    interp: TransformType,
    data: KeyTRS
}

impl AnimKey for KeyTransform {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let interp = self.interp;
        match interp {
            TransformType::Step => Self { interp, data: self.data.clone() },
            TransformType::Blend => Self { interp, data: self.data.blend(&other.data, rate) },
            TransformType::Cosine => {
                let rate_adj = 1. - (rate * std::f32::consts::FRAC_PI_2).cos();
                Self { interp, data: self.data.blend(&other.data, rate_adj) }
            },
            TransformType::Sine => {
                let rate_adj = (rate * std::f32::consts::FRAC_PI_2).sin();
                Self { interp, data: self.data.blend(&other.data, rate_adj) }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyMotion {
    interp: TransformType,
    motion: u32,
    data: KeyController,
    next: KeyController,
}

impl AnimKey for KeyMotion {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        self.clone()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyController {
    flags: u32,
    anim: u32,
    blend: f32,
    weight: f32,
    speed: f32,
}

impl AnimKey for KeyController {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        self.clone()
    }
}


// OBJECT TYPES

pub fn try_blend<T>(in0: T, in1: T, rate: f32, cond: bool) -> T
where T: Default + AnimKey {
    match cond {
        true => in0.blend(&in1, rate),
        false => T::default()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyNode {
    dirty: NodeDirty,
    trs: KeyTRS,
    visible: f32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct NodeDirty : u32 {
        const TR = 1 << 0;
        const Scale = 1 << 1;
        const Visible = 1 << 2;
    }
}

impl AnimKey for KeyNode {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let dirty = other.dirty;
        let trs = if dirty.contains(NodeDirty::TR | NodeDirty::Scale) {
            self.trs.blend(&other.trs, rate)
        } else if dirty.contains(NodeDirty::TR) {
            let t= self.trs.t.lerp(other.trs.t, rate);
            let r = self.trs.r.blend(&other.trs.r, rate);
            KeyTRS { t, r, s: Vec3::default() }
        } else {
            KeyTRS::default()
        };
        let visible = try_blend(self.visible, other.visible, rate, dirty.contains(NodeDirty::Visible));
        Self {  dirty, trs, visible  }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialDirty : u32 {
        const Ambient = 1 << 0;
        const Diffuse = 1 << 1;
        const Specular = 1 << 2;
        const Shininess = 1 << 3;
        const Reflectivity = 1 << 4;
        const Diffusivity = 1 << 5;
        const Transparency = 1 << 6;
        const UV = 1 << 7;
        const Emissive = 1 << 8;
        const Reflection = 1 << 9;
        const UVMultiple = 1 << 10;
        const TransparencyMultiple = 1 << 11;
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct KeyMaterial {
    dirty: MaterialDirty,
    ambient: KeyRGB,
    diffuse: KeyRGB,
    specular: KeyRGB,
    shininess: f32,
    reflect: KeyRGBA,
    diffusivity: f32,
    transparency: f32,
    uv: KeyUV,
    emissive: KeyRGB,
    reflection: f32,
    uv_multiple: KeyUV,
    transparency_multiple: f32
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ShapeDirty : u32 {
        const Weight = 1 << 0;
    }
}

impl AnimKey for KeyMaterial {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let dirty = other.dirty;
        let ambient = try_blend(self.ambient, other.ambient, rate, dirty.contains(MaterialDirty::Ambient));
        let diffuse = try_blend(self.diffuse, other.diffuse, rate, dirty.contains(MaterialDirty::Diffuse));
        let specular = try_blend(self.specular, other.specular, rate, dirty.contains(MaterialDirty::Specular));
        let shininess = try_blend(self.shininess, other.shininess, rate, dirty.contains(MaterialDirty::Shininess));
        let reflect = try_blend(self.reflect, other.reflect, rate, dirty.contains(MaterialDirty::Reflectivity));
        let diffusivity = try_blend(self.diffusivity, other.diffusivity, rate, dirty.contains(MaterialDirty::Diffusivity));
        let transparency = try_blend(self.transparency, other.transparency, rate, dirty.contains(MaterialDirty::Transparency));
        let uv = try_blend(self.uv, other.uv, rate, dirty.contains(MaterialDirty::UV));
        let emissive = try_blend(self.emissive, other.emissive, rate, dirty.contains(MaterialDirty::Emissive));
        let reflection = try_blend(self.reflection, other.reflection, rate, dirty.contains(MaterialDirty::Reflection));
        let uv_multiple = try_blend(self.uv_multiple, other.uv_multiple, rate, dirty.contains(MaterialDirty::UVMultiple));
        let transparency_multiple = try_blend(self.transparency_multiple, other.transparency_multiple, rate, dirty.contains(MaterialDirty::TransparencyMultiple));
        Self {
            dirty, ambient, diffuse, specular, shininess,
            reflect, diffusivity, transparency, uv,
            emissive, reflection, uv_multiple, transparency_multiple
        }
    }
}


#[repr(C, align(16))]
#[derive(Debug)]
pub struct KeyShape {
    dirty: ShapeDirty,
    weight: f32,
}

impl AnimKey for KeyShape {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let dirty = other.dirty;
        let weight = try_blend(self.weight, other.weight, rate, dirty.contains(ShapeDirty::Weight));
        Self { dirty, weight }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CameraDirty : u32 {
        const Fovy = 1 << 0;
        const Roll = 1 << 1;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyCamera {
    dirty: CameraDirty,
    fovy: f32,
    roll: f32
}

impl AnimKey for KeyCamera {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let dirty = other.dirty;
        let fovy = try_blend(self.fovy, other.fovy, rate, dirty.contains(CameraDirty::Fovy));
        let roll = try_blend(self.roll, other.roll, rate, dirty.contains(CameraDirty::Roll));
        Self { dirty, fovy, roll }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LightDirty : u32 {
        const Diffuse = 1 << 0;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyLight {
    dirty: LightDirty,
    diffuse: KeyRGB
}

impl AnimKey for KeyLight {
    fn blend(&self, other: &Self, rate: f32) -> Self {
        let dirty = other.dirty;
        let diffuse = try_blend(self.diffuse, other.diffuse, rate, dirty.contains(LightDirty::Diffuse));
        Self { dirty, diffuse }
    }
}


#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum KeyType {
    NULL = 0,
    NodePR = 1, // TR
    NodePRS = 2, // TRS
    Vector3 = 3, // RGB
    Quaternion = 4, // RGBA
    Single = 5, // FLOAT
    Vector3_2 = 6, // RGB
    Vector3_3 = 7, // RGB
    Vector3_4 = 8, // RGB
    Single_2 = 9, // FLOAT
    Quaternion_2 = 10, // RGBA
    Single_3 = 11, // FLOAT
    MaterialSingle_4 = 12, // FLOAT
    Single5 = 13, // UV
    MaterialVector3_5 = 14, // RGB
    Single_5 = 15, // FLOAT
    Single_6 = 16, // FLOAT
    PRSByte = 17, // TRANSFORM
    Single3Byte = 18, // COLOR
    SingleByte = 19, // ALPHA
    Single5_2 = 20, // UV
    Single5Alt = 21, // UVStep
    Type22 = 22, // MOTION
    CameraFieldOfView = 23, // FLOAT
    Single_8 = 24, // FLOAT
    SingleAlt_2 = 25, // FLOATStep
    NodePRHalf = 26, // CompressTR
    NodePRSHalf = 27, // CompressTRS
    NodePRHalf_2 = 28, // CompressTR
    MaterialSingle_9 = 29, // FLOAT
    SingleAlt_3 = 30, // FLOATStep
    Type31 = 31,
    NodeRHalf = 32,
    NodeSHalf = 33,
	P5R_34 = 34,
	P5R_35 = 35,
    P5R_36 = 36,
}

#[repr(C)]
pub struct KeyMaxDiff {
    t: Vec3,
    s: Vec3
}

#[repr(C)]
pub struct KeyList {
    key_type: KeyType,
    key_count: u32,
    keys: Option<NonNull<u8>>,
    times: Option<NonNull<f32>>,
    diff: KeyMaxDiff
}

impl KeyList {
    pub fn get_key_type(&self) -> KeyType {
        self.key_type
    }
    pub fn get_times(&self) -> &[f32] {
        match self.times {
            Some(s) => unsafe { std::slice::from_raw_parts(s.as_ptr(), self.key_count as usize) },
            None => &[]
        }
    }
}
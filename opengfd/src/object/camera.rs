use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use allocator_api2::alloc::Allocator;
use crate::{
    kernel::allocator::GfdAllocator,
    object::{
        node::Node,
        object::{CastFromObject, Object, ObjectId}
    },
    utility::reference::Reference,
};
use glam::{ Mat4, Vec3, Vec3A, Vec4, Quat };
use std::ptr::NonNull;
use crate::kernel::version::GfdVersion;

#[cfg(feature = "serialize")]
use crate::utility::stream::{
    DeserializationHeap,
    DeserializationStrategy,
    GfdSerialize,
    SerializationSingleAllocator,
    Stream,
    StreamIODevice
};

type DynRes<T> = Result<T, Box<dyn Error>>;

#[repr(C)]
#[derive(Debug)]
pub struct Camera<A = GfdAllocator> 
where A: Allocator + Clone
{
    super_: Object<A>,
    view: Mat4,
    projection: Mat4,
    #[cfg(feature = "v1-core")]
    flags: u32,
    plane_frustum: [Vec4; 6usize],
    vec_frustrum: [Vec3A; 8usize],
    #[cfg(feature = "v1-core")]
    field190: [u8; 0x10],
    #[cfg(feature = "v2-core")]
    field180: f32,
    near_clip: f32,
    far_clip: f32,
    fovy: f32,
    aspect: f32,
    roll: f32,
    #[cfg(feature = "v2-core")]
    field11_0x198: u8,
    #[cfg(feature = "v2-core")]
    field12_0x19c: f32,
    #[cfg(feature = "v2-core")]
    field13_0x1a0: f32,
    dirty: i32,
    sync: NonNull<Object<A>>,
    ref_: Reference,
    _allocator: A
}

impl<A> Camera<A>
where A: Allocator + Clone
{
    /// Original function: gfdCameraGetAspect
    pub fn get_aspect(&self) -> f32 { self.aspect }
    pub fn get_far_clip(&self) -> f32 { self.far_clip }
    /// Original function: gfdCameraGetFovy
    pub fn get_fovy(&self) -> f32 { self.fovy }
    pub fn get_near_clip(&self) -> f32 { self.near_clip }
    /// Original function: gfdCameraGetNode
    pub fn get_node(&self) -> Option<&Node<A>> {
        self.super_.get_parent()
    }
    pub fn get_projection_transform(&self) -> Mat4 {
        self.projection
    }
    pub fn get_roll(&self) -> f32 { self.roll }
    /// Original function: gfdCameraGetViewTransform
    pub fn get_view_transform(&self) -> Mat4 {
        self.view
    }
    /// Original function: gfdCameraSetAspect
    pub fn set_aspect(&mut self, value: f32) {
        self.aspect = value;
        self.dirty |= 1;
    }
    /// Original function: gfdCameraSetFarClipPlane
    pub fn set_far_clip(&mut self, value: f32) {
        self.far_clip = value;
        self.dirty |= 1;
    }
    /// Original function: gfdCameraSetFovy
    pub fn set_fovy(&mut self, value: f32) {
        self.fovy = value;
        self.dirty |= 1;
    }
    /// Original function: gfdCameraSetNearClipPlane
    pub fn set_near_clip(&mut self, value: f32) {
        self.near_clip = value;
        self.dirty |= 1;
    }
    /// Original function: gfdCameraSetRoll
    pub fn set_roll(&mut self, value: f32) {
        self.roll = value;
    }
    /// Original function: gfdCameraSetViewTransform
    pub fn set_view_transform(&mut self, value: Mat4) {
        self.view = value;
    }
    pub fn get_translate(&self) -> Vec3A {
        self.view.to_scale_rotation_translation().2.into()
    }
    pub fn get_rotate(&self) -> Quat {
        self.view.to_scale_rotation_translation().1
    }
    pub fn get_scale(&self) -> Vec3A {
        self.view.to_scale_rotation_translation().0.into()
    }
    pub fn get_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        self.view.to_scale_rotation_translation()
    }
    pub fn get_scale_rotation_translation_mut(&mut self) -> (Vec3, Quat, Vec3) {
        self.view.to_scale_rotation_translation()
    }
    pub fn set_projection_transform(&mut self, value: Mat4) {
        self.projection = value;
    }

    pub fn set_translate(&mut self, value: Vec3A) {
        self.view.w_axis = Vec4::new(value.x, value.y, value.z, 1.);
    }

    // For imgui
    pub fn get_near_clip_mut(&mut self) -> &mut f32 { &mut self.near_clip }
    pub fn get_far_clip_mut(&mut self) -> &mut f32 { &mut self.far_clip }
    pub fn get_fovy_mut(&mut self) -> &mut f32 { &mut self.fovy }
    pub fn get_aspect_ratio_mut(&mut self) -> &mut f32 { &mut self.aspect }
    pub fn get_roll_mut(&mut self) -> &mut f32 { &mut self.roll }

    pub fn get_super(&self) -> &Object<A> { &self.super_ }
    pub fn get_super_mut(&mut self) -> &mut Object<A> { &mut self.super_ }
}

#[cfg(feature = "v2-core")]
impl<A> Camera<A>
where A: Allocator + Clone
{
    pub fn get_field198_mut(&mut self) -> &mut u8 { &mut self.field11_0x198 }
    pub fn get_field19c_mut(&mut self) -> &mut f32 { &mut self.field12_0x19c }
    pub fn get_field1a0_mut(&mut self) -> &mut f32 { &mut self.field13_0x1a0 }
}

impl<A> CastFromObject for Camera<A>
where A: Allocator + Clone
{
    const OBJECT_ID: ObjectId = ObjectId::Camera;
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Camera<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> DynRes<DeserializationHeap<Self, AObject>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        unsafe { this.super_.set_id(ObjectId::Camera) };
        this.ref_ = Reference::new();
        this.stream_read_inner(stream)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Camera<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> DynRes<()>
    where T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug {
        self.view = Mat4::stream_read(stream, &mut ())?.into_raw();
        self.near_clip = stream.read_f32()?;
        self.far_clip = stream.read_f32()?;
        self.fovy = stream.read_f32()?;
        self.aspect = stream.read_f32()?;
        self.roll = stream.has_feature(GfdVersion::CameraAddRoll).map_or(Ok(0.), |_| stream.read_f32())?;
        #[cfg(feature = "v2-core")]
        {
            self.field11_0x198 = stream.has_feature(GfdVersion::CameraAddUnkMetaphor).map_or(Ok(0), |_| stream.read_u8())?;
            self.field12_0x19c = stream.has_feature(GfdVersion::CameraAddUnkMetaphor).map_or(Ok(0.), |_| stream.read_f32())?;
            self.field13_0x1a0 = stream.has_feature(GfdVersion::CameraAddUnkMetaphor).map_or(Ok(0.), |_| stream.read_f32())?;
        }
        Ok(())
    }
}
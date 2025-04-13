use allocator_api2::alloc::Allocator;
use crate::{
    kernel::allocator::GfdAllocator,
    object::{
        node::Node,
        object::Object
    },
    utility::reference::Reference
};
use glam::{ Mat4, Vec3A, Vec4 };
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug)]
pub struct Camera<A = GfdAllocator> 
where A: Allocator + Clone
{
    _super: Object<A>,
    view: Mat4,
    projection: Mat4,
    plane_frustum: [Vec4; 6usize],
    vec_frustrum: [Vec3A; 8usize],
    near_clip: f32,
    far_clip: f32,
    fovy: f32,
    aspect: f32,
    roll: f32,
    field11_0x198: f32,
    field12_0x19c: f32,
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
        self._super.get_parent()
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
}
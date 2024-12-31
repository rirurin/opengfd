use crate::utility::reference::Reference;
use glam::{ Mat4, Vec3A, Vec4 };
use super::object::Object;
use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 448usize)]
pub struct Camera {
    #[field_offset(0usize)]
    pub object: Object,
    #[field_offset(32usize)]
    pub view: Mat4,
    #[field_offset(96usize)]
    pub projection: Mat4,
    #[field_offset(160usize)]
    pub plane_frustum: [Vec4; 6usize],
    #[field_offset(256usize)]
    pub vec_frustrum: [Vec3A; 8usize],
    #[field_offset(388usize)]
    pub near_clip: f32,
    #[field_offset(392usize)]
    pub far_clip: f32,
    #[field_offset(396usize)]
    pub fovy: f32,
    #[field_offset(400usize)]
    pub aspect: f32,
    #[field_offset(404usize)]
    pub roll: f32,
    #[field_offset(408usize)]
    pub field11_0x198: f32,
    #[field_offset(412usize)]
    pub field12_0x19c: f32,
    #[field_offset(416usize)]
    pub field13_0x1a0: f32,
    #[field_offset(420usize)]
    pub dirty: i32,
    #[field_offset(424usize)]
    pub sync: *mut Object,
    #[field_offset(432usize)]
    pub ref_: Reference,
}

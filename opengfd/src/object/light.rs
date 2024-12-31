use crate::utility::misc::RGBAFloat;
use glam::Vec3A;
use super::object::Object;
use riri_mod_tools_proc::ensure_layout;

#[allow(non_snake_case)]
#[ensure_layout(size = 192usize)]
pub struct Light {
    #[field_offset(0usize)]
    pub super_: Object,
    #[field_offset(32usize)]
    pub position: Vec3A,
    #[field_offset(48usize)]
    pub direction: Vec3A,
    #[field_offset(64usize)]
    pub ambient: RGBAFloat,
    #[field_offset(80usize)]
    pub diffuse: RGBAFloat,
    #[field_offset(96usize)]
    pub specular: RGBAFloat,
    #[field_offset(112usize)]
    pub attenuation: LightAttenuation,
    #[field_offset(140usize)]
    pub flags: u32,
    #[field_offset(144usize)]
    pub type_: u32,
    #[field_offset(148usize)]
    pub scale: f32,
    #[field_offset(152usize)]
    pub alpha: f32,
    #[field_offset(156usize)]
    pub scale2: f32,
    #[field_offset(160usize)]
    pub ref_: u32,
    #[field_offset(168usize)]
    pub field20_0xa8: *mut ::std::os::raw::c_void,
    #[field_offset(176usize)]
    pub prev: *mut Light,
    #[field_offset(184usize)]
    pub next: *mut Light,
}

#[repr(C)]
pub struct LightAttenuation {
    pub kc: f32,
    pub kl: f32,
    pub kq: f32,
    pub ds: f32,
    pub de: f32,
    pub theta: f32,
    pub phi: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct LightContainer {
    array: [*mut Light; 3],
    ref_: crate::utility::reference::Reference
}

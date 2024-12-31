use super::{
    light::LightContainer,
    object::Object
};
use glam::Vec3A;
use crate::{
    graphics::{
        cull::CullObject,
        material::Material,
        resources::ResBuffer
    },
    utility::{
        misc::{ BoundingBox, BoundingSphere },
        reference::Reference
    }
};
use riri_mod_tools_proc::ensure_layout;

#[allow(non_snake_case)]
#[ensure_layout(size = 448usize)]
pub struct Geometry {
    #[field_offset(0usize)]
    pub _super: Object,
    #[field_offset(32usize)]
    pub flags: i32,
    #[field_offset(36usize)]
    pub type_: i32,
    #[field_offset(40usize)]
    pub lock: i32,
    #[field_offset(44usize)]
    pub prim: i8,
    #[field_offset(48usize)]
    pub fvf: i32,
    #[field_offset(52usize)]
    pub numVertices: i32,
    #[field_offset(56usize)]
    pub numIndices: i32,
    #[field_offset(60usize)]
    pub numTriangles: i32,
    #[field_offset(64usize)]
    pub vertexBuffer: *mut ::std::os::raw::c_void,
    #[field_offset(72usize)]
    pub vertexUsage: i32,
    #[field_offset(80usize)]
    pub indexBuffer: *mut ::std::os::raw::c_void,
    #[field_offset(88usize)]
    pub skin: *mut ::std::os::raw::c_void,
    #[field_offset(96usize)]
    pub material: *mut Material,
    #[field_offset(104usize)]
    pub morphTargets: *mut ::std::os::raw::c_void,
    #[field_offset(112usize)]
    pub lightContainer: *mut LightContainer,
    #[field_offset(120usize)]
    pub vertices: [*mut ::std::os::raw::c_void; 2usize],
    #[field_offset(136usize)]
    pub indices: *mut ::std::os::raw::c_void,
    #[field_offset(144usize)]
    pub boundingBox: BoundingBox,
    #[field_offset(168usize)]
    pub boundingSphere: BoundingSphere,
    #[field_offset(184usize)]
    pub localOBB: *mut [Vec3A; 8usize],
    #[field_offset(192usize)]
    pub cull: [CullObject; 3usize],
    #[field_offset(240usize)]
    pub resources: [GeometryCommand; 3usize],
    #[field_offset(336usize)]
    pub reflection_mat: *mut Material,
    #[field_offset(344usize)]
    pub outline_mat: *mut Material,
    #[field_offset(352usize)]
    pub ssss_mat: *mut Material,
    #[field_offset(360usize)]
    pub lod_start: f32,
    #[field_offset(364usize)]
    pub lod_end: f32,
    #[field_offset(368usize)]
    pub color_mask: u32,
    #[field_offset(372usize)]
    pub stencil_enable: bool,
    #[field_offset(376usize)]
    pub stencil_fail: u32,
    #[field_offset(380usize)]
    pub stencil_depthfail: u32,
    #[field_offset(384usize)]
    pub stencil_depthpass: u32,
    #[field_offset(388usize)]
    pub stencil_func: u32,
    #[field_offset(392usize)]
    pub stencil_ref: u8,
    #[field_offset(393usize)]
    pub stencil_mask: u8,
    #[field_offset(396usize)]
    pub blend_enable: bool,
    #[field_offset(397usize)]
    pub field41_0x18d: u8,
    #[field_offset(398usize)]
    pub field42_0x18e: u8,
    #[field_offset(399usize)]
    pub field43_0x18f: u8,
    #[field_offset(400usize)]
    pub blend_srcColor: u16,
    #[field_offset(402usize)]
    pub blend_dstColor: u16,
    #[field_offset(404usize)]
    pub blend_srcAlpha: u16,
    #[field_offset(406usize)]
    pub blend_dstAlpha: u16,
    #[field_offset(408usize)]
    pub blend_colorOp: u16,
    #[field_offset(410usize)]
    pub blend_alphaOp: u16,
    #[field_offset(412usize)]
    pub field50_0x19c: u16,
    #[field_offset(414usize)]
    pub field51_0x19e: u16,
    #[field_offset(416usize)]
    pub field52_0x1a0: u8,
    #[field_offset(417usize)]
    pub field53_0x1a1: u8,
    #[field_offset(424usize)]
    // pub jobData: *mut JobData,
    pub job_data: *mut u8,
    #[field_offset(432usize)]
    pub container: *mut ::std::os::raw::c_void,
    #[field_offset(440usize)]
    pub ref_: Reference
}

#[ensure_layout(size = 32usize)]
pub struct GeometryCommand {
    #[field_offset(0usize)]
    pub prepare: *mut ResBuffer,
    #[field_offset(8usize)]
    pub render: *mut ResBuffer,
    #[field_offset(16usize)]
    pub shadow: *mut ResBuffer,
    #[field_offset(24usize)]
    pub reflection: *mut ResBuffer,
}

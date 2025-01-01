#![allow(unused_imports, dead_code)]
use bitflags::bitflags;
use crate::{
    device::ngr::renderer::{ ps, vs },
    graphics::{
        cull::CullObject,
        environment,
        render::cmd_buffer::CmdBuffer,
        render_ot::{ RenderOtGroup, RenderOtList },
        resources::{ Resources, ResBuffer },
        scene::Scene,
        shader::shader::ShaderSource,
        texture::Texture
    },
    kernel::{
        asset::Asset,
        global_common::RENDER_STATES
    },
    utility::{
        item_array::ItemArray,
        misc::Range,
        mutex::{ Mutex, RecursiveMutex },
        name::Name
    }
};
use glam::{ Vec3, Vec3A, Mat4 };
use riri_mod_tools_proc::ensure_layout;

#[repr(C)]
#[derive(Debug)]
pub struct Global {
    flags: u32,
    elapsed_time: f32,
    delta_time: f32,
    loop_counter: i32,
    field10: i32,
    field14: i32,
    pub graphics: GraphicsGlobal
}

// GRAPHICS START

pub(crate) const RENDER_LISTS: usize = 3;
pub(crate) const SHADER_SOURCE: usize = 219;
pub(crate) const FIXED_VERTEX_SHADERS: usize = 70;
pub(crate) const FIXED_PIXEL_SHADERS: usize = 198;
pub(crate) const FIXED_GEOMETRY_SHADERS: usize = 1;
pub(crate) const FIXED_COMPUTE_SHADERS: usize = 30;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GraphicsFlags : u32 {
        const HasInfiniteOcean = 1 << 15;
        const HasTemperare = 1 << 18;
    }
}

#[derive(Debug)]
pub struct GraphicsCBufferViewProjection(Mat4, Mat4);

#[repr(C)]
#[derive(Debug)]
pub struct GraphicsCBufferSystem {
    mtx_in: Mat4,
    mtx_in_view: Mat4,
    mtx_proj: Mat4,
    mtx_inv_proj: Mat4
}

// NOTE: Metaphor stores this set of buffer data every 0x59c. This means that reading vectors will
// be unaligned! Make sure to read to these types using std::ptr::read_unaligned to avoid UB!
#[repr(C, packed(4))]
pub struct GraphicsCBuffer {
    viewproj: GraphicsCBufferViewProjection,
    system: GraphicsCBufferSystem,
    view_proj_eye_position: Vec3,
    unk0: [u8; 88],
    fog_param_multiplier: f32,
    view_proj_fovy: f32,
    hdr: [u8; 76],
    unk1: [u8; 32],
    color_correct: [u8; 32],
    todo: [u8; 0x324]
}

impl std::fmt::Debug for GraphicsCBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO!")
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GraphicsGlobal {
    flags: GraphicsFlags,
    video_mode: super::global_common::VideoMode,
    fps: u32,
    fvf: u32,
    scene: [*mut Scene; 2],
    cmd_buffer: *mut CmdBuffer,
    texture_head: *mut Texture,
    texture_mutex: RecursiveMutex,
    material_array_stack: [*mut ItemArray<usize>; 8],
    material_array_count: u32,
    material_mutex: RecursiveMutex,
    shader_vtx_head: *mut vs::VertexShader,
    shader_vtx_mutex: Mutex,
    shader_frg_head: *mut ps::PixelShader,
    shader_frg_mutex: Mutex,
    shader_geo_head: *mut u8,
    shader_geo_mutex: Mutex,
    shader_cmp_head: *mut u8,
    shader_cmp_mutex: Mutex,
    asset_head: *mut Asset,
    asset_mutex: Mutex,
    asset_editor_mode: bool,
    ot_shadow_list: [*mut RenderOtList; RENDER_LISTS],
    ot_render_list: [*mut RenderOtList; RENDER_LISTS],
    ot_prepare_list: [*mut RenderOtList; RENDER_LISTS],
    ot_cull_object: *mut CullObject,
    prio_max: u32,
    ot: *mut [RenderOtGroup; 7],
    prio_group_max: u32,
    frame_id: u32,
    widget_prio: u32,
    dbg_fnt_prio: u32,
    mouse_prio: u32,
    resources: *mut Resources,
    setup: *mut ResBuffer,
    info: [GraphicsCBuffer; 3],
    unk: [u8; 4368],
    render_state_current: [usize; RENDER_STATES],
    render_state_stack: [[usize; 2]; RENDER_STATES],
    shader_source: [[*mut ShaderSource; RENDER_LISTS]; SHADER_SOURCE],
    pub shader_vertex: [*mut vs::VertexShader; FIXED_VERTEX_SHADERS],
    pub shader_pixel: [*mut ps::PixelShader; FIXED_PIXEL_SHADERS],
    pub shader_geometry: [*mut u8; FIXED_GEOMETRY_SHADERS],
    pub shader_compute: [*mut u8; FIXED_COMPUTE_SHADERS],
    pub shader_current_vertex: *mut vs::VertexShader,
    pub shader_current_fragment: *mut ps::PixelShader,
    pub shader_current_geometry: *mut u8,
    pub shader_current_compute: *mut u8,
    unk1: [u8; 136],
    shader_outline_texture: *mut Texture,
    shader_noise_texture: *mut Texture,
    shader_edge_dark_texture: *mut Texture,
    texture_4558: *mut Texture, 
}

// GRAPHICS END
// PHYSICS START

#[repr(C)]
#[derive(Debug)]
pub struct PhysicsWindParams {
    direction: Vec3A,
    power: Range,
    cycle: Range
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicsGlobalWind {
    current: Vec3A,
    params: PhysicsWindParams,
    power: f32,
    cycle: f32,
    delta: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicsGlobal {
    gravity: Vec3A,
    wind: PhysicsGlobalWind
}

// PHYSICS END
// TASK START
/*
pub struct TaskGlobal {
    pub flag: uint,
    pub begin: gfdTaskLinkList,
    pub update: gfdTaskLinkList,
    pub render: gfdTaskLinkList,
    pub end: gfdTaskLinkList,
    pub release: gfdTaskLinkList,
    pub taskCount: uint,
    pub pad: gfdTaskLinkList,
    pub detachMutex: *mut gfdTask,
    pub current: *mut gfdTask,
}
*/

// TASK END

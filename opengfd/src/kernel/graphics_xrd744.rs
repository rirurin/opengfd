use crate::kernel::global::{ FIXED_PIXEL_SHADERS, FIXED_VERTEX_SHADERS, MATERIAL_LISTS, RENDER_LISTS, RENDER_STATES, SCENE_LISTS, SHADER_SOURCE };
use crate::kernel::init::VideoMode;
use crate::utility::item_array::ItemArray;
use crate::utility::mutex::{RecursiveMutex, RecursiveMutexGuard};
use glam::Vec2;
use crate::graphics::scene::Scene;
use crate::object::camera::Camera;

include!("graphics_common.rs");

pub struct GraphicsGlobal;
impl GraphicsGlobal {
    pub fn get_gfd_graphics_global() -> &'static dyn GraphicsState {
        unsafe { &crate::globals::get_gfd_global_unchecked().graphics }
    }
    pub fn get_gfd_graphics_global_mut() -> &'static mut dyn GraphicsState {
        unsafe { &mut crate::globals::get_gfd_global_unchecked_mut().graphics }
    }
}

pub trait GraphicsState {
    fn has_flags(&self, flag: GraphicsFlags) -> bool;

    fn has_any_flag(&self, flag: GraphicsFlags) -> bool;

    fn get_flags(&self) -> GraphicsFlags;
    fn get_flags_mut(&mut self) -> &mut GraphicsFlags;
    /// Get a reference to the target scene graph from global state
    /// (Original function: gfdRenderGetScene)
    fn get_scene(&self, num: usize) -> Option<&Scene>;
    /// Get a mutable reference to the target scene graph from global state
    /// (Original function: gfdRenderGetScene)
    fn get_scene_mut(&mut self, num: usize) -> Option<&mut Scene>;
    /// Get a reference to the main camera for the target scene graph
    /// (Original function: gfdRenderGetSceneCamera)
    fn get_scene_camera(&self, num: usize) -> Option<&Camera>;
    /// Get a reference to the main camera for the target scene graph
    /// (Original function: gfdRenderGetSceneCamera)
    fn get_scene_camera_mut(&mut self, num: usize) -> Option<&mut Camera>;

    fn is_deferred_rendering_available(&self) -> bool { false }

    fn get_current_scene(&self) -> Option<&Scene> {
        self.get_scene(0)
    }

    fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.get_scene_mut(0)
    }

    fn get_current_scene_id(&self) -> usize;
    /*
    fn get_current_cmd_buffer(&mut self) -> Option<&mut CmdBuffer>;

    fn get_texture_head(&self) -> Option<&Texture>;
    fn lock_texture_mutex(&mut self) -> RecursiveMutexGuard<'_, *mut Texture>;
    fn lock_vertex_shader_mutex(&mut self) -> MutexGuard<'_, *mut VertexShader>;
    fn lock_pixel_shader_mutex(&mut self) -> MutexGuard<'_, *mut PixelShader>;
    fn lock_geometry_shader_mutex(&mut self) -> MutexGuard<'_, *mut GeometryShader>;
    fn lock_compute_shader_mutex(&mut self) -> MutexGuard<'_, *mut ComputeShader>;

    fn get_max_ot_priority(&self) -> usize;
    fn get_widget_ot_priority(&self) -> usize;
    fn get_debug_font_ot_priority(&self) -> usize;
    fn get_mouse_ot_priority(&self) -> usize;

    fn get_frame_id(&self) -> usize;

    fn get_vertex_shader(&self, index: usize) -> Option<&VertexShader>;
    fn get_pixel_shader(&self, index: usize) -> Option<&PixelShader>;
    fn get_geometry_cull(&mut self) -> Option<&mut CullObject>;
    fn get_ot_shadow_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList>;
    fn get_ot_render_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList>;
    fn get_ot_prepare_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList>;
    fn get_current_vertex_shader(&self) -> Option<&VertexShader>;
    fn get_current_pixel_shader(&self) -> Option<&PixelShader>;
    fn get_current_vertex_shader_ptr(&mut self) -> *mut *mut VertexShader;
    fn get_current_pixel_shader_ptr(&mut self) -> *mut *mut PixelShader;
    fn get_vertex_shader_platform(&self, index: usize) -> Option<&VertexShaderPlatform>;
    fn get_pixel_shader_platform(&self, index: usize) -> Option<&PixelShaderPlatform>;
    fn field44b8_clear(&mut self);
    */
}

#[repr(C)]
#[derive(Debug)]
pub struct GraphicsStateSteam {
    flags: GraphicsFlags,
    video: VideoMode,
    fps: u32,
    fvf: u32,
    pub(super) scene: [*mut Scene; SCENE_LISTS],
    pub(super) cmd_buffer: *mut u8,
    texture_head: *mut u8,
    texture_mutex: RecursiveMutex,
    config_button_resources: *mut u8,
    material_array_stack: [*mut ItemArray<usize>; MATERIAL_LISTS],
    material_array_count: u32,
    material_mutex: RecursiveMutex,
    shader_vtx_head: [*mut u8; SHADER_SOURCE],
    shader_vtx_mutex: RecursiveMutex,
    shader_frg_head: [*mut u8; SHADER_SOURCE],
    shader_frg_mutex: RecursiveMutex,
    shader_geo_head: *mut u8,
    shader_geo_mutex: RecursiveMutex,
    shader_cmp_head: *mut u8,
    shader_cmp_mutex: RecursiveMutex,
    fieldac0_head: *mut u8,
    fieldac0_mutex: RecursiveMutex,
    asset_head: *mut u8,
    asset_mutex: RecursiveMutex,
    asset_editor_mode: bool,
    pub(super) ot_shadow_list: [*mut u8; RENDER_LISTS],
    pub(super) ot_render_list: [*mut u8; RENDER_LISTS],
    pub(super) ot_prepare_list: [*mut u8; RENDER_LISTS],
    pub(super) ot_cull_object: *mut u8,
    pub(super) prio_max: u32,
    ot: *mut u8,
    prio_group_max: u32,
    pub(super) frame_id: u32,
    widget_prio: u32,
    dbg_fnt_prio: u32,
    mouse_prio: u32,
    resources: *mut u8,
    setup: *mut u8,
    render_info: [[*mut u8; 2]; 2],
    exist: [u32; 2],
    render_state_current: [usize; RENDER_STATES],
    render_state_stack: [[usize; 2]; RENDER_STATES],
    shader_source: [[*mut u8; 3]; SHADER_SOURCE],
    shader_vertex: [*mut u8; FIXED_VERTEX_SHADERS],
    shader_vertex_end: *mut u8,
    shader_fragment: [*mut u8; FIXED_PIXEL_SHADERS],
    shader_fragment_end: *mut u8,
    shader_geometry: *mut u8,
    shader_compute: [*mut u8; 7],
    shader_compute_end: *mut u8,
    current_shader_vertex: *mut u8,
    current_shader_fragment: *mut u8,
    current_shader_geometry: *mut u8,
    current_shader_compute: *mut u8,
    shader_id_flag_mask: [[u32; 3]; 3],
    field2334: [u8; 44],
    shader_cache_mutex: RecursiveMutex,
    field2388: *mut u8,
    effect_vertex_indices: [*mut u8; 6],
    light_placement: [*mut u8; 2],
    ascii_font_texture: *mut u8,
    shader_outline_texture: *mut u8,
    shader_noise_texture: *mut u8,
    shader_edge_dark_texture: *mut u8,
    environment: [u8; 0x280],
    image_path: [u8; 0x400],
    adjustment: Vec2,
    fullscreen_render_target: *mut u8,
    field2a80: [u8; 0x140]
}

impl GraphicsState for GraphicsStateSteam {
    fn has_flags(&self, flag: GraphicsFlags) -> bool {
        self.flags.contains(flag)
    }
    fn has_any_flag(&self, flag: GraphicsFlags) -> bool {
        self.flags.intersects(flag)
    }
    fn get_flags(&self) -> GraphicsFlags {
        self.flags
    }
    fn get_flags_mut(&mut self) -> &mut GraphicsFlags {
        &mut self.flags
    }
    fn get_scene(&self, num: usize) -> Option<&Scene> {
        unsafe { self.scene[num].as_ref() }
    }
    fn get_scene_mut(&mut self, num: usize) -> Option<&mut Scene> {
        unsafe { self.scene[num].as_mut() }
    }
    fn get_scene_camera(&self, num: usize) -> Option<&Camera> {
        match self.get_scene(num) {
            Some(v) => unsafe { v.camera.as_ref() },
            None => None
        }
    }
    fn get_scene_camera_mut(&mut self, num: usize) -> Option<&mut Camera> {
        match self.get_scene_mut(num) {
            Some(v) => unsafe { v.camera.as_mut() },
            None => None
        }
    }
    fn get_current_scene_id(&self) -> usize {
        self.frame_id as usize
    }
    /*
    fn get_current_cmd_buffer(&mut self) -> Option<&mut crate::graphics::render::cmd_buffer::CmdBuffer> {
        unsafe { self.cmd_buffer.as_mut() }
    }
    fn get_texture_head(&self) -> Option<&crate::graphics::texture::Texture> {
        unsafe { self.texture_head.as_ref() }
    }
    fn lock_texture_mutex(&mut self) -> RecursiveMutexGuard<'_, *mut crate::graphics::texture::Texture> {
        self.texture_mutex.lock(&mut self.texture_head)
    }
    */
}
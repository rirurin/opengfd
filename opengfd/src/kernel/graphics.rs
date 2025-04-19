use bitflags::bitflags;
use crate::{
    device::ngr::{
        allocator::AllocatorHook,
        renderer::shader::{
           ComputeShader,
           GeometryShader,
           PixelShader, 
           VertexShader,
           PixelShaderPlatform,
           VertexShaderPlatform
        }
    },
    graphics::{
        cull::CullObject,
        render::cmd_buffer::CmdBuffer,
        render_ot::{ RenderOtGroup, RenderOtList },
        resources::{ Resources, ResBuffer },
        scene::{ Scene, SceneLightPlacement },
        shader::shader::ShaderSource,
        texture::Texture
    },
    kernel::{
        asset::Asset,
        global_common::{
            RENDER_STATES,
            SCENE_LISTS,
            MATERIAL_LISTS,
            RENDER_LISTS,
            SHADER_SOURCE,
            FIXED_VERTEX_SHADERS,
            FIXED_PIXEL_SHADERS,
            FIXED_GEOMETRY_SHADERS,
            FIXED_COMPUTE_SHADERS,
            OT_GROUP_COUNT,
        },
        init::VideoMode,
    },
    object::{
        camera::Camera,
        mesh::Mesh
    },
    utility::{
        item_array::ItemArray,
        mutex::{ Mutex, RecursiveMutex },
        name::Name
    }
};
use glam::{ Vec3, Mat4 };

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GraphicsFlags : u32 {
        const ShadowCaster = 1 << 0;
        const Fog = 1 << 3;
        const HeightFog = 1 << 7;
        const HasInfiniteOcean = 1 << 15;
        const HasTemperare = 1 << 18;
    }
}

#[allow(dead_code)]
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

pub struct GraphicsGlobal;
impl GraphicsGlobal {
    pub fn get_gfd_graphics_global() -> &'static dyn GraphicsState {
        let glb = unsafe { &crate::globals::get_gfd_global_unchecked().graphics };
        if unsafe { *crate::globals::get_is_steam_unchecked() } { glb }
        else { unsafe { &*(&raw const *glb as *const GraphicsStateUWP) } }
    }
    pub fn get_gfd_graphics_global_mut() -> &'static mut dyn GraphicsState {
        let glb = unsafe { &mut crate::globals::get_gfd_global_unchecked_mut().graphics };
        if unsafe { *crate::globals::get_is_steam_unchecked() } { glb }
        else { unsafe { &mut *(&raw mut *glb as *mut GraphicsStateUWP) } }
    }
}

pub trait GraphicsState {
    fn has_flags(&self, flag: GraphicsFlags) -> bool;

    fn has_any_flag(&self, flag: GraphicsFlags) -> bool;
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
        self.get_scene(self.get_current_scene_id())
    }

    fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.get_scene_mut(self.get_current_scene_id())
    }

    fn get_current_scene_id(&self) -> usize;

    fn get_current_cmd_buffer(&mut self) -> Option<&mut CmdBuffer>;

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
}

#[repr(C)]
#[derive(Debug)]
pub struct GraphicsStateSteam {
    flags: GraphicsFlags,
    video_mode: VideoMode,
    fps: u32, // always 30
    fvf: u32,
    pub(super) scene: [*mut Scene; SCENE_LISTS],
    pub(super) cmd_buffer: *mut CmdBuffer,
    texture_head: *mut Texture,
    texture_mutex: RecursiveMutex,
    material_array_stack: [*mut ItemArray<usize>; MATERIAL_LISTS],
    material_array_count: u32,
    material_mutex: RecursiveMutex,
    shader_vtx_head: *mut VertexShader,
    shader_vtx_mutex: Mutex,
    shader_frg_head: *mut PixelShader,
    shader_frg_mutex: Mutex,
    shader_geo_head: *mut u8,
    shader_geo_mutex: Mutex,
    shader_cmp_head: *mut u8,
    shader_cmp_mutex: Mutex,
    asset_head: *mut Asset,
    asset_mutex: Mutex,
    asset_editor_mode: bool,
    pub(super) ot_shadow_list: [*mut RenderOtList; RENDER_LISTS],
    pub(super) ot_render_list: [*mut RenderOtList; RENDER_LISTS],
    pub(super) ot_prepare_list: [*mut RenderOtList; RENDER_LISTS],
    pub(super) ot_cull_object: *mut CullObject,
    pub(super) prio_max: u32,
    ot: *mut [RenderOtGroup; OT_GROUP_COUNT],
    prio_group_max: u32,
    pub(super) frame_id: u32,
    widget_prio: u32,
    dbg_fnt_prio: u32,
    mouse_prio: u32,
    resources: *mut Resources,
    setup: *mut ResBuffer,
    info: [GraphicsCBuffer; 3],
    unk: [u8; 4352],
    field2390: *const u8,
    field2398: *const u8,
    pub render_state_current: [usize; RENDER_STATES],
    pub render_state_stack: [[usize; 2]; RENDER_STATES],
    shader_source: [[*mut ShaderSource; RENDER_LISTS]; SHADER_SOURCE],
    // split after here
    pub shader_vertex: [*mut VertexShader; FIXED_VERTEX_SHADERS],
    pub shader_pixel: [*mut PixelShader; FIXED_PIXEL_SHADERS],
    pub shader_geometry: [*mut u8; FIXED_GEOMETRY_SHADERS],
    pub shader_compute: [*mut u8; FIXED_COMPUTE_SHADERS],
    pub shader_current_vertex: *mut VertexShader,
    pub shader_current_fragment: *mut PixelShader,
    pub shader_current_geometry: *mut GeometryShader,
    pub shader_current_compute: *mut ComputeShader,
    // split before here
    pub(crate) field44b8: *mut u8,
    pub(crate) field44c0: *mut u8, 
    field44c8: usize,
    field44d0: usize,
    shader_hash_vertex: [u32; 3],
    shader_hash_pixel: [u32; 3],
    shader_hash_geometry: [u32; 3],
    shader_hash_4: [u32; 3], // shaderCacheStream
    shader_hash_5: [u32; 3], // shaderCacheMutex
    effect_vertex_indices: [*mut u8; 6],
    light_placement: [*mut SceneLightPlacement; 3],
    shader_outline_texture: *mut Texture,
    shader_noise_texture: *mut Texture,
    shader_edge_dark_texture: *mut Texture,
    texture_4558: *mut Texture,
    unk1: [u8; 0x10d8],
    widget_surface: *mut u8,
    widget_ref: *mut u8,
    swap_cb: *mut u8,
    swap_cb_data: *mut u8,
    sphere_mesh: *mut Mesh,
    hemisphere_mesh: *mut Mesh,
    unk3: [u8; 0x60],
    pub(super) current_scene: u32,
    unk2: [u8; 0xc],
    hdr_filename: Name<AllocatorHook>,
    ibl_filename: Name<AllocatorHook>,
    lut_filename: Name<AllocatorHook>,
    env_toon_filename: Name<AllocatorHook>,
    skybox_filename: Name<AllocatorHook>,
    infinite_ocean_filename: Name<AllocatorHook>,
    env_field_784: f32,
    env_field_788: u8,
    scene_ambient_toon_r: f32,
    scene_ambient_toon_g: f32,
    field5798: f32,
    field579c: f32,
    field57a0: [u32; 8],
}

impl GraphicsState for GraphicsStateSteam {
    fn has_flags(&self, flag: GraphicsFlags) -> bool {
        self.flags.contains(flag)
    }
    fn has_any_flag(&self, flag: GraphicsFlags) -> bool {
        self.flags.intersects(flag)
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
        self.current_scene as usize
    }
    fn get_current_cmd_buffer(&mut self) -> Option<&mut CmdBuffer> {
        unsafe { self.cmd_buffer.as_mut() }
    }
    fn get_frame_id(&self) -> usize {
        self.frame_id as usize
    }
    fn get_vertex_shader(&self, index: usize) -> Option<&VertexShader> {
        unsafe { self.shader_vertex.get_unchecked(index).as_ref() }
    }
    fn get_pixel_shader(&self, index: usize) -> Option<&PixelShader> {
        unsafe { self.shader_pixel.get_unchecked(index).as_ref() }
    }
    fn get_geometry_cull(&mut self) -> Option<&mut CullObject> {
        unsafe { self.ot_cull_object.as_mut() }
    }
    fn get_ot_shadow_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList> {
        unsafe { (*self.ot_shadow_list.get_unchecked(id)).add(prio).as_mut() }
    }
    fn get_ot_render_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList> {
        unsafe { (*self.ot_render_list.get_unchecked(id)).add(prio).as_mut() }
    }
    fn get_ot_prepare_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList> {
        unsafe { (*self.ot_prepare_list.get_unchecked(id)).add(prio).as_mut() }
    }
    fn get_current_vertex_shader(&self) -> Option<&VertexShader> {
        unsafe { self.shader_current_vertex.as_ref() }
    }
    fn get_current_pixel_shader(&self) -> Option<&PixelShader> {
        unsafe { self.shader_current_fragment.as_ref() }
    }
    fn get_current_vertex_shader_ptr(&mut self) -> *mut *mut VertexShader {
        &raw mut self.shader_current_vertex
    }
    fn get_current_pixel_shader_ptr(&mut self) -> *mut *mut PixelShader {
        &raw mut self.shader_current_fragment
    }
    fn get_vertex_shader_platform(&self, index: usize) -> Option<&VertexShaderPlatform> {
        unsafe { (&**self.shader_vertex.get_unchecked(index)).data.as_ref() }
    }
    fn get_pixel_shader_platform(&self, index: usize) -> Option<&PixelShaderPlatform> {
        unsafe { (&**self.shader_pixel.get_unchecked(index)).data.as_ref() }
    }
    fn field44b8_clear(&mut self) {
        self.field44b8 = std::ptr::null_mut();
        self.field44c0 = std::ptr::null_mut();
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GraphicsStateUWP {
    flags: GraphicsFlags,
    video_mode: VideoMode,
    fps: u32, // always 30
    fvf: u32,
    pub(super) scene: [*mut Scene; SCENE_LISTS],
    pub(super) cmd_buffer: *mut CmdBuffer,
    texture_head: *mut Texture,
    texture_mutex: RecursiveMutex,
    material_array_stack: [*mut ItemArray<usize>; MATERIAL_LISTS],
    material_array_count: u32,
    material_mutex: RecursiveMutex,
    shader_vtx_head: *mut VertexShader,
    shader_vtx_mutex: Mutex,
    shader_frg_head: *mut PixelShader,
    shader_frg_mutex: Mutex,
    shader_geo_head: *mut u8,
    shader_geo_mutex: Mutex,
    shader_cmp_head: *mut u8,
    shader_cmp_mutex: Mutex,
    asset_head: *mut Asset,
    asset_mutex: Mutex,
    asset_editor_mode: bool,
    pub(super) ot_shadow_list: [*mut RenderOtList; RENDER_LISTS],
    pub(super) ot_render_list: [*mut RenderOtList; RENDER_LISTS],
    pub(super) ot_prepare_list: [*mut RenderOtList; RENDER_LISTS],
    pub(super) ot_cull_object: *mut CullObject,
    pub(super) prio_max: u32,
    ot: *mut [RenderOtGroup; OT_GROUP_COUNT],
    prio_group_max: u32,
    pub(super) frame_id: u32,
    widget_prio: u32,
    dbg_fnt_prio: u32,
    mouse_prio: u32,
    resources: *mut Resources,
    setup: *mut ResBuffer,
    info: [GraphicsCBuffer; 3],
    unk: [u8; 4352],
    field2390: *const u8,
    field2398: *const u8,
    pub render_state_current: [usize; RENDER_STATES],
    pub render_state_stack: [[usize; 2]; RENDER_STATES],
    shader_source: [[*mut ShaderSource; RENDER_LISTS]; SHADER_SOURCE],
    pub shader_vertex: [*mut VertexShader; FIXED_VERTEX_SHADERS],
    pub shader_pixel: [*mut PixelShader; FIXED_PIXEL_SHADERS],
    pub shader_geometry: [*mut u8; FIXED_GEOMETRY_SHADERS],
    pub shader_compute: [*mut u8; FIXED_COMPUTE_SHADERS],
    pub shader_current_vertex: *mut VertexShader,
    pub shader_current_fragment: *mut PixelShader,
    pub shader_current_geometry: *mut GeometryShader,
    pub shader_current_compute: *mut ComputeShader,
    field44b8_filler: [u8; 0x20], // deal with difference in struct size
    pub(crate) field44b8: *mut u8,
    pub(crate) field44c0: *mut u8, 
    field44c8: usize,
    field44d0: usize,
    shader_hash_vertex: [u32; 3],
    shader_hash_pixel: [u32; 3],
    shader_hash_geometry: [u32; 3],
    shader_hash_4: [u32; 3], // shaderCacheStream
    shader_hash_5: [u32; 3], // shaderCacheMutex
    effect_vertex_indices: [*mut u8; 6],
    light_placement: [*mut SceneLightPlacement; 3],
    shader_outline_texture: *mut Texture,
    shader_noise_texture: *mut Texture,
    shader_edge_dark_texture: *mut Texture,
    texture_4558: *mut Texture,
    unk1: [u8; 0x10d8],
    widget_surface: *mut u8,
    widget_ref: *mut u8,
    swap_cb: *mut u8,
    swap_cb_data: *mut u8,
    sphere_mesh: *mut Mesh,
    hemisphere_mesh: *mut Mesh,
    unk3: [u8; 0x60],
    pub(super) current_scene: u32,
    unk2: [u8; 0xc],
    hdr_filename: Name<AllocatorHook>,
    ibl_filename: Name<AllocatorHook>,
    lut_filename: Name<AllocatorHook>,
    env_toon_filename: Name<AllocatorHook>,
    skybox_filename: Name<AllocatorHook>,
    infinite_ocean_filename: Name<AllocatorHook>,
    env_field_784: f32,
    env_field_788: u8,
    scene_ambient_toon_r: f32,
    scene_ambient_toon_g: f32,
    field5798: f32,
    field579c: f32,
    field57a0: [u32; 8],
    field57c0: u64
}

impl GraphicsState for GraphicsStateUWP {
    fn has_flags(&self, flag: GraphicsFlags) -> bool {
        self.flags.contains(flag)
    }
    fn has_any_flag(&self, flag: GraphicsFlags) -> bool {
        self.flags.intersects(flag)
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
        self.current_scene as usize
    }
    fn get_current_cmd_buffer(&mut self) -> Option<&mut CmdBuffer> {
        unsafe { self.cmd_buffer.as_mut() }
    }
    fn get_frame_id(&self) -> usize {
        self.frame_id as usize
    }
    fn get_vertex_shader(&self, index: usize) -> Option<&VertexShader> {
        unsafe { self.shader_vertex.get_unchecked(index).as_ref() }
    }
    fn get_pixel_shader(&self, index: usize) -> Option<&PixelShader> {
        unsafe { self.shader_pixel.get_unchecked(index).as_ref() }
    }
    fn get_geometry_cull(&mut self) -> Option<&mut CullObject> {
        unsafe { self.ot_cull_object.as_mut() }
    }
    fn get_ot_shadow_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList> {
        unsafe { (*self.ot_shadow_list.get_unchecked(id)).add(prio).as_mut() }
    }
    fn get_ot_render_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList> {
        unsafe { (*self.ot_render_list.get_unchecked(id)).add(prio).as_mut() }
    }
    fn get_ot_prepare_list(&self, id: usize, prio: usize) -> Option<&mut RenderOtList> {
        unsafe { (*self.ot_prepare_list.get_unchecked(id)).add(prio).as_mut() }
    }
    fn get_current_vertex_shader(&self) -> Option<&VertexShader> {
        unsafe { self.shader_current_vertex.as_ref() }
    }
    fn get_current_pixel_shader(&self) -> Option<&PixelShader> {
        unsafe { self.shader_current_fragment.as_ref() }
    }
    fn get_current_vertex_shader_ptr(&mut self) -> *mut *mut VertexShader {
        &raw mut self.shader_current_vertex
    }
    fn get_current_pixel_shader_ptr(&mut self) -> *mut *mut PixelShader {
        &raw mut self.shader_current_fragment
    }
    fn get_vertex_shader_platform(&self, index: usize) -> Option<&VertexShaderPlatform> {
        unsafe { (&**self.shader_vertex.get_unchecked(index)).data.as_ref() }
    }
    fn get_pixel_shader_platform(&self, index: usize) -> Option<&PixelShaderPlatform> {
        unsafe { (&**self.shader_pixel.get_unchecked(index)).data.as_ref() }
    }
    fn field44b8_clear(&mut self) {
        self.field44b8 = std::ptr::null_mut();
        self.field44c0 = std::ptr::null_mut();
    }
}
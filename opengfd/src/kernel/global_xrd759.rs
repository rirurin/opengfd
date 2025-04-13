#![allow(unused_imports, dead_code)]
use bitflags::bitflags;
use crate::{
    device::ngr::{
        allocator::AllocatorHook,
        renderer::shader
    },
    graphics::{
        cull::CullObject,
        environment,
        render::cmd_buffer::CmdBuffer,
        render_ot::{ RenderOtGroup, RenderOtList },
        resources::{ Resources, ResBuffer },
        scene::{ Scene, SceneLightPlacement },
        shader::shader::ShaderSource,
        texture::Texture
    },
    kernel::{
        allocator::GfdAllocator,
        asset::Asset,
        chip::Chip,
        global_common::RENDER_STATES,
        init::VideoMode,
        task::{
            Task as GfdTask,
            TaskList
        }
    },
    object::mesh::Mesh,
    utility::{
        free_list::FreeList as GfdFreeList,
        item_array::ItemArray,
        math::RandomAligned,
        misc::Range,
        mutex::{ Mutex, RecursiveMutex },
        name::Name
    }
};
use glam::{ Vec3, Vec3A, Mat4 };
use riri_mod_tools_proc::ensure_layout;
use riri_mod_tools_rt::address::ProcessInfo;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GlobalFlags: u32 {
        const NO_INCREASE_TASK_START_TIMER = 1 << 0;
        const RENDER_TASK_RUNNING = 1 << 1;
        /// True if _pre_<GFD> or _post_<GFD> are executing
        const SYSTEM_TASK_RUNNING = 1 << 2;
        const RESIZE_FLAG0 = 1 << 3;
        const RESIZE_FLAG1 = 1 << 4;
    }
}

pub trait GlobalImpl {
    fn get_free_list_mutex(&mut self) -> &mut Mutex;
    fn get_free_list_head(&self) -> Option<&GfdFreeList>;
    fn get_free_list_head_mut(&self) -> Option<&mut GfdFreeList>;
    fn get_free_list_head_ptr(&self) -> *mut GfdFreeList;
    fn set_free_list_head_mut(&mut self, new: *mut GfdFreeList);
    fn get_uid(&mut self);
    unsafe fn get_task_free_list_unchecked_mut(&mut self) -> &mut GfdFreeList<GfdTask, GfdAllocator>;
    fn get_flags(&self) -> GlobalFlags;
    fn get_tasks(&self) -> &TaskGlobal;
    fn get_tasks_mut(&mut self) -> &mut TaskGlobal;
    fn get_chip_free_list(&self) -> Option<&GfdFreeList<Chip, GfdAllocator>>;
    fn get_chip_free_list_mut(&self) -> Option<&mut GfdFreeList<Chip, GfdAllocator>>;
}

#[repr(C)]
#[derive(Debug)]
pub struct GlobalUWP {
    flags: GlobalFlags,
    elapsed_time: f32,
    delta_time: f32,
    loop_counter: i32,
    field10: i32,
    field14: i32,
    pub graphics: GraphicsGlobal,
    pub physics: PhysicsGlobal,
    pub tasks: TaskGlobal,
    pub random: RandomAligned,
    field58c0: *mut u8,
    field58c8: *mut u8,
    field58d0: *mut u8,
    field58d8: *mut u8,
    current_dir: *mut u8,
    init_path: *mut u8,
    field58f0: *mut u8,
    field58f0_count: u32,
    dev_file_head: *mut u8,
    dev_file_mutex: Mutex,
    free_list_head: *mut GfdFreeList,
    free_list_mutex: Mutex, 
    chip_free_list: *mut GfdFreeList<Chip, GfdAllocator>,
    task_free_list: *mut GfdFreeList<GfdTask, GfdAllocator>,
    task_free_list_entries_per_block: u32,
    // TODO: check node list is in right position
    node_free_list: *mut GfdFreeList,
    node_free_list_entries_per_block: u32,
    delayed_proc_item_head: *mut u8,
    delayed_proc_item_tail: *mut u8,
    delayed_proc_item_mutex: RecursiveMutex,
    field5980: [u8; 0x18],
    delay_frame: u32,
    delay_force_frame: u32,
    main_stack_size: u32,
    field59a4: [u8; 0x74],
    uid: u64
}


#[repr(C)]
#[derive(Debug)]
pub struct Global {
    flags: GlobalFlags,
    elapsed_time: f32,
    delta_time: f32,
    loop_counter: i32,
    field10: i32,
    field14: i32,
    pub graphics: GraphicsGlobal,
    pub physics: PhysicsGlobal,
    pub tasks: TaskGlobal,
    pub random: RandomAligned,
    controller_callback: *mut u8, // callback for controller
    controller_arg: *mut u8,
    field58d0: *mut u8,
    field58d8: *mut u8,
    current_dir: *mut u8,
    init_path: *mut u8,
    field58f0: *mut u8,
    field58f0_count: u32,
    dev_file_head: *mut u8,
    dev_file_mutex: Mutex,
    free_list_head: *mut GfdFreeList,
    free_list_mutex: Mutex, 
    chip_free_list: *mut GfdFreeList<Chip, GfdAllocator>,
    task_free_list: *mut GfdFreeList<GfdTask, GfdAllocator>,
    task_free_list_entries_per_block: u32,
    // TODO: check node list is in right position
    node_free_list: *mut GfdFreeList,
    node_free_list_entries_per_block: u32,
    delayed_proc_item_head: *mut u8,
    delayed_proc_item_tail: *mut u8,
    delayed_proc_item_mutex: RecursiveMutex,
    field5980: [u8; 0x18],
    delay_frame: u32,
    delay_force_frame: u32,
    main_stack_size: u32,
    field59a4: [u8; 0x74],
    uid: u64
}

impl Global {
    pub fn get_free_list_mutex(&mut self) -> &mut Mutex {
        &mut self.free_list_mutex
    }
    pub fn get_free_list_head(&self) -> Option<&GfdFreeList> {
        if self.free_list_head.is_null() { None }
        else { Some(unsafe { &*self.free_list_head } )}
    }
    pub fn get_free_list_head_mut(&self) -> Option<&mut GfdFreeList> {
        if self.free_list_head.is_null() { None }
        else { Some(unsafe { &mut *self.free_list_head } )}
    }
    pub fn get_free_list_head_ptr(&self) -> *mut GfdFreeList {
        &raw mut *self.free_list_head
    }
    pub fn set_free_list_head_mut(&mut self, new: *mut GfdFreeList) {
        self.free_list_head = new;
    }
    /// Original function: gfdGetUID
    pub fn get_uid(&mut self) -> u64 {
        // self.uid.wrapping_add(1).max(1)
        let process = ProcessInfo::get_current_process().unwrap();
        match process.get_executable_hash() {
            0x51d3b74ca903fd98 => { // hack for UWP since it's gfdGlobal is slightly larger (+ 0x28)
                let ptr = unsafe { (&raw mut *self as *mut u8).add(0x5a40) as *mut u64 };
                unsafe { (*ptr).wrapping_add(1).max(1) }
            },
            _ => self.uid.wrapping_add(1).max(1)
        }
    }
    pub unsafe fn get_task_free_list_unchecked_mut(&mut self) 
    -> &mut GfdFreeList<GfdTask, GfdAllocator> {
        let process = ProcessInfo::get_current_process().unwrap();
        match process.get_executable_hash() {
            0x51d3b74ca903fd98 => { // hack for UWP since it's gfdGlobal is slightly larger (+ 0x28)
                &mut **((&raw mut *self as *mut u8).add(0x5950) as *mut *mut GfdFreeList<GfdTask, GfdAllocator>)
            },
            _ => &mut *self.task_free_list
        }
    }

    pub fn get_flags(&self) -> GlobalFlags {
        self.flags
    }
    pub fn get_tasks(&self) -> &TaskGlobal {
        let process = ProcessInfo::get_current_process().unwrap();
        match process.get_executable_hash() {
            0x51d3b74ca903fd98 => { // hack for UWP since it's gfdGlobal is slightly larger (+ 0x28)
                unsafe { &*((&raw const *self as *const u8).add(0x5850) as *const TaskGlobal) }
            },
            _ => &self.tasks
        }
    }
    pub fn get_tasks_mut(&mut self) -> &mut TaskGlobal {
        let process = ProcessInfo::get_current_process().unwrap();
        match process.get_executable_hash() {
            0x51d3b74ca903fd98 => { // hack for UWP since it's gfdGlobal is slightly larger (+ 0x28)
                unsafe { &mut *((&raw mut *self as *mut u8).add(0x5850) as *mut TaskGlobal) }
            },
            _ => &mut self.tasks
        }
    }
    pub fn get_chip_free_list(&self) -> Option<&GfdFreeList<Chip, GfdAllocator>> {
        unsafe { self.chip_free_list.as_ref() }
    }
    pub fn get_chip_free_list_mut(&self) -> Option<&mut GfdFreeList<Chip, GfdAllocator>> {
        unsafe { self.chip_free_list.as_mut() }
    }
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
        const ShadowCaster = 1 << 0;
        const Fog = 1 << 3;
        const HeightFog = 1 << 7;
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
    video_mode: VideoMode,
    fps: u32,
    fvf: u32,
    pub(super) scene: [*mut Scene; 2],
    pub(super) cmd_buffer: *mut CmdBuffer,
    texture_head: *mut Texture,
    texture_mutex: RecursiveMutex,
    material_array_stack: [*mut ItemArray<usize>; 8],
    material_array_count: u32,
    material_mutex: RecursiveMutex,
    shader_vtx_head: *mut shader::VertexShader,
    shader_vtx_mutex: Mutex,
    shader_frg_head: *mut shader::PixelShader,
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
    ot: *mut [RenderOtGroup; 7],
    prio_group_max: u32,
    pub(super) frame_id: u32,
    widget_prio: u32,
    dbg_fnt_prio: u32,
    mouse_prio: u32,
    resources: *mut Resources,
    setup: *mut ResBuffer,
    info: [GraphicsCBuffer; 3],
    unk: [u8; 4368],
    pub render_state_current: [usize; RENDER_STATES],
    pub render_state_stack: [[usize; 2]; RENDER_STATES],
    shader_source: [[*mut ShaderSource; RENDER_LISTS]; SHADER_SOURCE],
    pub shader_vertex: [*mut shader::VertexShader; FIXED_VERTEX_SHADERS],
    pub shader_pixel: [*mut shader::PixelShader; FIXED_PIXEL_SHADERS],
    pub shader_geometry: [*mut u8; FIXED_GEOMETRY_SHADERS],
    pub shader_compute: [*mut u8; FIXED_COMPUTE_SHADERS],
    pub shader_current_vertex: *mut shader::VertexShader,
    pub shader_current_fragment: *mut shader::PixelShader,
    pub shader_current_geometry: *mut shader::GeometryShader,
    pub shader_current_compute: *mut shader::ComputeShader,
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

impl GraphicsGlobal {
    pub fn has_flags(&self, flag: GraphicsFlags) -> bool {
        self.flags.contains(flag)
    }
    pub fn has_any_flag(&self, flag: GraphicsFlags) -> bool {
        self.flags.intersects(flag)
    }
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
    delta: f32,
    delta2: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicsGlobal {
    data: [u8; 0x50]
    // gravity: Vec3A,
    // wind: PhysicsGlobalWind
}

// PHYSICS END

// TASK START

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TaskFlag : u32 {
        const PRE_GFD = 1 << 2;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TaskGlobal {
    pub flag: TaskFlag,
    pub begin: TaskList<GfdAllocator>,
    pub update: TaskList<GfdAllocator>,
    pub render: TaskList<GfdAllocator>,
    pub end: TaskList<GfdAllocator>,
    pub release: TaskList<GfdAllocator>,
    pub task_count: u32,
    pub pad: TaskList<GfdAllocator>,
    pub detach_mutex: *mut GfdTask<GfdAllocator>,
    pub current: *mut GfdTask<GfdAllocator>,
    pub mutex: Mutex,
}

impl TaskGlobal {
    pub fn get_first_begin_task(&self) -> *const GfdTask<GfdAllocator> {
        self.begin.get_head_ptr()
    }
    pub fn get_first_begin_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.begin.get_head_ptr()
    }
    pub fn get_first_update_task(&self) -> *const GfdTask<GfdAllocator> {
        self.update.get_head_ptr()
    }
    pub fn get_first_update_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.update.get_head_ptr()
    }
    pub fn get_first_render_task(&self) -> *const GfdTask<GfdAllocator> {
        self.render.get_head_ptr()
    }
    pub fn get_first_render_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.render.get_head_ptr()
    }
    pub fn get_first_ending_task(&self) -> *const GfdTask<GfdAllocator> {
        self.end.get_head_ptr()
    }
    pub fn get_first_ending_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.end.get_head_ptr()
    }

    pub fn get_last_begin_task(&self) -> *const GfdTask<GfdAllocator> {
        self.begin.get_tail_ptr()
    }
    pub fn get_last_begin_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.begin.get_tail_ptr()
    }
    pub fn get_last_update_task(&self) -> *const GfdTask<GfdAllocator> {
        self.update.get_tail_ptr()
    }
    pub fn get_last_update_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.update.get_tail_ptr()
    }
    pub fn get_last_render_task(&self) -> *const GfdTask<GfdAllocator> {
        self.render.get_tail_ptr()
    }
    pub fn get_last_render_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.render.get_tail_ptr()
    }
    pub fn get_last_ending_task(&self) -> *const GfdTask<GfdAllocator> {
        self.end.get_tail_ptr()
    }
    pub fn get_last_ending_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.end.get_tail_ptr()
    }

    pub fn set_first_begin_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.begin.set_head(val);
    }
    pub fn set_first_update_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.update.set_head(val);
    }
    pub fn set_first_render_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.render.set_head(val);
    }
    pub fn set_first_ending_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.end.set_head(val);
    }
    pub fn set_last_begin_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.begin.set_tail(val);
    }
    pub fn set_last_update_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.update.set_tail(val);
    }
    pub fn set_last_render_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.render.set_tail(val);
    }
    pub fn set_last_ending_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.end.set_tail(val);
    }
}

// TASK END

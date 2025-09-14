#![allow(unused_imports, dead_code)]
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
        asset::Asset,
        chip::Chip,
        graphics::{
            GraphicsState,
            GraphicsStateSteam,
            GraphicsStateUWP
        },
        global_impl::GlobalImpl,
        init::VideoMode,
    },
    platform::utils::PlatformInfo,
    object::mesh::Mesh,
    utility::{
        free_list::FreeList as GfdFreeList,
        item_array::ItemArray,
        math::RandomUnaligned,
        misc::Range,
        name::Name
    }
};
use glam::{ Vec3, Vec3A, Mat4 };
use riri_mod_tools_proc::ensure_layout;
use riri_mod_tools_rt::address::ProcessInfo;

include!("global_common.rs");

pub(crate) const RENDER_STATES: usize = 33;
pub(crate) const SCENE_LISTS: usize = 2;
pub(crate) const MATERIAL_LISTS: usize = 8;
pub(crate) const RENDER_LISTS: usize = 3;
// pub(crate) const SHADER_SOURCE: usize = 219; // (before 1.0.13)
pub(crate) const SHADER_SOURCE: usize = 221;
pub(crate) const FIXED_VERTEX_SHADERS: usize = 70;
pub(crate) const FIXED_PIXEL_SHADERS: usize = 198; // (before 1.0.13)
// pub(crate) const FIXED_PIXEL_SHADERS: usize = 200;
pub(crate) const FIXED_GEOMETRY_SHADERS: usize = 1;
pub(crate) const FIXED_COMPUTE_SHADERS: usize = 30;
pub(crate) const OT_GROUP_COUNT: usize = 7;

#[repr(C)]
#[derive(Debug)]
pub struct Global { // (GlobalSteam, default state)
    flags: GlobalFlags,
    elapsed_time: f32,
    delta_time: f32,
    loop_counter: i32,
    field10: i32,
    field14: i32,
    pub(crate) graphics: GraphicsStateSteam,
    physics: PhysicsGlobal,
    tasks: TaskGlobal,
    random: RandomUnaligned,
    controller_callback: *mut u8, // callback for controller
    controller_arg: *mut u8,
    field58d0: *mut u8,
    field58d8: *mut u8,
    // field58e0: *mut u8,
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

impl GlobalImpl for Global {
    fn get_flags(&self) -> GlobalFlags { self.flags }
    fn get_tasks(&self) -> &TaskGlobal { &self.tasks }
    fn get_tasks_mut(&mut self) -> &mut TaskGlobal { &mut self.tasks }
    fn get_random_mut(&mut self) -> &mut RandomUnaligned { &mut self.random }
    fn get_uid(&mut self) -> u64 {
        self.uid = self.uid.wrapping_add(1).max(1);
        self.uid
    }
    fn get_task_free_list_unchecked_mut(&mut self) 
    -> &mut GfdFreeList<GfdTask, GfdAllocator> {
        unsafe { &mut *self.task_free_list }
    }
    fn get_free_list_mutex(&mut self) -> &mut Mutex {
        &mut self.free_list_mutex
    }
    fn get_free_list_head(&self) -> Option<&GfdFreeList> {
        if self.free_list_head.is_null() { None }
        else { Some(unsafe { &*self.free_list_head } )}
    }
    fn get_free_list_head_mut(&self) -> Option<&mut GfdFreeList> {
        if self.free_list_head.is_null() { None }
        else { Some(unsafe { &mut *self.free_list_head } )}
    }
    fn get_free_list_head_ptr(&self) -> *mut GfdFreeList {
        unsafe { &raw mut *self.free_list_head }
    }
    fn set_free_list_head_mut(&mut self, new: *mut GfdFreeList) {
        self.free_list_head = new;
    }
    fn get_chip_free_list(&self) -> Option<&GfdFreeList<Chip, GfdAllocator>> {
        unsafe { self.chip_free_list.as_ref() }
    }
    fn get_chip_free_list_mut(&self) -> Option<&mut GfdFreeList<Chip, GfdAllocator>> {
        unsafe { self.chip_free_list.as_mut() }
    }
    fn get_task_free_list(&self) -> Option<&GfdFreeList<GfdTask, GfdAllocator>> {
        unsafe { self.task_free_list.as_ref() }
    }
    fn get_task_free_list_mut(&self) -> Option<&mut GfdFreeList<GfdTask, GfdAllocator>> {
        unsafe { self.task_free_list.as_mut() }
    }
}

/*
impl Global {
    pub fn new()
}
*/

#[repr(C)]
#[derive(Debug)]
pub struct GlobalUWP {
    flags: GlobalFlags,
    elapsed_time: f32,
    delta_time: f32,
    loop_counter: i32,
    field10: i32,
    field14: i32,
    graphics: GraphicsStateUWP,
    physics: PhysicsGlobal,
    tasks: TaskGlobal,
    random: RandomUnaligned,
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

impl GlobalImpl for GlobalUWP {
    fn get_flags(&self) -> GlobalFlags { self.flags }
    fn get_tasks(&self) -> &TaskGlobal { &self.tasks }
    fn get_tasks_mut(&mut self) -> &mut TaskGlobal { &mut self.tasks }
    fn get_random_mut(&mut self) -> &mut RandomUnaligned { &mut self.random }
    fn get_uid(&mut self) -> u64 {
        self.uid = self.uid.wrapping_add(1).max(1);
        self.uid
    }
    fn get_task_free_list_unchecked_mut(&mut self) 
    -> &mut GfdFreeList<GfdTask, GfdAllocator> {
        unsafe { &mut *self.task_free_list }
    }
    fn get_free_list_mutex(&mut self) -> &mut Mutex {
        &mut self.free_list_mutex
    }
    fn get_free_list_head(&self) -> Option<&GfdFreeList> {
        if self.free_list_head.is_null() { None }
        else { Some(unsafe { &*self.free_list_head } )}
    }
    fn get_free_list_head_mut(&self) -> Option<&mut GfdFreeList> {
        if self.free_list_head.is_null() { None }
        else { Some(unsafe { &mut *self.free_list_head } )}
    }
    fn get_free_list_head_ptr(&self) -> *mut GfdFreeList {
        &raw mut *self.free_list_head
    }
    fn set_free_list_head_mut(&mut self, new: *mut GfdFreeList) {
        self.free_list_head = new;
    }
    fn get_chip_free_list(&self) -> Option<&GfdFreeList<Chip, GfdAllocator>> {
        unsafe { self.chip_free_list.as_ref() }
    }
    fn get_chip_free_list_mut(&self) -> Option<&mut GfdFreeList<Chip, GfdAllocator>> {
        unsafe { self.chip_free_list.as_mut() }
    }
    fn get_task_free_list(&self) -> Option<&GfdFreeList<GfdTask, GfdAllocator>> {
        unsafe { self.task_free_list.as_ref() }
    }
    fn get_task_free_list_mut(&self) -> Option<&mut GfdFreeList<GfdTask, GfdAllocator>> {
        unsafe { self.task_free_list.as_mut() }
    }
}

impl Global {
    pub fn gfd_global_exists() -> bool {
        unsafe { crate::globals::get_gfd_global().is_some() }
    }

    pub fn get_gfd_global() -> &'static dyn GlobalImpl {
        let glb = unsafe { crate::globals::get_gfd_global_unchecked() };
        if unsafe { *crate::globals::get_is_steam_unchecked() } { glb }
        else { unsafe { &*(&raw const *glb as *const GlobalUWP) } }
    }
    pub fn get_gfd_global_mut() -> &'static mut dyn GlobalImpl {
        let glb = unsafe { crate::globals::get_gfd_global_unchecked_mut() };
        if unsafe { *crate::globals::get_is_steam_unchecked() } { glb }
        else { unsafe { &mut *(&raw mut *glb as *mut GlobalUWP) } }
    }
}

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
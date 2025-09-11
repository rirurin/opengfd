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
        graphics::{
            GraphicsState,
            GraphicsStateSteam,
            GraphicsStateUWP
        },
        init::VideoMode,
        task::{
            Task as GfdTask,
            TaskList
        }
    },
    platform::utils::PlatformInfo,
    object::mesh::Mesh,
    utility::{
        free_list::FreeList as GfdFreeList,
        item_array::ItemArray,
        math::RandomUnaligned,
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
        const Flag5 = 1 << 5;
        const Flag6 = 1 << 6;
        const Flag7 = 1 << 7;
        const Flag8 = 1 << 8;
        const Flag9 = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const Flag15 = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const Flag18 = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}

pub trait GlobalImpl {
    fn get_flags(&self) -> GlobalFlags;
    fn get_tasks(&self) -> &TaskGlobal;
    fn get_tasks_mut(&mut self) -> &mut TaskGlobal;
    fn get_random_mut(&mut self) -> &mut RandomUnaligned;
    /// Original function: gfdGetUID
    fn get_uid(&mut self) -> u64;
    fn get_task_free_list_unchecked_mut(&mut self) 
    -> &mut GfdFreeList<GfdTask, GfdAllocator>;
    fn get_free_list_mutex(&mut self) -> &mut Mutex;
    fn get_free_list_head(&self) -> Option<&GfdFreeList>;
    fn get_free_list_head_mut(&self) -> Option<&mut GfdFreeList>;
    fn get_free_list_head_ptr(&self) -> *mut GfdFreeList;
    fn set_free_list_head_mut(&mut self, new: *mut GfdFreeList);
    fn get_chip_free_list(&self) -> Option<&GfdFreeList<Chip, GfdAllocator>>;
    fn get_chip_free_list_mut(&self) -> Option<&mut GfdFreeList<Chip, GfdAllocator>>;
    fn get_task_free_list(&self) -> Option<&GfdFreeList<GfdTask, GfdAllocator>>;
    fn get_task_free_list_mut(&self) -> Option<&mut GfdFreeList<GfdTask, GfdAllocator>>;
}

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

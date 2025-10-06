use crate::{
    kernel::{
        chip::Chip,
        graphics::GraphicsStateSteam,
        global_impl::GlobalImpl
    },
    utility::{
        free_list::FreeList as GfdFreeList,
        math::RandomUnaligned,
    }
};

include!("global_common.rs");

pub(crate) const RENDER_LISTS: usize = 2;
pub(crate) const RENDER_STATES: usize = 33;
pub(crate) const SCENE_LISTS: usize = 2;
pub(crate) const MATERIAL_LISTS: usize = 8;
pub(crate) const SHADER_SOURCE: usize = 146;
pub(crate) const FIXED_VERTEX_SHADERS: usize = 48;
pub(crate) const FIXED_PIXEL_SHADERS: usize = 146;

#[repr(C)]
#[derive(Debug)]
pub struct Global { // (GlobalSteam, default state)
    controller_disconnected: Option<fn()>,
    callback_8: Option<fn()>,
    controller_connected: Option<fn()>,
    tick: u64,
    timestamp: u64,
    frames1: u64,
    pub(crate) graphics: GraphicsStateSteam,
    physics: [u8; 0x50],
    tasks: TaskGlobal,
    field2ce8: [u8; 0x80],
    dev_file_mutex: RecursiveMutex,
    free_list_head: *mut GfdFreeList,
    free_list_mutex: RecursiveMutex,
    chip_free_list: *mut GfdFreeList<Chip, GfdAllocator>,
    node_free_list: *mut GfdFreeList,
    node_free_list_entries_per_block: u32,
    task_free_list: *mut GfdFreeList<GfdTask, GfdAllocator>,
    task_free_list_entries_per_block: u32,
    delayed_proc_free_list: *mut GfdFreeList,
    dlyp_free_list_entries_per_block: u32,
    delayed_proc_item_head: *mut u8,
    delayed_proc_item_tail: *mut u8,
    delayed_proc_item_mutex: RecursiveMutex,
    field2e30: u32,
    delay_frame: u32,
    delay_force_frame: u32,
    main_stack_size: u32,
    obj_sync_cb_head: [*mut u8; 3],
    obj_sync_cb_mutex: RecursiveMutex,
    field2e80: [*mut u8; 2],
    system_font: *mut u8,
    uid: u64
}

impl Global {
    pub fn gfd_global_exists() -> bool {
        unsafe { crate::globals::get_gfd_global().is_some() }
    }
    pub fn get_gfd_global() -> &'static dyn GlobalImpl {
        let glb = unsafe { crate::globals::get_gfd_global_unchecked() };
        glb
    }
    pub fn get_gfd_global_mut() -> &'static mut dyn GlobalImpl {
        let glb = unsafe { crate::globals::get_gfd_global_unchecked_mut() };
        glb
    }
}

impl GlobalImpl for Global {
    fn get_flags(&self) -> GlobalFlags {
        todo!()
    }
    fn get_tasks(&self) -> &TaskGlobal {
        &self.tasks
    }
    fn get_tasks_mut(&mut self) -> &mut TaskGlobal {
        &mut self.tasks
    }
    fn get_random_mut(&mut self) -> &mut RandomUnaligned {
        todo!()
    }
    fn get_uid(&mut self) -> u64 {
        self.uid = self.uid.wrapping_add(1).max(1);
        self.uid
    }
    fn get_task_free_list_unchecked_mut(&mut self) 
    -> &mut GfdFreeList<GfdTask, GfdAllocator> {
        unsafe { &mut *self.task_free_list }
    }
    fn get_free_list_mutex(&mut self) -> &mut RecursiveMutex {
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
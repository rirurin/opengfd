#![allow(dead_code, unused_imports)]
use allocator_api2::alloc::{ Allocator, Global };
use cpp_types::msvc::{
    function::FunctionImpl,
    hash::FNV1A,
    list::{ List, ListNode, ListSingleNode, ListDoubleNode },
    mutex::{ ConditionVariable, Mutex, MutexGuard },
    shared_ptr::{ SharedPtr, WeakPtr },
    string::String as CppString,
    unordered::Map,
};
use crate::{
    device::ngr::allocator::AllocatorHook,
    tpl::resource::Resource
};
use riri_mod_tools_rt::logln;
use std::fmt::Debug;

// type LoadingFileList<A> = List<ListNode<SharedPtr<Resource<usize, A>, A>, A>, SharedPtr<Resource<usize, A>, A>, A>; 
// type ActiveFileMap<A> = Map<FNV1A, CppString<u8, A>, *const Resource<usize, A>, A>;
type LoadingFileList<A> = List<ListNode<SharedPtr<Resource<usize, A>, A>, A>, SharedPtr<Resource<usize, A>, A>, A>; 
type ActiveFileMap<A> = Map<FNV1A, CppString<u8, A>, WeakPtr<Resource<usize, A>, A>>;

#[repr(C)]
pub struct FileManager<A = AllocatorHook>
where A: Allocator + Clone
{
    running: SharedPtr<bool, A>,
    mutex: Mutex,
    cond_mtx_ptr: SharedPtr<Mutex, A>,
    cond: ConditionVariable,
    queue: bool,
    thread_info: [usize; 2],
    loading_file_list: LoadingFileList<A>,
    active_files: ActiveFileMap<A>,
    ui_callback: FunctionImpl<usize, bool>, // UUIC_SimpleLoading
    _allocator: A
}
/*
impl<A> Debug for FileManager<A>
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    }
}
*/

impl<A> FileManager<A>
where A: Allocator + Clone
{
    pub fn get_active_files(&self) -> &ActiveFileMap<A> {
        &self.active_files
    }
    pub fn get_loading_file_list(&self) -> &LoadingFileList<A> {
        &self.loading_file_list
    }
    pub fn get_active_files_mut(&mut self) -> &mut ActiveFileMap<A> {
        &mut self.active_files
    }
    pub fn get_loading_file_list_mut(&mut self) -> &mut LoadingFileList<A> {
        &mut self.loading_file_list
    }

    pub fn find_by_key(&self, key: &CppString<u8, A>) -> Option<&WeakPtr<Resource<usize, A>, A>> {
        self.get_active_files().find(key).map(|v| v.get_value())
    }

    pub fn find_by_str(&self, key: &str) -> Option<&WeakPtr<Resource<usize, A>, A>> {
        self.find_by_key(&CppString::from_str_in(key, self._allocator.clone()))
    }
    // 0x141471590 and 0x141474070
    // TPL::Resource::CreateResourceContainer
    // TPL::Resource::tplResourceCreate
    pub fn add_resource(&mut self, key: &CppString<u8, A>) -> SharedPtr<Resource<usize, A>, A> {
        // create resource
        let new_resrc = SharedPtr::make_shared_in(Resource::new(
            key.clone(), self._allocator.clone()), self._allocator.clone());
        self.add_resource_inner(key, new_resrc)
    }

    #[inline(always)]
    fn add_resource_inner(&mut self, key: &CppString<u8, A>, mut new_resrc: SharedPtr<Resource<usize, A>, A>)
        -> SharedPtr<Resource<usize, A>, A> {
        let resrc_vtable = unsafe { &raw const *crate::globals::get_tpl_resource_shared_ptr_unchecked() };
        new_resrc._force_set_vtable(resrc_vtable);
        let resrc_mut = &raw mut *new_resrc.get_mut();
        unsafe { &mut *resrc_mut }.set_owner(new_resrc.downgrade()); // <1:1> -> <1:2>
        let mut mutex = self.lock_file_mutex();
        // add to active file map
        let path = new_resrc.get().get_filename().clone();
        (&mut *mutex).active_files.insert(path, new_resrc.downgrade()); //  <1:2> -> <1:3>
        // add to loading list
        (&mut *mutex).loading_file_list.push(new_resrc.clone()); // <1:3> -> <2:3>
        if !(&*mutex).queue {
            // UI callback (show Gallica loading screen)
            if let Some(cb) = (&mut *mutex).ui_callback.get() {
                cb.call(true);
            }
            (&mut *mutex).queue = true;
            // wake up condition variable for loading files
            let mut signal_cnd_lock = (&mut *mutex).lock_cond_var();
            (&mut *signal_cnd_lock).cond.signal();
        }
        new_resrc
    }

    /*
    pub fn add_resource_apk(&mut self, key: &CppString<u8, A>) -> SharedPtr<Resource<usize, A>, A> {
        // create resource
        let new_resrc = SharedPtr::make_shared_in(Resource::new_apk(
            key.clone(), self._allocator.clone()), self._allocator.clone());
        self.add_resource_inner(key, new_resrc)
    }
    */

    pub fn get_running(&self) -> bool { *self.running.get() }
    pub fn set_running(&mut self, new: bool) { *self.running.get_mut() = new; }

    pub fn lock_file_mutex(&mut self) -> MutexGuard<'_, Self> {
        let self_ptr = &raw mut *self;
        self.mutex.lock(unsafe { &mut *self_ptr })
    }
    pub fn lock_cond_var(&mut self) -> MutexGuard<'_, Self> {
        let self_ptr = &raw mut *self;
        self.cond_mtx_ptr.get_mut().lock(unsafe { &mut *self_ptr })
    }

    pub fn enable_queue(&mut self) { self.queue = true }
    pub fn disable_queue(&mut self) { self.queue = false }
    pub fn check_queue(&self) -> bool { self.queue }

    pub fn wait_for_file_to_load(&mut self) {
        let mut cnd_mtx = self.cond_mtx_ptr.clone();
        self.cond.wait(cnd_mtx.get_mut());
        // let cnd_ptr = cnd_mtx.get_ptr();
        // let mut cnd_lock = cnd_mtx.get_mut().lock(unsafe { &mut *cnd_ptr });
        // self.cond.wait(&mut *cnd_lock);
    }
}

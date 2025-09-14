use crate::{
    kernel::{
        allocator::GfdAllocator,
        chip::Chip,
        global::{
            GlobalFlags,
            TaskGlobal
        },
        task::Task as GfdTask,
    },
    utility::{
        free_list::FreeList,
        math::RandomUnaligned,
    }
};
#[cfg(feature = "v2-core")]
use crate::utility::mutex::Mutex;
#[cfg(feature = "v1-core")]
use crate::utility::mutex::RecursiveMutex;

pub trait GlobalImpl {
    fn get_flags(&self) -> GlobalFlags;
    fn get_tasks(&self) -> &TaskGlobal;
    fn get_tasks_mut(&mut self) -> &mut TaskGlobal;
    fn get_random_mut(&mut self) -> &mut RandomUnaligned;
    /// Original function: gfdGetUID
    fn get_uid(&mut self) -> u64;
    fn get_task_free_list_unchecked_mut(&mut self) -> &mut FreeList<GfdTask, GfdAllocator>;
    #[cfg(feature = "v2-core")]
    fn get_free_list_mutex(&mut self) -> &mut Mutex;
    #[cfg(feature = "v1-core")]
    fn get_free_list_mutex(&mut self) -> &mut RecursiveMutex;
    fn get_free_list_head(&self) -> Option<&FreeList>;
    fn get_free_list_head_mut(&self) -> Option<&mut FreeList>;
    fn get_free_list_head_ptr(&self) -> *mut FreeList;
    fn set_free_list_head_mut(&mut self, new: *mut FreeList);
    fn get_chip_free_list(&self) -> Option<&FreeList<Chip, GfdAllocator>>;
    fn get_chip_free_list_mut(&self) -> Option<&mut FreeList<Chip, GfdAllocator>>;
    fn get_task_free_list(&self) -> Option<&FreeList<GfdTask, GfdAllocator>>;
    fn get_task_free_list_mut(&self) -> Option<&mut FreeList<GfdTask, GfdAllocator>>;
}
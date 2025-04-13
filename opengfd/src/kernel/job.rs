use allocator_api2::alloc::Allocator;
use crate::{
    kernel::{
        allocator::GfdAllocator,
        chip::Chip
    },
    utility::free_list::FreeList
};
use riri_mod_tools_rt::address::get_thread_id;
use std::ptr::NonNull;
use windows::Win32::{
    Foundation::HANDLE,
    System::Threading::CRITICAL_SECTION
};

#[repr(C)]
pub struct Job<A = GfdAllocator> 
where A: Allocator + Clone
{
    field0: u32,
    field4: u32,
    area: *const u8,
    entry_size: u32,
    field14: u32,
    field18: u32,
    field1c: u32,
    event0: NonNull<JobEvent>,
    event1: NonNull<JobEvent>,
    mutex: u32,
    field34: u32,
    workers: NonNull<JobWorker<A>>,
    worker_count: u32,
    event_handle: HANDLE,
    _allocator: A
}

#[repr(C)]
pub struct JobEvent {
    field0: u32,
    field4: u32,
    field8: u32,
    fieldc: u32,
    handle0: HANDLE,
    handle1: HANDLE,
    mutex: CRITICAL_SECTION
}

#[repr(C)]
pub struct JobWorker<A = GfdAllocator>
where A: Allocator + Clone
{
    field0: *const u8,
    field8: *const u8,
    field10: *const u8,
    data: JobWorkerData<A>,
    thread_id: u32
}

#[repr(C)]
pub struct JobWorkerData<A = GfdAllocator> 
where A: Allocator + Clone
{
    field0: [u8; 0x48],
    free_list: NonNull<FreeList<Chip, A>>,
    _allocator: A
}

impl<A> Job<A> 
where A: Allocator + Clone
{
    pub fn get_worker_count(&self) -> usize {
        self.worker_count as usize
    }
    pub unsafe fn is_main_thread() -> bool {
        *crate::globals::get_main_thread_id_unchecked() == get_thread_id() as u32
    }
}

impl Job<GfdAllocator> {
    pub fn get_worker() -> Option<&'static mut JobWorkerData<GfdAllocator>> {
        if unsafe { !Self::is_main_thread() } {
            let curr_thread = get_thread_id() as u32;
            let job = unsafe { crate::globals::get_job_list_unchecked_mut() };
            for worker in job {
                if worker.thread_id == curr_thread { return Some(&mut worker.data) }
            }
        }
        None
    }
}

impl<'a, A: 'static> IntoIterator for &'a mut Job<A>
where A: Allocator + Clone
{
    type IntoIter = JobWorkerIterator<'a, A>;
    type Item = &'a mut JobWorker<A>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { owner: self, count: 0 }
    }
}

pub struct JobWorkerIterator<'a, A> 
where A: Allocator + Clone
{
    owner: &'a mut Job<A>,
    count: usize,
}

impl<'a, A> Iterator for JobWorkerIterator<'a, A>
where A: Allocator + Clone
{
    type Item = &'a mut JobWorker<A>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.owner.get_worker_count() < self.count {
            let p_worker = unsafe { self.owner.workers.as_ptr().add(self.count) };
            self.count += 1;
            Some(unsafe { &mut *p_worker })
        } else { None }
    }
}

impl<A> JobWorkerData<A>
where A: Allocator + Clone
{
    pub fn get_free_list_mut(&mut self) -> &mut FreeList<Chip, A> {
        unsafe { self.free_list.as_mut() }
    }
}
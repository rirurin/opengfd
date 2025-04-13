use allocator_api2::alloc::{ Allocator, AllocError };
use crate::{
    kernel::{
        allocator::GfdAllocator,
        job::Job
    },
    utility::free_list::FreeList
};
use std::{
    alloc::Layout,
    mem::MaybeUninit,
    ptr::NonNull
};

type ChipList = FreeList<Chip, GfdAllocator>;

const CHIP_SIZE: usize = 0x200;
const MAXIMUM_ACCEPTABLE_SIZE: usize = CHIP_SIZE - size_of::<*const ChipList>();

#[repr(C)]
pub struct Chip(MaybeUninit<[u8; CHIP_SIZE]>);
impl Chip {
    fn get_free_list_ptr(&mut self) -> *mut &mut ChipList {
        let ptr = unsafe { (&raw const *self as *const u8).add(MAXIMUM_ACCEPTABLE_SIZE) };
        ptr as *mut &mut ChipList
    }
    pub fn get_free_list_mut(&mut self) -> &mut ChipList {
        unsafe { &mut **self.get_free_list_ptr() }
    }
}

#[derive(Debug)]
pub struct ChipAllocator;

unsafe impl Allocator for ChipAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if layout.size() > MAXIMUM_ACCEPTABLE_SIZE { return Err(AllocError) }
        let chip_list = match Job::get_worker() {
            // async thread
            Some(v) => v.get_free_list_mut(),
            None => {
                let glb = unsafe { crate::globals::get_gfd_global_unchecked_mut() };
                glb.get_chip_free_list_mut().unwrap()
            }
        };
        let new = chip_list.add();
        unsafe { *(&mut *new).get_free_list_ptr() = chip_list; }
        let alloc = unsafe { NonNull::new_unchecked(new as *mut u8) };
        Ok(NonNull::slice_from_raw_parts(alloc, CHIP_SIZE))

    }
    #[allow(unused_variables)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let inst = unsafe { &mut *(ptr.as_ptr() as *mut Chip) };
        inst.get_free_list_mut().remove(ptr.as_ptr() as *mut Chip);
    }
}
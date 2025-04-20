use allocator_api2::alloc::Allocator;
use crate::{
    kernel::allocator::GfdAllocator,
    utility::{
        name::Name,
        reference::Reference
    }
};
use std::ptr::NonNull;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AssetTypeHandle(*mut u8);

#[repr(C)]
#[derive(Debug)]
pub struct Asset<A = GfdAllocator> 
where A: Allocator + Clone
{
    type_: u32,
    data: *mut u8,
    size: u32,
    handle: AssetTypeHandle,
    dirty: u32,
    access: u32,
    attribute: u32,
    name: Name<A>,
    length: u32,
    ref_: Reference,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _allocator: A
}

impl<A> Asset<A>
where A: Allocator + Clone
{
    pub fn get_next(&self) -> Option<&Self> {
        self.next.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_prev(&self) -> Option<&Self> {
        self.prev.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_size(&self) -> usize {
        self.size as usize
    }
}
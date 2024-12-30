#![allow(dead_code, unused_variables)]
use allocator_api2::alloc::{ Allocator, Global };
use std::{
    alloc::Layout,
    // borrow::Borrow,
    mem::{ align_of, size_of },
    ptr::NonNull
};

type IndexType = i32;

/// A structure used to manage arrays of objects in various sections of GFD Engine. It holds
/// ownership over elements stored inside it, and can call a destructor for each field once the
/// array is sent out of scope (see destructor field for more info)
/// (Original file: gfdItemArray.c)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ItemArray<T, A: Allocator = Global> {
    /// The capacity of the buffer
    capacity: IndexType,
    /// The size of each element stored in the buffer
    element: IndexType,
    /// The number of elements stored in the buffer
    count: IndexType,
    /// A pointer to the buffer
    buffer: Option<NonNull<T>>,
    /// Seems to mostly be a debug thing?
    hint: IndexType,
    /// Call a custom destructor for each element. This will automatically be set to a Rust type's
    /// drop method if it implements the Drop trait, otherwise it's left blank unless explicitly
    /// defined.
    // destructor: Option<fn(T) -> ()>,
    destructor: Option<usize>,
    _allocator: A
}

// const CAPACITY_START: i32 = 8;

impl<T> ItemArray<T, Global> {
    pub fn new() -> Self {
        Self::new_unmanaged(Global)
    }
    pub fn with_capacity(cap: i32) -> Self {
        Self::with_capacity_unmanaged(cap, Global)
    }
}
/*
impl<T> ItemArray<T, Global> 
    where T: Drop
{

}
*/

impl<T, A> ItemArray<T, A>
    where A: Allocator 
{
    /// (Original function: gfdItemArrayCreateHint)
    pub fn new_unmanaged(alloc: A) -> Self {
        let element = size_of::<T>().try_into().unwrap();
        Self {
            capacity: 0,
            element,
            count: 0,
            buffer: None,
            hint: 0,
            destructor: None,
            _allocator: alloc
        }
    }

    fn get_layout(&self) -> Layout { Self::get_layout_sized(self.element, self.capacity) }

    fn get_layout_sized(element: IndexType, cap: IndexType) -> Layout {
        Layout::from_size_align((element * cap) as usize, align_of::<T>()).unwrap()
    }
    pub fn clear(&mut self) {

    }
    pub fn with_capacity_unmanaged(cap: IndexType, alloc: A) -> Self {
        let element = size_of::<T>().try_into().unwrap();
        let capacity = cap;
        let count = 0;
        let buffer = Some(alloc.allocate(Self::get_layout_sized(element, cap)).unwrap().cast());
        let hint = 0;
        let destructor = None;
        Self { capacity, element, count, buffer, hint, destructor, _allocator: alloc }
    }
    /// (Original function: gfdItemArrayInsert)
    pub fn insert(&mut self, index: IndexType, new_clear: bool) -> bool {
        true
    }
    pub fn extend(&mut self, count: IndexType, new_clear: bool) {

    }
    /// (Original function: gfdItemArrayNewEntry)
    pub fn extend_once(&mut self, new_clear: bool) { self.extend(1, new_clear) }

    pub fn push(&mut self, entry: T) {

    }
    /// (Original function: gfdItemArrayRemove)
    pub fn remove(&mut self, idx: IndexType) -> bool {
        true
    }
    /// (Original function: gfdItemArraySetCapacity)
    fn set_capacity(&mut self, new_cap: IndexType, new_clear: bool) -> bool {
        true
    }
    /// (Original function: gfdItemArraySetCount)
    fn set_count(&mut self, new_cap: IndexType, new_clear: bool) {

    }
    /// (Original function: gfdItemArraySwap)
    pub fn swap(&mut self, idx0: IndexType, idx1: IndexType) -> bool {
        true
    }
}
/*
impl<T, A> ItemArray<T, A>
    where T: Drop,
    A: Allocator
{
    pub fn with_capacity_in(cap: IndexType, alloc: A) -> Self {
        let element = size_of::<T>().try_into().unwrap();
        let buffer = Some(alloc.allocate(Self::get_layout_sized(element, cap)).unwrap().cast());
        // let a = std::ptr::addr_of!(std::mem::drop::<T>);
        // let destructor = Some(std::mem::drop::<T>);
        let destructor = None;
        // std::mem::needs_drop()
        Self {
            capacity: cap,
            element,
            count: 0,
            buffer,
            hint: 0,
            destructor,
            _allocator: alloc
        }
    }
}
*/
impl<T, A> Drop for ItemArray<T, A>
    where A: Allocator
{
    /// (Original function: gfdItemArrayRelease)
    fn drop(&mut self) {
        for i in 0..self.element {
            let el = unsafe { self.buffer.unwrap().as_ptr().add(i as usize) };
            let _ = unsafe { Box::from_raw(el) };
        }
    }
}

pub mod ffi {

}

#[cfg(test)]
pub mod tests {

}

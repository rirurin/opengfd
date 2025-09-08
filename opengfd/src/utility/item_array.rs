#![allow(dead_code, unused_variables)]
use allocator_api2::alloc::Allocator;
use crate::kernel::allocator::GfdAllocator;
use std::{
    alloc::Layout,
    fmt::{ Debug, Display, Formatter },
    error::Error,
    io,
    mem::{align_of, size_of},
    ops::{Index, IndexMut},
    ptr::NonNull
};
use std::ops::Range;

type IndexType = i32;

/// A structure used to manage arrays of objects in various sections of GFD Engine. It holds
/// ownership over elements stored inside it, and can call a destructor for each field once the
/// array is sent out of scope (see destructor field for more info)
/// (Original file: gfdItemArray.c)
#[repr(C)]
pub struct ItemArray<T, A = GfdAllocator>
where A: Allocator + Clone
{
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
    destructor: Option<fn(&T)>,
    _allocator: A
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ItemArrayError {
    AllocationTooLarge,
    CannotAllocate,
    IndexOutOfRange
}
impl Error for ItemArrayError {}
impl Display for ItemArrayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T, A> ItemArray<T, A>
// where T: Debug + Clone,
//       A: Allocator + Clone
where A: Allocator + Clone
{
    /// (Original function: gfdItemArrayCreateHint)
    pub fn new(alloc: A) -> Self {
        Self {
            capacity: 0,
            element: size_of::<T>() as IndexType,
            count: 0,
            buffer: None,
            hint: 0,
            destructor: None,
            _allocator: alloc
        }
    }

    pub fn with_capacity(capacity: usize, alloc: A) -> Result<Self, Box<dyn Error>> {
        let buffer = alloc.allocate(Self::get_layout_sized(size_of::<T>() as IndexType, capacity as IndexType))?.cast();
        Ok(Self {
            capacity: capacity as IndexType,
            element: size_of::<T>() as IndexType,
            count: 0,
            buffer: Some(buffer),
            hint: 0,
            destructor: None,
            _allocator: alloc
        })
    }

    fn get_layout(&self) -> Layout { Self::get_layout_sized(self.element, self.capacity) }

    fn get_layout_sized(element: IndexType, cap: IndexType) -> Layout {
        unsafe { Layout::from_size_align_unchecked((element * cap) as usize, align_of::<usize>()) }
    }

    fn resize_to(&mut self, new_size: usize) -> Result<(), ItemArrayError> {
        if new_size * self.element as usize > IndexType::MAX as usize {
            return Err(ItemArrayError::AllocationTooLarge);
        }
        let new = self._allocator
            .allocate(Self::get_layout_sized(self.element, new_size as IndexType))
            .map_err(|_| ItemArrayError::CannotAllocate)?.cast();
        if let Some(b) = self.buffer {
            unsafe {
                std::ptr::copy_nonoverlapping(b.as_ptr(), new.as_ptr(), self.capacity as usize);
                self._allocator.deallocate(std::mem::transmute(b), self.get_layout());
            }
        }
        self.capacity = new_size as IndexType;
        self.buffer = Some(new);
        Ok(())
    }

    fn resize_auto(&mut self) -> Result<(), ItemArrayError> {
        self.resize_to(self.buffer.map_or(0x8, |_| self.capacity as usize * 2))
    }

    fn resize_for_extra(&mut self, extra: usize) -> Result<(), ItemArrayError> {
        let new_min = self.count as usize + extra;
        self.resize_to(1 << (usize::BITS - new_min.leading_zeros()))
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

    pub fn len(&self) -> IndexType {
        self.count
    }

    pub fn capacity(&self) -> IndexType {
        self.capacity
    }

    fn check_bounds_inner(&self, index: usize, fail: fn(usize, usize) -> bool) -> Result<(), ItemArrayError> {
        match fail(index, self.count as usize) {
            true => Err(ItemArrayError::IndexOutOfRange),
            false => Ok(())
        }
    }

    fn check_bounds_for_access(&self, index: usize) -> Result<(), ItemArrayError> {
        self.check_bounds_inner(index, |i, c| i >= c)
    }

    fn check_bounds_for_insertion(&self, index: usize) -> Result<(), ItemArrayError> {
        self.check_bounds_inner(index, |i, c| i > c)
    }

    fn check_resize(&mut self) -> Result<(), ItemArrayError> {
        if self.count + 1 > self.capacity {
            self.resize_auto()?;
        }
        Ok(())
    }
}

impl<T, A> ItemArray<T, A>
where A: Allocator + Clone
{
    unsafe fn insert_inner(&mut self, index: usize, values: &[T]) {
        std::ptr::copy(
            self.buffer.unwrap().as_ptr().add(index),
            self.buffer.unwrap().as_ptr().add(index + values.len()),
            self.count as usize - index
        );
        std::ptr::copy_nonoverlapping(
            values.as_ptr(),
            self.buffer.unwrap().as_ptr().add(index),
            values.len()
        );
        self.count += values.len() as IndexType;
    }

    unsafe fn remove_inner(&mut self, range: Range<usize>) {
        let len = range.len();
        let start = range.start;
        let end = range.end;
        for i in range {
            std::ptr::drop_in_place(self.buffer.unwrap().as_ptr().add(i));
        }
        std::ptr::copy(
            self.buffer.unwrap().as_ptr().add(end),
            self.buffer.unwrap().as_ptr().add(start),
            self.count as usize - end
        );
        self.count -= len as IndexType;
    }
}

// ADD ELEMENTS

impl<T, A> ItemArray<T, A>
where A: Allocator + Clone
{
    pub fn push(&mut self, entry: T) -> Result<(), ItemArrayError> {
        self.check_resize()?;
        unsafe { *self.buffer.unwrap().as_ptr().add(self.count as usize) = entry };
        self.count += 1;
        Ok(())
    }
    /// (Original function: gfdItemArrayInsert)
    pub fn insert(&mut self, entry: T, index: usize) -> Result<(), ItemArrayError> {
        self.check_bounds_for_insertion(index)?;
        self.check_resize()?;
        unsafe { self.insert_inner(index, &[entry]) };
        Ok(())
    }

    pub fn extend(&mut self, values: &[T]) -> Result<(), ItemArrayError> {
        if self.count as usize + values.len() > self.capacity as usize {
            self.resize_for_extra(values.len())?;
        }
        unsafe { self.insert_inner(self.count as usize, values) };
        Ok(())
    }
}

// REMOVE ELEMENTS

impl<T, A> ItemArray<T, A>
where A: Allocator + Clone
{
    pub fn clear(&mut self) {
        unsafe { self.remove_inner(0..self.count as usize) };
    }
    /// (Original function: gfdItemArrayRemove)
    pub fn remove(&mut self, index: usize) -> Result<(), ItemArrayError> {
        self.check_bounds_for_access(index)?;
        unsafe { self.remove_inner(index..index + 1) };
        Ok(())
    }
}

// ACCESSOR OPERATORS (get)

impl<T, A> ItemArray<T, A>
where A: Allocator + Clone
{
    pub fn get(&self, index: usize) -> Result<&T, ItemArrayError> {
        self.check_bounds_for_access(index)?;
        Ok(unsafe { std::mem::transmute(self.buffer.unwrap().as_ptr().add(index)) })
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, ItemArrayError> {
        self.check_bounds_for_access(index)?;
        Ok(unsafe { std::mem::transmute(self.buffer.unwrap().as_ptr().add(index)) })
    }

    pub fn as_slice(&self) -> &[T] {
        self.buffer.map_or(&[], |b| unsafe { std::slice::from_raw_parts(b.as_ptr(), self.count as usize) })
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        self.buffer.map_or(&mut [], |b| unsafe { std::slice::from_raw_parts_mut(b.as_ptr(), self.count as usize) })
    }
}

impl<T, A> Index<usize> for ItemArray<T, A>
where A: Allocator + Clone
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T, A> Index<IndexType> for ItemArray<T, A>
where A: Allocator + Clone
{
    type Output = T;
    fn index(&self, index: IndexType) -> &Self::Output {
        self.get(index as usize).unwrap()
    }
}

impl<T, A> IndexMut<usize> for ItemArray<T, A>
where A: Allocator + Clone
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T, A> IndexMut<IndexType> for ItemArray<T, A>
where A: Allocator + Clone
{
    fn index_mut(&mut self, index: IndexType) -> &mut Self::Output {
        self.get_mut(index as usize).unwrap()
    }
}

impl<T, A> Debug for ItemArray<T, A>
where T: Debug,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printf = "[ ".to_owned();
        let values = self.as_slice().iter().map(|v| format!("{:?}", v)).collect::<Vec<_>>();
        printf.push_str(&values.join(", "));
        printf.push_str(" ]");
        write!(f, "{}", printf)
    }
}

impl<T, A> Drop for ItemArray<T, A>
where A: Allocator + Clone
{
    /// (Original function: gfdItemArrayRelease)
    fn drop(&mut self) {
        if let Some(buf) = self.buffer {
            for e in self.as_slice_mut() {
                unsafe { std::ptr::drop_in_place(e) };
            }
            unsafe { self._allocator.deallocate(std::mem::transmute(self.buffer.unwrap()), self.get_layout()) };
        }
    }
}

pub mod ffi {

}

#[cfg(test)]
pub mod tests {

}

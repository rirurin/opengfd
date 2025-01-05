#![allow(unused_imports)]
use allocator_api2::alloc::Allocator;
use crate::{
    device::ngr::hint::MemHint,
    globals,
    utility::{
        mutex::{ RecursiveMutex, RecursiveMutexGuard },
        reference::{ GfdRc, GfdRcType, Reference }
    }
};
use std::{
    mem::{ align_of, size_of },
    ptr::NonNull
};
use opengfd_proc::GfdRcAuto;
use windows::Win32::System::{
    Performance::QueryPerformanceCounter,
    Threading::CRITICAL_SECTION
};
/*
#[repr(C)]
#[derive(Debug)]
// https://en.wikipedia.org/wiki/Left-leaning_red%E2%80%93black_tree
pub struct LLRBTree<K, V> {
    _cpp_vtable: *mut u8,
    _key: std::marker::PhantomData<V>,
    _value: std::marker::PhantomData<K>,
    size: usize
}
*/
#[repr(C)]
#[derive(Debug)]
pub struct List {
    _cpp_vtable: *mut u8,

}
#[repr(C)]
#[derive(Debug)]
pub struct PointerList<V> 
{
    _cpp_vtable: *mut u8,
    _head: Option<NonNull<PointerListEntry<V>>>,
    _tail: Option<NonNull<PointerListEntry<V>>>,
    size: usize,
    hint: MemHint
}
// vtable: 0x1422a7430
impl<V> PointerList<V> {
    pub fn find_by_predicate<F>(&self, entry: F) -> Option<&V>
        where F: Fn(&V) -> bool {
        let mut current = self._head;
        while let Some(e) = current {
            let data = unsafe { e.as_ref().data.as_ref() };
            if entry(data) { return Some(data); }
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
}

impl<V> PointerList<V> 
    where V: PartialEq
{
    pub fn find(&self, entry: &V) -> Option<&V> {
        let mut current = self._head;
        while let Some(e) = current {
            let data = unsafe { e.as_ref().data.as_ref() };
            if data == entry { return Some(data); }
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
}
// 0x14118bb30
impl<V> Drop for PointerList<V> {
    fn drop(&mut self) {
        let mut current = self._head;
        while let Some(e) = current {
            unsafe { std::ptr::drop_in_place(e.as_ref().data.as_ptr()) };
            let next = unsafe { e.as_ref().next };
                current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PointerListEntry<V> 
    // where V: PartialEq
{
    next: Option<NonNull<PointerListEntry<V>>>,
    prev: Option<NonNull<PointerListEntry<V>>>,
    data: NonNull<V>,
}

#[repr(C)]
#[derive(Debug)]
pub struct CriticalSection {
    _cpp_vtable: *mut u8,
    crit_section: RecursiveMutex
}

impl CriticalSection {
    pub fn lock<'a, T>(&'a mut self, data: &'a mut T) -> RecursiveMutexGuard<'a, T> {
        self.crit_section.lock(data)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CrcHash {
    _cpp_vtable: *const u8,
    hash: u32
}

impl CrcHash {
    pub fn new<T: std::hash::Hash>(val: &T) -> Self {
        let mut hasher = crc32fast::Hasher::new();
        val.hash(&mut hasher); 
        Self {
            _cpp_vtable: match globals::get_ngr_crchash_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            hash: hasher.finalize()
        }
    }
    pub fn get_hash(&self) -> u32 { self.hash }
}

#[repr(C)]
#[derive(Debug)]
pub struct ListNodeFreeList {
    _cpp_vtable: *const u8,
    list: *mut FreeList<PointerListEntryTypeless>,
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct ngr_1422ecad8 {
    _cpp_vtable: *const u8,
    field08: usize,
    field10: usize,
    counter: i64,
    field20: usize,
    field28: usize
}

impl ngr_1422ecad8 {
    // 0x1411e7b90
    pub fn new() -> Self {
        let mut out = Self {
            _cpp_vtable: std::ptr::null(),
            field08: 0,
            field10: 0,
            counter: 0,
            field20: 0,
            field28: 0
        };
        unsafe { QueryPerformanceCounter(&raw mut out.counter).unwrap() };
        out
    }
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct FreeList<T> {
    _cpp_vtable: *const u8,
    ref_: Reference,
    block_size_as_pointer: usize,
    element_size: usize,
    aligned_element_size: usize,
    entries_per_block: usize,
    alignment: usize,
    hint: MemHint,
    hint2: *mut MemHint,
    field50: i32,
    block_start: Option<NonNull<FreeListBlockLink<T>>>,
    block_end: Option<NonNull<FreeListBlockLink<T>>>,
    field68: ngr_1422ecad8
}

const FREE_LIST_ENTRIES_PER_BLOCK: usize = 0x400;
const FREE_LIST_BLOCKS: usize = 1;

impl<T> FreeList<T> {
    // 0x1411b0540
    pub fn new() -> Self {
        Self::new_inner(align_of::<T>(), FREE_LIST_ENTRIES_PER_BLOCK, FREE_LIST_BLOCKS, 2)
    }
    pub fn new_with_alignment(alignment: usize) -> Self {
        Self::new_inner(alignment, FREE_LIST_ENTRIES_PER_BLOCK, FREE_LIST_BLOCKS, 2)
    }
    fn new_inner(
        alignment: usize, 
        entries_per_block: usize,
        blocks: usize,
        field50: i32
    ) -> Self {
        let alignment = if alignment > align_of::<T>() { alignment } else { align_of::<T>() };
        Self {
            _cpp_vtable: std::ptr::null(),
            ref_: Reference::new(),
            block_size_as_pointer: size_of::<T>() + 7 >> 3,
            element_size: size_of::<T>(),
            aligned_element_size: size_of::<T>() - 1 + alignment & !(alignment - 1),
            entries_per_block,
            alignment,
            hint: MemHint::new(),
            hint2: std::ptr::null_mut(),
            field50,
            block_start: None,
            block_end: None,
            field68: ngr_1422ecad8::new()
        }
    }
}
/*
impl<T> FreeList<T> {
    fn new_inner_with_allocator<A>(
        alignment: usize,
        entries_per_block: usize,
        blocks: usize,
        field50: i32,
        alloc: A
    ) -> GfdRc<Self, A> 
    where A: Allocator
    {
        let alignment = if alignment > align_of::<T>() { alignment } else { align_of::<T>() };
        GfdRc::new(Self {
            _cpp_vtable: std::ptr::null(),
            ref_: Reference::new(),
            block_size_as_pointer: size_of::<T>() + 7 >> 3,
            element_size: size_of::<T>(),
            aligned_element_size: size_of::<T>() - 1 + alignment & !(alignment - 1),
            entries_per_block,
            alignment,
            hint: MemHint::new(),
            hint2: std::ptr::null_mut(),
            field50,
            block_start: None,
            block_end: None,
            field68: ngr_1422ecad8::new()
        })
    }
}
*/

#[repr(C)]
#[derive(Debug)]
pub struct FreeListBlockLink<T> {
    prev: Option<NonNull<FreeListBlockLink<T>>>,
    next: Option<NonNull<FreeListBlockLink<T>>>,
    field10: u8,
    data: std::marker::PhantomData<T>
}

#[repr(C)]
#[derive(Debug)]
struct PointerListEntryTypeless {

}

#[cfg(test)]
pub mod tests {
    use crate::device::ngr::renderer::state::{
        CullMode,
        FillMode,
        RasterizerKey
    };
    use crate::tests::{ OpengfdError, TestReturn };
    use std::hash::Hash;

    #[test]
    pub fn hash_rasterizer_platform() -> TestReturn {
        // Sample value collected from Metaphor
        let key = RasterizerKey {
            field_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            is_front_counter_clockwise: true,
            scissor_enable: false,
            antialiased_line_enable: true,
            depth_bias: 0,
            depth_bias_clamp: 0f32,
            slope_scaled_depth_bias: 0f32,
            depth_clip_enable: true
        };
        let mut hasher = crc32fast::Hasher::new();
        key.hash(&mut hasher);
        let expected: u32 = 0x1815a91a;
        let result = hasher.finalize();
        if expected == result {
            Ok(())
        } else {
            Err(Box::new(OpengfdError::new(
                format!("Incorrect hash value. Got 0x{:x}, expected 0x{:x}", result, expected)
            )))
        }
    }
}

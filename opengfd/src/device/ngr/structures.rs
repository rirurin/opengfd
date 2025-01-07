#![allow(unused_imports, dead_code)]
use allocator_api2::{
    alloc::Allocator,
    boxed::Box as Box2,
};
use crate::{
    device::ngr::{
        allocator::AllocatorHook,
        hint::MemHint
    },
    globals,
    utility::{
        mutex::{ RecursiveMutex, RecursiveMutexGuard },
        reference::{ GfdRc, GfdRcType, Reference }
    }
};
use std::{
    alloc::Layout,
    fmt::Debug,
    mem::{ align_of, size_of },
    ops::{ Deref, DerefMut },
    ptr::NonNull,
    sync::atomic::AtomicI32
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
pub struct PointerList<V, A = AllocatorHook>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    _head: Option<NonNull<PointerListEntry<V>>>,
    _tail: Option<NonNull<PointerListEntry<V>>>,
    size: usize,
    hint: MemHint,
    _allocator: A
}

// vtable: 0x1422a7430
impl<V> PointerList<V, AllocatorHook> 
{
    pub fn new() -> Self {
        Self::new_in(AllocatorHook)
    }
}

impl<V, A> PointerList<V, A>
where A: Allocator + Clone
{
    pub fn new_in(alloc: A) -> Self {
        assert!(size_of::<A>() == 0, "Allocator must be a zero sized type!");
        Self {
            // would have to define vtable for each type - C++ monomorphizes too!
            _cpp_vtable: std::ptr::null(),
            _head: None,
            _tail: None,
            size: 0,
            hint: MemHint::new_value(0xb000004),
            _allocator: alloc
        }
    }
    pub fn find_by_predicate<F>(&self, entry: F) -> Option<&V>
        where F: Fn(&V) -> bool {
        let mut current = self._head;
        while let Some(e) = current {
            let data = unsafe { e.as_ref().data.as_ref() };
            if entry(data) { 
                return Some(data); 
            }
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
    pub fn find_by_predicate_mut<F>(&self, entry: F) -> Option<&mut V>
        where F: Fn(&mut V) -> bool {
        let mut current = self._head;
        while let Some(mut e) = current {
            let data = unsafe { e.as_mut().data.as_mut() };
            if entry(data) { 
                return Some(data); 
            }
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
    pub fn add(&self, entry: &V) {

    }
}

impl<V> PointerList<V>
where V: GfdRcType + Debug
{
    /*
    pub fn add_in_rc<A: Allocator>(&mut self, entry: &V, alloc: A) {
        let a = GfdRc::clone_from_raw(entry);
    }
    */
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
impl<V, A> Drop for PointerList<V, A> 
where A: Allocator + Clone
{
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
pub struct PointerListEntry<V, A = AllocatorHook>
where A: Allocator + Clone
    // where V: PartialEq
{
    next: Option<NonNull<PointerListEntry<V>>>,
    prev: Option<NonNull<PointerListEntry<V>>>,
    data: NonNull<V>,
    _allocator: A
}
/*
impl<V, A> PointerListEntry<V, A>
where A: Allocator + Clone
{
    fn new(entry: &V) -> Self {
        Self {

        }
    }
}
*/

#[repr(C)]
#[derive(Debug)]
pub struct String<A = AllocatorHook> {
    _cpp_vtable: *const u8,
    text: *const u8,
    hint: MemHint,
    _allocator: A
}

#[repr(C)]
#[derive(Debug)]
pub struct StringHashed<A = AllocatorHook> {
    _cpp_vtable: *const u8,
    name: String<A>,
    hash: CrcHash
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
pub struct ListNodeFreeList<T> {
    _cpp_vtable: *const u8,
    list: *mut FreeList<T>,
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
            _cpp_vtable: match globals::get_ngr_1422ecad8_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
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

// ngrFreeList.cpp
#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct FreeList<T, A = AllocatorHook> 
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    ref_: Reference,
    free_indices_section_size: usize,
    element_size: usize,
    block_entry_size: usize,
    entries_per_block: usize,
    alignment: usize,
    hint: MemHint,
    lock: *mut SpinLock,
    field50: i32,
    block_end: Option<NonNull<FreeListBlockLink<T>>>,
    block_start: Option<NonNull<FreeListBlockLink<T>>>,
    field68: ngr_1422ecad8,
    _allocator: A
}

const FREE_LIST_ENTRIES_PER_BLOCK: usize = 0x400;
const FREE_LIST_BLOCKS: usize = 1;
impl<T> FreeList<T, AllocatorHook>
where T: GfdRcType + Debug
{
    pub fn new() -> GfdRc<Self, AllocatorHook> {
        Self::new_with_alignment(align_of::<T>())
    }
    pub fn new_with_alignment(alignment: usize) -> GfdRc<Self, AllocatorHook> {
        Self::new_inner(alignment, FREE_LIST_ENTRIES_PER_BLOCK, FREE_LIST_BLOCKS, 2, AllocatorHook)
    }
}
impl<T, A> FreeList<T, A> 
where T: GfdRcType + Debug,
      A: Allocator + Clone + Debug
{
    // 0x1411b0540
    pub fn new_in(alloc: A) -> GfdRc<Self, A> {
        Self::new_in_with_alignment(alloc, align_of::<T>())
    }
    pub fn new_in_with_alignment(alloc: A, alignment: usize) -> GfdRc<Self, A> {
        Self::new_inner(alignment, FREE_LIST_ENTRIES_PER_BLOCK, FREE_LIST_BLOCKS, 2, alloc)
    }
    fn new_inner(
        alignment: usize, 
        entries_per_block: usize,
        blocks: usize,
        field50: i32,
        alloc: A
    ) -> GfdRc<Self, A> {
        let alignment = if alignment > align_of::<T>() { alignment } else { align_of::<T>() };
        let free_indices_section_size = size_of::<T>() + 7 >> 3; // alignof(usize)
        let block_entry_size = size_of::<T>() - 1 + alignment & !(alignment - 1); // alignof
        let block_layout = unsafe { 
            Layout::from_size_align_unchecked(
                // originally was + 0x17, but that gets rounded up to 0x18 due to alignment anyway
                free_indices_section_size + size_of::<FreeListBlockLink<T>>() + 
                entries_per_block * block_entry_size + alignment,
                alignment
            ) 
        };
        let mut block_start = None;
        let mut block_end = None;
        for _ in 0..blocks {
            let mut new_block: NonNull<FreeListBlockLink<T>> = alloc.allocate_zeroed(block_layout).unwrap().cast();
            unsafe { new_block.as_mut().is_init = true; }
            if block_end.is_none() {
                block_start = Some(new_block);
                unsafe { new_block.as_mut().next = None; }
            } else {
                unsafe { 
                    new_block.as_mut().next = block_end; 
                    block_end.unwrap().as_mut().prev = Some(new_block);
                }
            }
            block_end = Some(new_block);
        } 
        GfdRc::new_in(Self {
            _cpp_vtable: std::ptr::null(),
            ref_: Reference::new(),
            free_indices_section_size,
            element_size: size_of::<T>(),
            block_entry_size,
            entries_per_block,
            alignment,
            hint: MemHint::new(),
            lock: Box2::into_raw(Box2::new_in(SpinLock::new(), alloc.clone())),
            field50,
            block_end,
            block_start,
            field68: ngr_1422ecad8::new(),
            _allocator: alloc.clone()
        }, alloc.clone())
    }
    pub fn add(&mut self) {
        let this = (unsafe { &mut *self.lock }).get_lock(self);
        if (&*this).block_start.is_none() {
        } else {

        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FreeListBlockLink<T> 
{
    prev: Option<NonNull<FreeListBlockLink<T>>>,
    next: Option<NonNull<FreeListBlockLink<T>>>,
    is_init: bool,
    // structure: 
    // free_indices: 
    _data: std::marker::PhantomData<T>
}

#[repr(C)]
#[derive(Debug)]
struct PointerListEntryTypeless {

}

#[repr(C)]
#[derive(Debug)]
pub struct SpinLock {
    _cpp_vtable: *const u8,
    lock: AtomicI32,
}

pub const SPIN_COUNT_BEFORE_YIELDING: usize = 1500;

impl SpinLock {
    // 0x141207930
    pub fn new() -> Self {
        Self {
            _cpp_vtable: match globals::get_ngr_spinlock_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            lock: 0.into()
        }
    }
    // 0x141207980
    // https://marabos.nl/atomics/memory-ordering.html
    pub fn get_lock<'a, T>(&'a mut self, item: &'a mut T) -> SpinLockGuard<'a, T> {
        loop {
            let mut count = SPIN_COUNT_BEFORE_YIELDING;
            while count > 0 {
                match self.lock.compare_exchange_weak(0, 1, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed) {
                    Ok(_) => return SpinLockGuard {
                        owner: self,
                        data: item
                    },
                    Err(_) => ()
                };
                count -= 1;
            }
            std::thread::yield_now();
        }
    }
}

#[derive(Debug)]
pub struct SpinLockGuard<'a, T> {
    owner: &'a mut SpinLock,
    data: &'a mut T
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    // 0x1412079d0
    fn drop(&mut self) {
        self.owner.lock.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
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

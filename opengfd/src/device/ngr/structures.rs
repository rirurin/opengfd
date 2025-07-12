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
    globals::{self, get_ngr_pointer_freelist_unchecked_mut},
    utility::{
        mutex::{ RecursiveMutex, RecursiveMutexGuard },
        reference::{ GfdRc, GfdRcType, Reference }
    }
};
use std::{
    alloc::Layout,
    ffi::CStr,
    fmt::Debug,
    ops::{ Deref, DerefMut, Index, IndexMut },
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
            match data {
                Some(v) => if entry(v) { return Some(v); },
                _ => (),
            }; 
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
            // unsafe { println!("Find by predicate! data: 0x{:x}", e.as_ref().data as usize); }
            let data = unsafe { e.as_mut().data.as_mut() };
            match data {
                Some(v) => if entry(v) { return Some(v); },
                _ => ()
            };
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
    /*
    pub fn add(&self, entry: &V) {

    }
    */
}

impl<V, A> PointerList<V, A>
where V: GfdRcType + Debug,
      A: Allocator + Clone
{
    pub fn add_in_rc(&mut self, entry: GfdRc<V, A>) {
        let freelist = unsafe { get_ngr_pointer_freelist_unchecked_mut().get_list() };
        let new_entry = unsafe { std::mem::transmute::<
            &mut PointerListEntry<u8>, &mut PointerListEntry<V>
            >(&mut *freelist.add())
        };
        new_entry.next = None;
        new_entry.prev = self._tail;
        let new_entry_ptr = NonNull::new(&raw mut *new_entry);
        if self._tail.is_none() {
            self._head = new_entry_ptr;
        } else {
            unsafe { self._tail.unwrap().as_mut().next = new_entry_ptr };
        }
        self._tail = new_entry_ptr;
        unsafe { std::ptr::write(
            &raw mut new_entry.data as *mut *mut V,
            entry.as_ptr() as *mut V
        )}
        // We've written the data into the pointer list entry, now "leak" this reference so it isn't
        // dropped at the end of this method's scope 
        GfdRc::into_raw(entry);
        self.size += 1;
    }
}

impl<V> PointerList<V> 
    where V: PartialEq
{
    pub fn find(&self, entry: &V) -> Option<&V> {
        let mut current = self._head;
        while let Some(e) = current {
            let data = unsafe { e.as_ref().data.as_ref() };
            match data {
                Some(v) => if v == entry { return Some(v); },
                _ => ()
            };
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
            match unsafe { e.as_ref().data.as_mut() } {
                Some(v) => unsafe { std::ptr::drop_in_place(&raw mut *v) },
                _ => (),
            };
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
    // data: NonNull<V>,
    data: *mut V,
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
pub struct Array<T, A = AllocatorHook>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    alloc: Option<NonNull<T>>,
    entry_size: usize,
    len: usize,
    max: usize,
    hint: MemHint,
    _allocator: A
}

impl<T> Array<T, AllocatorHook> {
    pub fn new(len: usize) -> Self { Self::new_in(len, AllocatorHook) }
}

static ARRAY_INITIAL_CAPACITY: usize = 4;

impl<T, A> Array<T, A>
where A: Allocator + Clone
{
    pub fn new_in(len: usize, allocator: A) -> Self {
        Self::new_in_hint(1, len, allocator)
    }

    // 0x1411cc7f0
    pub fn new_in_hint(hint: u32, len: usize, allocator: A) -> Self {
        let alloc = match len {
            0 => None,
            _ => Some(allocator.allocate(unsafe { Self::get_layout(len) }).unwrap().cast())
        };
        Self {
            _cpp_vtable: std::ptr::null(),
            alloc,
            entry_size: std::mem::size_of::<T>(),
            len: 0,
            max: 0,
            hint: MemHint::new_value(hint),
            _allocator: allocator
        }
    }

    pub fn from_slice(hint: u32, data: &[T], allocator: A) -> Self {
        let out = Self::new_in_hint(hint, data.len(), allocator);
        out
    }

    unsafe fn get_layout(count: usize) -> Layout {
        Layout::from_size_align_unchecked(std::mem::size_of::<T>() * count, std::mem::align_of::<T>())
    }

    fn resize(&mut self, max: usize) {
        let new_alloc = Some(self._allocator.allocate(unsafe { Self::get_layout(max) }).unwrap().cast());
        match &self.alloc {
            Some(old) => {
                unsafe {
                    std::ptr::copy_nonoverlapping(old.as_ptr(), new_alloc.unwrap().as_ptr(), self.max);
                    self._allocator.deallocate(NonNull::new_unchecked(old.as_ptr() as *mut u8), Self::get_layout(self.max));
                }
            },
            None => ()
        };
        self.alloc = new_alloc;
    }

    pub fn add(&mut self, new: T) {
        if self.len == self.max {
            match self.max {
                0 => self.resize(ARRAY_INITIAL_CAPACITY),
                i => self.resize(i * 2)
            }
        }
        unsafe { std::ptr::write(self.alloc.unwrap().add(self.len).as_ptr(), new); }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "Specified index for remove must be within bounds");
        let target = unsafe { std::ptr::read(self.alloc.unwrap().add(index).as_ptr()) };
        unsafe { std::ptr::copy(
            self.alloc.unwrap().add(index).as_ptr(), 
            self.alloc.unwrap().add(index + 1).as_ptr(),
            self.len - index - 1
        )};
        self.len -= 1;
        target
    }

    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn get_length(&self) -> usize { self.len }

    pub fn as_slice(&self) -> Option<&[T]> {
        self.alloc.map(|v| unsafe { std::slice::from_raw_parts(v.as_ptr(), self.len) })
    }
    pub fn as_slice_mut(&mut self) -> Option<&mut [T]> {
        self.alloc.map(|v| unsafe { std::slice::from_raw_parts_mut(v.as_ptr(), self.len) })
    }
}

impl<T, A> Array<T, A>
where T: PartialEq,
      A: Allocator + Clone
{
    pub fn find(&self, item: T) -> Option<&T> {
        match &self.alloc {
            Some(v) => {
                for i in 0..self.len {
                    let entry = unsafe { v.add(i).as_ref() };
                    if entry == &item { return Some(entry); }
                }
                None
            },
            None => None
        }
    }

    pub fn find_mut(&mut self, item: T) -> Option<&mut T> {
        match &self.alloc {
            Some(v) => {
                for i in 0..self.len {
                    let entry = unsafe { v.add(i).as_mut() };
                    if entry == &item { return Some(entry); }
                }
                None
            },
            None => None
        }
    }

    pub fn find_by_predicate<F>(&self, cb: F) -> Option<&T> 
    where F: Fn(&T) -> bool
    {
        match &self.alloc {
            Some(v) => {
                for i in 0..self.len {
                    let entry = unsafe { v.add(i).as_ref() };
                    if cb(entry) { return Some(entry); }
                }
                None
            },
            None => None
        }
    }

    pub fn find_by_predicate_mut<F>(&mut self, cb: F) -> Option<&mut T> 
    where F: Fn(&T) -> bool
    {
        match &self.alloc {
            Some(v) => {
                for i in 0..self.len {
                    let entry = unsafe { v.add(i).as_mut() };
                    if cb(entry) { return Some(entry); }
                }
                None
            },
            None => None
        }
    }
}

impl<T, A> Drop for Array<T, A>
where A: Allocator + Clone
{
    fn drop(&mut self) {
        match self.alloc {
            Some(v) => unsafe {
                for i in 0..self.len { std::ptr::drop_in_place(v.add(i).as_ptr()); }
                self._allocator.deallocate(
                    NonNull::new_unchecked(v.as_ptr() as *mut u8), Self::get_layout(self.max));  
            },
            None => ()
        }
    }
}

impl<T, A> Index<usize> for Array<T, A>
where A: Allocator + Clone
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.alloc.unwrap().as_ptr().add(index) }
    }
}

impl<T, A> IndexMut<usize> for Array<T, A>
where A: Allocator + Clone
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.alloc.unwrap().as_ptr().add(index) }
    }
}

/// Null terminated UTF-8 string
#[repr(C)]
#[derive(Debug)]
pub struct String<A = AllocatorHook> 
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    text: *const u8,
    hint: MemHint,
    _allocator: A
}

impl String {
    pub fn new() -> Self {
        Self::new_in(AllocatorHook)
    }
}

impl<A> String<A>
where A: Allocator + Clone
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            _cpp_vtable: unsafe { globals::get_ngr_string_vtable().map_or(std::ptr::null(), |v| &raw const *v) },
            text: std::ptr::null(),
            hint: MemHint::new_value(0x1000000),
            _allocator: alloc
        }
    }
    pub fn from_c_string(_text: *const std::ffi::c_char, alloc: A) -> Self {
        Self {
            _cpp_vtable: unsafe { globals::get_ngr_string_vtable().map_or(std::ptr::null(), |v| &raw const *v) },
            text: std::ptr::null(),
            hint: MemHint::new_value(0x1000000),
            _allocator: alloc
        }
    }
    pub fn value(&self) -> Option<&str> {
        if !self.text.is_null() {
            Some(unsafe { CStr::from_ptr(self.text as *const i8).to_str().unwrap() })
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct StringHashed<A = AllocatorHook>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    name: String<A>,
    hash: CrcHash
}

impl<A> StringHashed<A>
where A: Allocator + Clone
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            _cpp_vtable: unsafe { globals::get_ngr_string_hash_vtable().map_or(std::ptr::null(), |v| &raw const *v) },
            name: String::new_in(alloc),
            hash: CrcHash::new_empty()
        }
    }

    pub fn value(&self) -> Option<&str> {
        self.name.value()
    }
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
    pub fn new_empty() -> Self {
        Self {
            _cpp_vtable: unsafe { globals::get_ngr_crchash_vtable().map_or(std::ptr::null(), |v| &raw const *v) },
            hash: u32::MAX
        }
    }
    pub fn new<T: std::hash::Hash>(val: &T) -> Self {
        let mut hasher = crc32fast::Hasher::new();
        val.hash(&mut hasher); 
        Self {
            _cpp_vtable: unsafe { globals::get_ngr_crchash_vtable().map_or(std::ptr::null(), |v| &raw const *v) },
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
impl<T> ListNodeFreeList<T> {
    pub fn get_list(&mut self) -> &mut FreeList<T> {
        unsafe { &mut *self.list }
    }
}

// 0x1422ecad8
#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct PreciseTimer {
    _cpp_vtable: *const u8,
    field08: usize,
    field10: usize,
    counter: i64,
    field20: usize,
    field28: usize
}

impl PreciseTimer {
    // 0x1411e7b90
    pub fn new() -> Self {
        let mut out = Self {
            _cpp_vtable: unsafe { globals::get_ngr_1422ecad8_vtable().map_or(std::ptr::null(), |v| &raw const *v) },
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
pub struct FreeList<T = u8, A = AllocatorHook> 
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
    timer: PreciseTimer,
    _allocator: A,
    // _inner_data: std::marker::PhantomData<T>
}

const FREE_LIST_ENTRIES_PER_BLOCK: usize = 0x400;
const FREE_LIST_BLOCKS: usize = 1;
impl<T> FreeList<T, AllocatorHook>
where T: Debug
// where T: GfdRcType + Debug
{
    pub fn new() -> GfdRc<Self, AllocatorHook> {
        Self::new_with_alignment(align_of::<T>())
    }
    pub fn new_with_alignment(alignment: usize) -> GfdRc<Self, AllocatorHook> {
        Self::new_inner(alignment, FREE_LIST_ENTRIES_PER_BLOCK, FREE_LIST_BLOCKS, 2, AllocatorHook)
    }
}
impl<T, A> FreeList<T, A> 
where T: Debug,
      A: Allocator + Clone + Debug
{
    // 0x1411b0540
    pub fn new_in(alloc: A) -> GfdRc<Self, A> {
        Self::new_in_with_alignment(alloc, align_of::<T>())
    }
    pub fn new_in_with_alignment(alloc: A, alignment: usize) -> GfdRc<Self, A> {
        Self::new_inner(alignment, FREE_LIST_ENTRIES_PER_BLOCK, FREE_LIST_BLOCKS, 2, alloc)
    }

    fn add_new_blocks(&mut self, count: usize) {
        let block_layout = unsafe { 
            Layout::from_size_align_unchecked(
                // originally was + 0x17, but that gets rounded up to 0x18 due to alignment anyway
                self.free_indices_section_size + size_of::<FreeListBlockLink<T>>() + 
                self.entries_per_block * self.block_entry_size + self.alignment,
                self.alignment
            ) 
        };
        for _ in 0..count {
            let mut new_block: NonNull<FreeListBlockLink<T>> = self._allocator.allocate_zeroed(block_layout).unwrap().cast();
            unsafe { new_block.as_mut().is_init = true; }
            if self.block_end.is_none() {
                self.block_start = Some(new_block);
                unsafe { new_block.as_mut().next = None; }
            } else {
                unsafe { 
                    new_block.as_mut().next = self.block_end; 
                    self.block_end.unwrap().as_mut().prev = Some(new_block);
                }
            }
            self.block_end = Some(new_block);
        }
    }

    pub fn new_inner(
        alignment: usize, 
        entries_per_block: usize,
        blocks: usize,
        field50: i32,
        alloc: A
    ) -> GfdRc<Self, A> {
        let alignment = if alignment > align_of::<T>() { alignment } else { align_of::<T>() };
        let free_indices_section_size = entries_per_block + 7 >> 3; // alignof(usize)
        let block_entry_size = size_of::<T>() - 1 + alignment & !(alignment - 1); // alignof
        let mut new_list = GfdRc::new_in(Self {
            _cpp_vtable: match unsafe { globals::get_ngr_freelist_vtable() } {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            ref_: Reference::new(),
            free_indices_section_size,
            element_size: size_of::<T>(),
            block_entry_size,
            entries_per_block,
            alignment,
            hint: MemHint::new_value(0x1000000),
            lock: Box2::into_raw(Box2::new_in(SpinLock::new(), alloc.clone())),
            field50,
            block_end: None,
            block_start: None,
            timer: PreciseTimer::new(),
            _allocator: alloc.clone(),
            // _inner_data: std::marker::PhantomData
        }, alloc.clone());
        (&mut *new_list).add_new_blocks(blocks);
        new_list
    }
    pub fn add(&mut self, /*_hint: &MemHint*/) -> *mut T {
        let mut this = (unsafe { &mut *self.lock }).get_lock(self);
        if (&*this).block_start.is_none() {
            unsafe { (&mut *this).add_first_entry_in_new_block() }
        } else {
            (&mut *this).add_to_block_allocation()
        }
    }

    unsafe fn get_block_entry_ptr(&self, block: &mut FreeListBlockLink<T>, entry: usize) -> *mut T {
        (block.get_free_indices_ptr(self.free_indices_section_size) as *mut T).add(entry)
    }

    unsafe fn add_first_entry_in_new_block(&mut self) -> *mut T {
        self.add_new_blocks(1);
        let last_block = unsafe { self.block_end.unwrap().as_mut() };
        unsafe { *last_block.get_free_indices_ptr(0) = 0x80; }
        self.get_block_entry_ptr(last_block, 0)
    }

    fn add_to_block_allocation(&mut self) -> *mut T {
        let mut block = self.block_start;
        while let Some(mut n) = block {
            let block_in = unsafe { n.as_mut() };
            for i in 0..self.free_indices_section_size {
                unsafe {
                    if block_in.is_index_byte_full(i) { continue; }
                    else {
                        let exist = block_in.get_free_indices_ptr(i);
                        let leads = (*exist).leading_ones() as usize;
                        *exist |= 1 << (7 - leads);
                        return self.get_block_entry_ptr(block_in, i * 8 + leads)
                    }
                }
            }
            block = block_in.next;
        }
        unsafe { self.add_first_entry_in_new_block() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FreeListBlockLink<T> 
{
    prev: Option<NonNull<FreeListBlockLink<T>>>,
    next: Option<NonNull<FreeListBlockLink<T>>>,
    is_init: bool,
    _data: std::marker::PhantomData<T>
}

impl<T> FreeListBlockLink<T> {
    unsafe fn get_free_indices_ptr(&self, index: usize) -> *mut u8 {
        ((&raw const *self).add(1) as *mut u8).add(index)
    }
    unsafe fn get_free_indices(&self, index: usize) -> u8 {
        *self.get_free_indices_ptr(index)
    }
    unsafe fn is_index_byte_full(&self, index: usize) -> bool {
        self.get_free_indices(index) == 0xff    
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SpinLock {
    _cpp_vtable: *const u8,
    lock: AtomicI32,
}

impl SpinLock {
    // 0x141207930
    pub fn new() -> Self {
        Self {
            _cpp_vtable: match unsafe { globals::get_ngr_spinlock_vtable() } {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            lock: 0.into()
        }
    }
    // 0x141207980
    // https://marabos.nl/atomics/memory-ordering.html
    // Also see: crate::utility::mutex::Mutex
    pub fn get_lock<'a, T>(&'a mut self, item: &'a mut T) -> SpinLockGuard<'a, T> {
        SpinLockGuard::new(self, item)
    }
}

#[derive(Debug)]
pub struct SpinLockGuard<'a, T> {
    mutex: &'a mut SpinLock,
    data: &'a mut T
}

impl<'a, T> SpinLockGuard<'a, T> {
    fn new(mutex: &'a mut SpinLock, data: &'a mut T) -> Self {
        loop {
            for _ in 0..crate::utility::mutex::SPIN_COUNT_BEFORE_YIELDING {
                if let Ok(_) = mutex.lock.compare_exchange_weak(0, 1, 
                    std::sync::atomic::Ordering::Acquire, 
                    std::sync::atomic::Ordering::Relaxed) {
                    return Self { mutex, data }
                }
            }
            std::thread::yield_now();
        }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    // 0x1412079d0
    fn drop(&mut self) {
        self.mutex.lock.store(0, std::sync::atomic::Ordering::Release);
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

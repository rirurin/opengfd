use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::kernel::{
    allocator::GfdAllocator,
    global::Global
};
#[cfg(feature = "v2-core")]
use crate::utility::mutex::Mutex as GfdMutex;
#[cfg(feature = "v1-core")]
use crate::utility::mutex::RecursiveMutex as GfdMutex;
use std::{
    alloc::Layout,
    fmt::Debug,
    mem::{ align_of, size_of },
    ptr::NonNull
};

#[repr(C)]
#[derive(Debug)]
pub struct FreeList<T = usize, A = GfdAllocator>
where A: Allocator + Clone
{
    // flags: i32,
    flags: FreeListFlags,
    heap_size: u32, // free indices size (bytes)
    entry_size: u32, // entry size (bytes)
    entries_per_block: u32,
    alignment: u32,
    head: Option<NonNull<FreeListBlockLink<T>>>,
    tail: Option<NonNull<FreeListBlockLink<T>>>,
    mutex: GfdMutex,
    hint: u32, // GfdMemoryHint
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _allocator: A,
}

const DEFAULT_FREE_LIST_ENTRIES_PER_BLOCK: u32 = 0x80;
const DEFAULT_FREE_LIST_BLOCKS: u32 = 1;
const DEFAULT_MEMORY_HINT: u32 = 0xff000001;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FreeListFlags : u32 {
        const ALLOCATED_BEFORE_INIT = 1 << 0;
        const MALLOC_BLOCKS = 1 << 1;
    }
}

impl<T> FreeList<T>
where T: Debug + 'static
{
    pub fn new(alloc: GfdAllocator) -> &'static mut Self { Self::new_in(alloc) }
}

impl<T, A> FreeList<T, A>
where A: Allocator + Clone
{
    unsafe fn get_block_layout(&self) -> Layout {
        Layout::from_size_align_unchecked(
            (self.heap_size + size_of::<FreeListBlockLink<T>>() as u32 + 
            self.entries_per_block * self.entry_size + self.alignment) as usize,
            self.alignment as usize
        ) 
    }

    fn add_new_blocks(&mut self, count: u32) {
        let block_layout = unsafe { self.get_block_layout() };
        for _ in 0..count {
            let mut new_block: NonNull<FreeListBlockLink<T>> = self._allocator.allocate_zeroed(block_layout).unwrap().cast();
            if self.head.is_none() {
                self.tail = Some(new_block);
            } else {
                unsafe {
                    new_block.as_mut().next = self.head;
                    self.head.unwrap().as_mut().prev = Some(new_block);
                }
            }
            self.head = Some(new_block);
        }
    }

    pub fn link(&mut self) {
        let glb = Global::get_gfd_global_mut();
        let glb2 = unsafe { &mut *(&raw mut *glb) };
        let mut glb_mutex = glb.get_free_list_mutex().lock(glb2);
        if (&mut *glb_mutex).get_free_list_head_mut().is_some() {
            self.next = Some(unsafe { NonNull::new_unchecked((&*glb_mutex).get_free_list_head_ptr() as *mut Self) } );
            (&mut *glb_mutex).get_free_list_head_mut().unwrap().prev = Some(unsafe { NonNull::new_unchecked(&raw mut *self as *mut FreeList) }); 
        }
        (&mut *glb_mutex).set_free_list_head_mut(&raw mut *self as *mut FreeList);
    }

    pub fn add(&mut self, /*_hint: &MemHint*/) -> *mut T {
        let mut this = (unsafe { &mut *(&raw const self.mutex as *mut GfdMutex) }).lock(self);
        if (&*this).head.is_none() { unsafe { (&mut *this).add_first_entry_in_new_block() } } 
        else { (&mut *this).add_to_block_allocation() }
    }

    pub(crate) unsafe fn get_block_entry_ptr(&self, block: &FreeListBlockLink<T>, entry: usize) -> *mut T {
        // impractical for FFI
        // (block.get_free_indices_ptr(self.heap_size as usize) as *const T as *mut T).add(entry)
        (block.get_free_indices_ptr(self.heap_size as usize) as *const u8 as *mut u8)
            .add(entry * self.entry_size as usize) as *mut T
    }

    unsafe fn add_first_entry_in_new_block(&mut self) -> *mut T {
        self.add_new_blocks(1);
        let new_block = unsafe { self.head.unwrap().as_mut() };
        unsafe { *new_block.get_free_indices_ptr(0) = 0x80; }
        self.get_block_entry_ptr(new_block, 0)
    }

    fn add_to_block_allocation(&mut self) -> *mut T {
        let mut block = self.tail;
        while let Some(mut n) = block {
            let block_in = unsafe { n.as_mut() };
            for i in 0..self.heap_size as usize {
                unsafe {
                    if block_in.is_index_byte_full(i) { continue; }
                    else {
                        let exist = block_in.get_free_indices_ptr(i);
                        let leads = (*exist).leading_ones() as usize;
                        *exist |= 1 << (7 - leads);
                        return self.get_block_entry_ptr(block_in, i * u8::BITS as usize + leads)
                    }
                }
            }
            block = block_in.prev;
        }
        unsafe { self.add_first_entry_in_new_block() }
    }

    #[allow(dead_code)]
    pub(super) fn get(&self, index: usize) -> Option<&T> {
        let mut index_in_block = index;
        if self.tail.is_none() { return None }
        let mut block = unsafe { self.tail.unwrap().as_ref() };
        while index_in_block >= self.entries_per_block as usize {
            if block.prev.is_none() { return None }
            index_in_block -= self.entries_per_block as usize;
            block = unsafe { block.prev.unwrap().as_ref() };
        }
        let block_mask = unsafe { block.get_free_indices(index_in_block >> 3) };
        if block_mask & (1 << 7 - (index_in_block & 7)) != 0 {
            Some(unsafe { &*self.get_block_entry_ptr(block, index_in_block) })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub(super) fn get_block(&self, index: usize) -> Option<&FreeListBlockLink<T>> {
        let mut block = unsafe { self.tail.unwrap().as_ref() };
        for _ in 0..index {
            if block.prev.is_none() { return None }
            block = unsafe { block.prev.unwrap().as_ref() };
        }
        Some(block)
    }

    // #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut len = 0;
        // check each block
        let mut curr_block = self.tail;
        while let Some(v) = curr_block {
            let block = unsafe { v.as_ref() };
            curr_block = block.prev;
            for i in 0..self.heap_size as usize {
                len += unsafe { block.get_free_indices(i) }.count_ones() as usize;
            }
        }   
        len
    }

    #[allow(dead_code)]
    fn iter(&self) -> FreeListIterator<'_, T, A> {
        self.into_iter()
    }

    #[allow(dead_code)]
    fn iter_mut(&mut self) -> FreeListIteratorMut<'_, T, A> {
        self.into_iter()
    }

    #[allow(dead_code)]
    fn is_entry(&self, data: *mut T) -> bool {
        for v in self {
            if std::ptr::eq(&raw const *v, data) { return true }
        }
        false
    }

    fn remove_locked(&mut self, data: *mut T) {
        let mut curr_block = self.tail;
        while let Some(v) = curr_block {
            let block = unsafe { v.as_ref() };
            for i in 0..self.entries_per_block as usize {
                if unsafe { block.get_free_indices(i >> 3) 
                & 1 << (7 - i % u8::BITS as usize) != 0 } {
                    let ptr = unsafe { self.get_block_entry_ptr(block, i) };
                    if std::ptr::eq(ptr, data) {
                        unsafe { *block.get_free_indices_ptr(i >> 3) ^= 1 << (7 - i % u8::BITS as usize) }
                        let _ = unsafe { std::ptr::read(ptr) };
                    }
                }
            }
            curr_block = block.prev;
        }
    }
    /// Original function: gfdFreeListFree
    pub fn remove(&mut self, data: *mut T) {
        let mut this = (unsafe { &mut *(&raw const self.mutex as *mut GfdMutex) }).lock(self);
        (&mut *this).remove_locked(data)
    }
}

#[allow(dead_code)]
impl<T, A> FreeList<T, A>
where T: PartialEq,
      A: Allocator + Clone
{
    fn contains(&self, data: &T) -> bool {
        for v in self {
            if v == data { return true }
        }
        false
    }

    fn index_of(&self, data: &T) -> Option<usize> {
        let mut index = 0;
        let mut curr_block = self.tail;
        while let Some(v) = curr_block {
            let block = unsafe { v.as_ref() };
            for _ in 0..self.entries_per_block {
                let block_index = index % self.entries_per_block as usize;
                if unsafe { block.get_free_indices(block_index >> 3) 
                & 1 << (7 - block_index % u8::BITS as usize) != 0 } {
                    let curr = unsafe { &*(self.get_block_entry_ptr(block, block_index)) };
                    if curr == data { return Some(index) }
                }
                index += 1;
            }
            curr_block = block.prev;
        }
        None
    }
}

impl<T, A> FreeList<T, A> 
where T: 'static,
      A: Allocator + Clone + 'static
{
    // 0x1411b0540
    pub fn new_in(alloc: A) -> &'static mut Self {
        Self::new_in_with_alignment(alloc, align_of::<T>() as u32)
    }
    pub fn new_in_with_alignment(alloc: A, alignment: u32) -> &'static mut Self {
        Self::new_inner(alignment, 
            DEFAULT_FREE_LIST_ENTRIES_PER_BLOCK, 
            DEFAULT_FREE_LIST_BLOCKS, 
            DEFAULT_MEMORY_HINT, alloc)
    }

    fn allocate(this: Self) -> &'static mut Self {
        let layout = Layout::new::<Self>();
        let mut inner: NonNull<Self> = this._allocator.allocate(layout).unwrap().cast();
        // SAFETY: No existing instance of T lives here
        unsafe { inner.write(this); }
        unsafe { inner.as_mut() }
    }

    // 0x1410fbe70
    /// Original function: gfdFreeListCreate
    pub fn new_inner(
        alignment: u32,
        entries_per_block: u32,
        prealloc_blocks: u32,
        hint: u32,
        alloc: A
    ) -> &'static mut Self {
        unsafe { Self::new_inner_untyped(
            size_of::<T>() as u32, alignment, entries_per_block, prealloc_blocks, hint, alloc
        )}
    }

    // 0x1410fbe70
    /// Original function: gfdFreeListCreate
    pub unsafe fn new_inner_untyped(
        entry_size: u32,
        alignment: u32,
        entries_per_block: u32,
        prealloc_blocks: u32,
        hint: u32,
        alloc: A
    ) -> &'static mut Self {
        let alignment = match alignment {
            0 => 0x10,
            _ => if alignment as usize > align_of::<T>() { alignment } else { align_of::<T>() as u32 }
        };
        let heap_size = entries_per_block + 7 >> 3;
        let entry_size = entry_size as u32 - 1 + alignment & !(alignment - 1); // alignof
        let new_list = Self::allocate(Self {
            flags: FreeListFlags::MALLOC_BLOCKS,
            heap_size,
            entry_size,
            entries_per_block,
            alignment,
            head: None,
            tail: None,
            mutex: GfdMutex::new(),
            hint,
            prev: None,
            next: None,
            _allocator: alloc.clone()
        });
        new_list.add_new_blocks(prealloc_blocks);
        new_list
    }
}

impl<T, A> Drop for FreeList<T, A>
where A: Allocator + Clone 
{
    fn drop(&mut self) {
        let mut block = self.head;
        while let Some(v) = block {
            block = unsafe { v.as_ref() }.next;
            unsafe { self._allocator.deallocate(v.cast(), self.get_block_layout()); }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FreeListBlockLink<T> 
{
    prev: Option<NonNull<FreeListBlockLink<T>>>,
    next: Option<NonNull<FreeListBlockLink<T>>>,
    _data: std::marker::PhantomData<T>
}

impl<T> FreeListBlockLink<T> {
    pub(super) unsafe fn get_free_indices_ptr(&self, index: usize) -> *mut u8 {
        ((&raw const *self).add(1) as *mut u8).add(index)
    }
    pub(super) unsafe fn get_free_indices(&self, index: usize) -> u8 {
        *self.get_free_indices_ptr(index)
    }
    pub(super) unsafe fn is_index_byte_full(&self, index: usize) -> bool {
        self.get_free_indices(index) == 0xff
    }
    pub(super) unsafe fn is_entry_used(&self, index: usize) -> bool {
        self.get_free_indices(index >> 3) & (1 << 7 - (index % u8::BITS as usize)) != 0
    }
}

pub struct FreeListIterator<'a, T, A>
where A: Allocator + Clone
{
    list: &'a FreeList<T, A>,
    curr: Option<&'a FreeListBlockLink<T>>,
    entries_per_block: usize,
    current_block_entry: usize
}
impl<'a, T, A> FreeListIterator<'a, T, A> 
where A: Allocator + Clone
{
    fn next_block(&mut self) -> bool {
        self.curr.take().map(|v| {
            self.curr = v.prev.map(|v| unsafe { v.as_ref() });
            self.current_block_entry = 0;
        });
        self.curr.is_some()
    }
}

impl<'a, T, A> Iterator for FreeListIterator<'a, T, A>
where A: Allocator + Clone
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // early return for empty Free Lists
        if self.curr.is_none() { return None; }
        if self.current_block_entry == self.entries_per_block
        && !self.next_block() { return None; }
        // find next non-null entry if current entry is
        while unsafe { !self.curr.as_ref().unwrap()
            .is_entry_used(self.current_block_entry) } {
            if self.current_block_entry == self.entries_per_block {
                if !self.next_block() { return None; }
            } else { self.current_block_entry += 1; }
        }
        let out = unsafe { &*(self.list.get_block_entry_ptr(
            self.curr.unwrap(), self.current_block_entry)) };
        self.current_block_entry += 1;
        Some(out)
    }
}

impl<'a, T, A> IntoIterator for &'a FreeList<T, A>
where A: Allocator + Clone 
{
    type Item = &'a T;
    type IntoIter = FreeListIterator<'a, T, A>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: self,
            curr: self.tail.map(|v| unsafe { v.as_ref() }),
            entries_per_block: self.entries_per_block as usize,
            current_block_entry: 0
        }
    }
}

pub struct FreeListIteratorMut<'a, T, A>
where A: Allocator + Clone
{
    list: &'a FreeList<T, A>,
    curr: Option<&'a mut FreeListBlockLink<T>>,
    entries_per_block: usize,
    current_block_entry: usize
}
impl<'a, T, A> FreeListIteratorMut<'a, T, A> 
where A: Allocator + Clone
{
    fn next_block(&mut self) -> bool {
        self.curr.take().map(|v| {
            self.curr = v.prev.map(|mut v| unsafe { v.as_mut() });
            self.current_block_entry = 0;
        });
        self.curr.is_some()
    }
}

impl<'a, T, A> Iterator for FreeListIteratorMut<'a, T, A>
where A: Allocator + Clone
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // early return for empty Free Lists
        if self.curr.is_none() { return None; }
        if self.current_block_entry == self.entries_per_block
        && !self.next_block() { return None; }
        // find next non-null entry if current entry is
        while unsafe { !self.curr.as_ref().unwrap()
            .is_entry_used(self.current_block_entry) } {
            if self.current_block_entry == self.entries_per_block {
                if !self.next_block() { return None; }
            } else { self.current_block_entry += 1; }
        }
        let out = unsafe { &mut *(self.list.get_block_entry_ptr(
            self.curr.as_ref().unwrap(), self.current_block_entry)) };
        self.current_block_entry += 1;
        Some(out)
    }
}

impl<'a, T, A> IntoIterator for &'a mut FreeList<T, A>
where A: Allocator + Clone 
{
    type Item = &'a mut T;
    type IntoIter = FreeListIteratorMut<'a, T, A>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: self,
            curr: self.tail.map(|mut v| unsafe { v.as_mut() }),
            entries_per_block: self.entries_per_block as usize,
            current_block_entry: 0
        }
    }
}

#[cfg(test)]
pub mod tests {
    use allocator_api2::alloc::Global;
    use crate::tests::TestReturn;
    use super::{ FreeList, FreeListBlockLink };
    use std::fmt:: { Display, Debug };

    impl<T> FreeListBlockLink<T> {
        fn check_free_indices_for_block(&self, index: usize, expected: u8) {
            let byte_mask = unsafe { self.get_free_indices(index) };
            assert!(byte_mask == expected, 
                "Byte {} expected a mask value 0x{:x}, but got 0x{:x} instead",
                index, expected, byte_mask);
        }
    }

    impl<T> FreeList<T, Global>
    where T: PartialEq + Display + Debug
    {
        fn check_entries_by_iterator(&self, values: &[T]) {
            // for (i, v) in self.iter().enumerate() {

            // }
            for i in 0..values.len() {
                match self.get(i) {
                    Some(v) => assert!(*v == values[i], "At index {}, expected {} instead of {}", i, values[i], *v),
                    None => assert!(false, "Free list should have a value at {}, got null instead", i)
                };
            }
        }

        fn check_maybe_entries_by_iterator(&self, values: &[Option<T>]) {
            for i in 0..values.len() {
                assert!(self.get(i) == values[i].as_ref(), "At index {}, expected {:?} instead of {:?}",
                i, values[i], self.get(i));
            }
        }

        fn check_free_indices_by_iterator(&self, mask: &[u8]) {
            if self.tail.is_none() && mask.len() > 0 {
                assert!(false, "Free List is empty but mask contains values");
            }
            let mut curr_block = unsafe { self.tail.unwrap().as_ref() };
            for i in 0..mask.len() {
                let actual = unsafe { curr_block.get_free_indices(i % self.heap_size as usize) };
                assert!(actual == mask[i], "At index {}, expected {} instead of {}", i, mask[i], actual);
                if i + 1 < mask.len() 
                && (i + 1) % self.heap_size as usize == 0 {
                    curr_block = match curr_block.prev {
                        Some(v) => unsafe { v.as_ref() },
                        None => {
                            assert!(false, "Expected another block but got null instead");
                            unsafe { curr_block.prev.unwrap().as_ref() }
                        }
                    };
                }
            }
        }
    }

    fn create_expected_free_list_mask(len: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity((len + 7) / u8::BITS as usize);
        unsafe { std::ptr::write_bytes(out.as_mut_ptr(), 0xff, len / u8::BITS as usize); }
        unsafe { out.set_len(len / u8::BITS as usize) };
        let last = {
            let mut val = 0x0;
            for i in 0..len % 8 {
                val |= 1 << (7 - i);
            }
            val
        };
        out.push(last);
        out
    }

    #[test]
    fn create_free_list() -> TestReturn {
        let new_free_list: &mut FreeList<u32, Global> = FreeList::new_in(Global);
        for i in 0..12 {
            let curr_entry = new_free_list.add();
            unsafe { *curr_entry = i * 2 };
        }
        let first_block = new_free_list.get_block(0).unwrap();
        assert!(unsafe { first_block.is_index_byte_full(0) }, "First byte of free list mask should be true");
        first_block.check_free_indices_for_block(1, 0xf0);
        assert!(new_free_list.len() == 12, "Length should be 12 instead of {}", new_free_list.len());
        new_free_list.check_entries_by_iterator(&[0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22]);
        // drop it
        let _ = unsafe { std::ptr::read(&raw const *new_free_list) };
        Ok(())
    }

    #[test]
    fn multiple_list_blocks() -> TestReturn {
        let new_free_list: &mut FreeList<u32, Global> = FreeList::new_in(Global);
        for i in 0..303 {
            let curr_entry = new_free_list.add();
            unsafe { *curr_entry = i + 1 };
        }
        let expected = std::array::from_fn::<u32, 303, _>(|v| v as u32 + 1);
        assert!(new_free_list.len() == 303, "Length should be 303 instead of {}", new_free_list.len());
        new_free_list.check_entries_by_iterator(&expected);
        let mask_chk = create_expected_free_list_mask(new_free_list.len());
        new_free_list.check_free_indices_by_iterator(mask_chk.as_slice());
        // drop it
        let _ = unsafe { std::ptr::read(&raw const *new_free_list) };
        Ok(())
    }

    #[test]
    fn free_list_iterator() -> TestReturn {
        let new: &mut FreeList<u32, Global> = FreeList::new_in(Global);
        for i in 0..303 {
            let curr_entry = new.add();
            unsafe { *curr_entry = i + 1 };
        }
        let expected = std::array::from_fn::<u32, 303, _>(|v| v as u32 + 1);
        let values: Vec<_> = new.iter().collect();
        for i in 0..303 {
            assert!(*values[i] == expected[i], "In index {}, expected {} instead of {}",
                i, expected[i], *values[i]);
        }
        for i in 0..37 {
            assert!(new.contains(&(i * 8 + 1)), "Free list should contain the value {}", i * 8 + 1);
        }
        for i in 0..37 {
            let real = new.index_of(&(i * 8 + 1));
            assert!(real == Some(i as usize * 8), "Index of {} should be {} instead of {:?}",
                i * 8 + 1, i * 8, real);
        }
        let _ = unsafe { std::ptr::read(&raw const *new) };
        Ok(())
    }

    #[test]
    fn free_list_remove() -> TestReturn {
        let new: &mut FreeList<u32, Global> = FreeList::new_in(Global);
        let mut element_to_remove = std::ptr::null_mut(); // 8
        for i in 0..12 {
            let curr_entry = new.add();
            if i == 4 { element_to_remove = curr_entry }
            unsafe { *curr_entry = i * 2 }
        }
        assert!(new.len() == 12, "Length before removing element should be 12");
        new.remove_locked(element_to_remove);
        assert!(new.len() == 11, "Length after removing element should be 12");
        new.check_maybe_entries_by_iterator(&[
            Some(0), Some(2), Some(4), Some(6), None, Some(10), Some(12),
            Some(14), Some(16), Some(18), Some(20), Some(22)
        ]);
        let new_entry = new.add();
        unsafe { *new_entry = 24; }
        assert!(new.len() == 12, "Length after reinsertion should be 12");
        new.check_maybe_entries_by_iterator(&[
            Some(0), Some(2), Some(4), Some(6), Some(24), Some(10), Some(12),
            Some(14), Some(16), Some(18), Some(20), Some(22)
        ]);
        let _ = unsafe { std::ptr::read(&raw const *new) };
        Ok(())
    }
}

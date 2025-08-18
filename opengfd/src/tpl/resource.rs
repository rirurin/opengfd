use allocator_api2::alloc::Allocator;
use cpp_types::msvc::{
    string::String as CppString,
    shared_ptr::{
        SharedPtr, 
        WeakPtr 
    },
    vector::Vector
};

use crate::device::ngr::allocator::AllocatorHook;
use std::{
    marker::PhantomData,
    mem::MaybeUninit
};

// found in FUN_1404366c0
#[repr(C)]
#[derive(Debug)]
pub struct ResourceTimestamp<A = AllocatorHook>
where A: Allocator + Clone
{
    field00: bool,
    field08: u64,
    field10: Vector<u8, A>,
    _allocator: A
}

impl<A> ResourceTimestamp<A>
where A: Allocator + Clone
{
    pub fn new(alloc: A) -> Self {
        Self {
            field00: false,
            field08: 0,
            field10: Vector::new_in(alloc.clone()),
            _allocator: alloc
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Resource<S, A = AllocatorHook>
where A: Allocator + Clone
{
    owner: MaybeUninit<WeakPtr<Self, A>>,
    stream_type: StreamType,
    load_state_sprite_entry: LoadState,
    field18: i32,
    field1c: i32,
    filename: CppString<u8, A>,
    stream: *mut u8,
    // SharedPtr<(fn(*const u8) -> (), *const u8)>
    stream_owner: *mut SharedPtr<usize, A>,
    size: usize,
    sprite_entry_stream: *mut u8,
    load_state: LoadState,
    field64: u32,
    timestamp: ResourceTimestamp<A>,
    _allocator: A,
    _stream_fmt: PhantomData<S>
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadState {
    Uninitialized = 0,
    Initialized = 1, // CRIFSLOADER_STATUS_STOP
    Loading = 2, // CRIFSLOADER_STATUS_LOADING
    Ready = 3, // CRIFSLOADER_STATUS_COMPLETE
    Error = 4 // CRIFSLOADER_STATUS_ERROR
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamType {
    None = 0,
    GfdFile = 1,
    SpriteAPKPack = 2,
    SpriteAPKEntry = 3
}

#[repr(C)]
pub struct FieldOffsetStructure {
    // from the beginning of the file stream:
    header_size: u16, // in words
    field_count: u16,
    // entries: [[FieldOffsetField; entry_count] ; (header_size - 2) / entry_count]
}

impl FieldOffsetStructure {
    pub(super) fn get_entry_count(&self) -> u16 { (self.header_size - std::mem::size_of::<Self>() as u16 / self.field_count) / 2 }
    pub(super) unsafe fn get_field(&self, index: usize, field: usize) -> &[u8] {
        if index >= self.get_entry_count() as usize { &[] } 
        else {
            let field_ptr = &*(&raw const *self as *const FieldOffsetField).add(1 + index * self.field_count as usize + field);
            let slice_start = (&raw const *self as *const u8).add((self.header_size as usize * 2) + field_ptr.offset as usize);
            std::slice::from_raw_parts(slice_start, field_ptr.size as usize)
        }
    }
    pub(super) unsafe fn get_field_mut(&mut self, index: usize, field: usize) -> &mut [u8] {
        if index >= self.get_entry_count() as usize { &mut [] } 
        else {
            let field_ptr = &*(&raw const *self as *const FieldOffsetField).add(1 + index * self.field_count as usize + field);
            let slice_start = (&raw const *self as *mut u8).add((self.header_size as usize * 2) + field_ptr.offset as usize);
            std::slice::from_raw_parts_mut(slice_start, field_ptr.size as usize)
        }
    }
}

#[repr(C)]
pub struct FieldOffsetField {
    offset: u16,
    size: u16
}

impl<S, A> Resource<S, A>
where A: Allocator + Clone
{
    pub fn get_stream_size(&self) -> usize { self.size }
    pub fn get_raw_stream(&self) -> *const u8 { self.stream }

    pub fn get_stream(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.get_raw_stream(), self.get_stream_size()) }
    }

    pub fn get_stream_mut(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.get_raw_stream() as *mut u8, self.get_stream_size()) }
    }

    pub fn get_stream_type(&self) -> StreamType { self.stream_type }

    pub fn get_data(&self) -> Option<&S> {
        unsafe { (self.get_raw_stream() as *const S).as_ref() }
    }
    pub fn get_data_mut(&self) -> Option<&mut S> {
        unsafe { (self.get_raw_stream() as *mut S).as_mut() }
    }

    // 0x1414725b0
    /// Original function: tplResourceIsReady
    pub fn is_ready(&self) -> bool {
        match self.get_stream_type() {
            StreamType::SpriteAPKEntry => self.load_state_sprite_entry == LoadState::Ready,
            _ => self.load_state == LoadState::Ready
        }
    }

    pub fn get_load_state(&self) -> LoadState {
        match self.get_stream_type() {
            StreamType::SpriteAPKEntry => self.load_state_sprite_entry,
            _ => self.load_state
        }
    }

    // 0x141470010
    /// Original function: TPL::Resource::Initialize
    pub fn new(filename: CppString<u8, A>, alloc: A) -> Self {
        Self {
            owner: MaybeUninit::uninit(),
            // this: std::ptr::null_mut(),
            // owner: std::ptr::null_mut(),
            stream_type: StreamType::None,
            load_state_sprite_entry: LoadState::Uninitialized, 
            field18: 1,
            field1c: 0,
            filename,
            stream: std::ptr::null_mut(),
            stream_owner: std::ptr::null_mut(),
            size: 0,
            sprite_entry_stream: std::ptr::null_mut(),
            load_state: LoadState::Uninitialized,
            field64: 0,
            timestamp: ResourceTimestamp::new(alloc.clone()),
            _allocator: alloc.clone(),
            _stream_fmt: PhantomData
        }
    }

    // pub(super) fn set_this_ptr(&mut self, this: *mut Self) { self.this = this }
    // pub(super) fn set_owner(&mut self, this: *mut RefCountObject<Self, A>) { self.owner = this }

    pub fn get_filename(&self) -> &CppString<u8, A> { &self.filename }
    pub fn zero_timestamp(&mut self) { self.timestamp.field08 = 0; }
    // pub fn get_owner(&self) -> usize { self.owner as usize }

    pub fn set_owner(&mut self, ptr: WeakPtr<Self, A>) { self.owner = MaybeUninit::new(ptr); }
    pub fn get_owner(&self) -> &WeakPtr<Self, A> {
        unsafe { self.owner.assume_init_ref() }
    }
}

impl<A> Resource<FieldOffsetStructure, A>
where A: Allocator + Clone
{
    pub fn get_entry_count(&self) -> usize {
        let offsets = unsafe { &*(self.get_raw_stream() as *const FieldOffsetStructure) };
        offsets.get_entry_count() as usize
    }
    pub fn get_field_count(&self) -> usize {
        let offsets = unsafe { &*(self.get_raw_stream() as *const FieldOffsetStructure) };
        offsets.field_count as usize
    }
    pub fn get_field_as_slice(&self, index: usize, field: usize) -> &[u8] {
        if !self.is_ready() { &[] }
        else {
            let offsets = unsafe { &*(self.get_raw_stream() as *const FieldOffsetStructure) };
            unsafe { offsets.get_field(index, field) }
        }  
    }
    pub fn get_field_as_slice_mut(&mut self, index: usize, field: usize) -> &mut [u8] {
        if !self.is_ready() { &mut [] }
        else {
            let offsets = unsafe { &mut *(self.get_raw_stream() as *mut FieldOffsetStructure) };
            unsafe { offsets.get_field_mut(index, field) }
        }  
    }
    pub fn get_field_as_type<T>(&self, index: usize, field: usize) -> T {
        let slice = self.get_field_as_slice(index, field);
        assert!(slice.len() == std::mem::size_of::<T>(), "type does not match slice size");
        unsafe { std::ptr::read(slice.as_ptr() as *const T) }
    }
    pub fn set_field_as_type<T>(&mut self, index: usize, field: usize, value: T) {
        let slice = self.get_field_as_slice_mut(index, field);
        assert!(slice.len() == std::mem::size_of::<T>(), "type does not match slice size");
        unsafe { std::ptr::write(slice.as_ptr() as *mut T, value) };
    }
    pub fn get_field_as_ref<T>(&self, index: usize, field: usize) -> &T {
        let slice = self.get_field_as_slice(index, field);
        assert!(slice.len() == std::mem::size_of::<T>(), "type does not match slice size");
        unsafe { &*(slice.as_ptr() as *const T) }
    }
    pub fn get_field_as_str(&self, index: usize, field: usize) -> &str {
        let slice = self.get_field_as_slice(index, field);
        // remove last character, this is usually a null terminator
        unsafe { &std::str::from_utf8_unchecked(slice)[..slice.len() - 1] }
    }
}

#[cfg(test)]
// FieldOffsetStructure tests
pub mod tests {
    use super::FieldOffsetStructure;
    use std::{
        error::Error,
        fs,
        path::PathBuf
    };
    type TestReturn = Result<(), Box<dyn Error>>;

    fn assert_byte_slices(produced: &[u8], expected: &[u8]) {
        assert!(produced == expected, "Incorrect slice: got {:?} instead of {:?}", produced, expected);
    }

    #[test]
    pub fn test_single_entry_field_offset() -> TestReturn {
        let file = fs::read(PathBuf::from("E:\\Metaphor\\base_cpk\\COMMON\\game\\define\\GameDefine_Data.bin")).unwrap();
        let stream = unsafe { &*(file.as_ptr() as *const FieldOffsetStructure) };
        assert_byte_slices(unsafe { stream.get_field(0, 1)}, &[150, 0, 0, 0][..]);
        assert_byte_slices(unsafe { stream.get_field(0, 47)}, &[0x7f, 0x96, 0x98, 0][..]);
        assert_byte_slices(unsafe { stream.get_field(0, 53)}, &[] );
        Ok(())
    }
}

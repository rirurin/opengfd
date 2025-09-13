// See https://github.com/angryzor/rangers-api/blob/main/rangers-api/Hedgehog/Framework/LocalHeap.h

use std::ffi::CStr;
use std::ptr::NonNull;
use crate::device::hedge::{
    fnd::{
        allocator::MemoryRouter,
        free_list::FreeListHeapTemplate,
        heap_base::HeapBase,
        mutex::Mutex
    },
    ut::{
        array::InplaceArray,
        pair::Pair
    }
};

#[repr(C)]
struct SystemMemoryParam {
    unk1: usize,
    system_heap_size: usize,
    unk3: usize,
    unk4: usize,
    main_heap_size: usize,
    cri_system_heap_size: usize,
}

#[repr(C)]
struct MemoryParam {
}

#[repr(C)]
struct HeapDefinition {
    id: u32,
    name: *const i8,
    heap_size: usize
}

impl HeapDefinition {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }
    pub fn get_heap_size(&self) -> usize {
        self.heap_size
    }
}

#[repr(C)]
struct LocalHeap {
    base_heap: FreeListHeapTemplate<Mutex>,
    heaps: InplaceArray<Pair<u32, NonNull<HeapBase>>, 8>,
    memory_router: Option<NonNull<MemoryRouter>>,
    system_memory_param: Option<NonNull<SystemMemoryParam>>,
    memory_param: Option<NonNull<MemoryParam>>,
    unk_param: usize,
}
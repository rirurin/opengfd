use crate::device::hedge::fnd::heap_base::HeapBase;

type IndexType = u32;

#[repr(C)]
pub struct FreeListLikeHeapBase {
    _super: HeapBase,
    buffer_start: usize,
    buffer_end: usize,
    unk103: usize,
    unk104: usize,
    unk105: usize,
    unused: IndexType,
    unallocated: IndexType,
    live_allocations: u32,
    total_allocations: u32,
    unk109: u32,
    unk110: u32
}

#[repr(C)]
pub struct FreeListHeapBase {
    _super: FreeListLikeHeapBase,
    initialized: bool
}

#[repr(C)]
pub struct FreeListHeapTemplate<T = ()> {
    _super: FreeListHeapBase,
    data: T
}
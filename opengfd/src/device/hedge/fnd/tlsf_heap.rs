use crate::device::hedge::fnd::heap_base::HeapBase;

#[repr(C)]
pub struct BlockHead {
    unk1: usize,
    unk2: usize,
    unk3: usize,
    unk4: usize,
    unk5: usize,
    unk6: [u32; 25],
    unk7: [[*mut u8; 25]; 32]
}

#[repr(C)]
pub struct TlsfHeapBase {
    _super: HeapBase,
    buffer_begin: *mut u8,
    buffer_end: *mut u8,
    live_allocations: u32,
    total_allocations: u32,
    unk102: usize,
    unk103: usize,
    block_count: u32,
    block: usize,
    initialized: bool
}

#[repr(C)]
pub struct TlsfHeapTemplate<T = ()> {
    _super: TlsfHeapBase,
    data: T
}
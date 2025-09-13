use std::ptr::NonNull;
use crate::device::hedge::ut::link_list::{LinkListImpl, LinkListNode};

#[repr(C)]
pub struct HeapBase {
    _cpp_vtable: *const u8,
    unk0: *const u8,
    unk1: bool,
    name: [i8; 0x10],
    unk2: usize,
    parent: Option<NonNull<Self>>,
    children: LinkListImpl<Self>,
    children_node: LinkListNode<Self>,
    initialized: bool,
    unk11: u32,
    unk12: u16,
    debug_fill_on_alloc: bool,
    debug_fill_on_free: bool,
    debug_unk: bool,
    unk14: usize,
    unk15: usize,
    unk16: u32
}
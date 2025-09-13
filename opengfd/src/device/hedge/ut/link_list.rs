use std::marker::PhantomData;
use std::ptr::NonNull;

#[repr(C)]
pub struct LinkListImpl<T = ()> {
    count: u32,
    node_offset: u32,
    p_end: Option<NonNull<LinkListNode<T>>>,
    p_root: Option<NonNull<LinkListNode<T>>>,
}

#[repr(C)]
pub struct LinkListNode<T = ()> {
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    data: PhantomData<T>
    // data: T
}

#[repr(C)]
pub struct LinkList<T = ()> {
    _super: LinkListImpl<T>,
    cur_node: Option<NonNull<LinkListNode<T>>>,
    node_offset: u32
}
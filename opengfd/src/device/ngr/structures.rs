use crate::device::ngr::hint::MemHint;
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
pub struct PointerList<V> {
    _cpp_vtable: *mut u8,
    _head: *mut V,
    _tail: *mut V,
    size: usize,
    hint: MemHint
}

#[repr(C)]
#[derive(Debug)]
pub struct PointerListEntry<V> {
    next: *mut PointerListEntry<V>,
    prev: *mut PointerListEntry<V>,
    data: *mut V,
}

use crate::utility::{
    name::Name,
    reference::Reference
};
use std::marker::PhantomPinned;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AssetTypeHandle(*mut u8);

#[repr(C)]
#[derive(Debug)]
pub struct Asset {
    type_: u32,
    data: *mut u8,
    size: u32,
    handle: AssetTypeHandle,
    dirty: u32,
    access: u32,
    attribute: u32,
    name: Name,
    length: u32,
    ref_: Reference,
    prev: *mut Asset,
    next: *mut Asset,
    _pinned: PhantomPinned
}

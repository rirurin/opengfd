use crate::{
    kernel::asset::{ Asset, AssetTypeHandle },
    utility::reference::Reference
};

#[repr(C)]
#[derive(Debug)]
pub struct EPLMaterial {
    asset: *mut Asset,
    handle: *mut AssetTypeHandle
}

#[repr(C)]
#[derive(Debug)]
pub struct EPLParameter {
    data: *mut u8,
    size: u32,
    ref_: Reference
}

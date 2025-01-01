use crate::{
    graphics::shader::shader::ShaderID,
    utility::reference::Reference
};
use std::marker::PhantomPinned;

#[repr(C)]
#[derive(Debug)]
pub struct PixelShader {
    field00: i32,
    id: ShaderID,
    data: *mut u8,
    ref_: Reference,
    prev: *mut PixelShader,
    next: *mut PixelShader,
    _pinned: PhantomPinned
}

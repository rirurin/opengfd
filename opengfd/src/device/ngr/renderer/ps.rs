use crate::{
    device::ngr::renderer::vs::VertexShaderPlatform,
    graphics::shader::shader::ShaderID,
    utility::reference::Reference
};
use std::marker::PhantomPinned;

#[repr(C)]
#[derive(Debug)]
pub struct PixelShader {
    field00: i32,
    id: ShaderID,
    pub data: *mut VertexShaderPlatform,
    ref_: Reference,
    prev: *mut PixelShader,
    next: *mut PixelShader,
    _pinned: PhantomPinned
}

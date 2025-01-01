use riri_mod_tools_proc::ensure_layout;

use crate::{
    graphics::shader::shader::ShaderID,
    utility::reference::Reference
};
use std::marker::PhantomPinned;

use windows::Win32::Graphics::Direct3D11::{
    ID3D11InputLayout,
    ID3D11PixelShader,
    ID3D11VertexShader,
};

#[repr(C)]
#[derive(Debug)]
pub struct VertexShader {
    field00: i32,
    id: ShaderID,
    pub data: *mut VertexShaderPlatform,
    ref_: Reference,
    prev: *mut VertexShader,
    next: *mut VertexShader,
    _pinned: PhantomPinned
}
/*
impl VertexShader {
    pub(crate) fn get_data(&self) -> { self.data.is_some() }
}
*/

#[derive(Debug)]
#[ensure_layout(size = 0x88)]
pub struct VertexShaderPlatform {
    #[field_offset(0x68)] pub d3d_pixel: ID3D11PixelShader,
    #[field_offset(0x78)] pub d3d_vertex: ID3D11VertexShader,
    #[field_offset(0x80)] pub d3d_input_layout: ID3D11InputLayout
}

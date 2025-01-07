/*
// use riri_mod_tools_proc::ensure_layout;

use crate::{
    device::ngr::structures::StringHashed,
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
/*
#[repr(C)]
#[derive(Debug)]
// #[ensure_layout(size = 0x88)]
pub struct VertexShaderPlatform {
    _unk: [u8; 0x68],
    /*#[field_offset(0x68)]*/ pub(super) d3d_pixel: Option<ID3D11PixelShader>,
    _unk1: [u8; 0x8],
    /*#[field_offset(0x78)]*/ pub(super) d3d_vertex: Option<ID3D11VertexShader>,
    /*#[field_offset(0x80)]*/ pub(super) d3d_input_layout: ID3D11InputLayout
}*/
#[repr(C)]
#[derive(Debug)]
pub struct VertexShaderPlatform {
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    name: StringHashed,
    field50: usize,
    field58: [u32; 3],
    field64: u32,
    field68: usize,
    field70: u32,
    d3d_vertex: Option<ID3D11VertexShader>,
    d3d_input_layout: Option<ID3D11InputLayout>
}

impl VertexShaderPlatform {
    pub fn get_d3d_pixel(&self) -> Option<&ID3D11PixelShader> {
        self.d3d_pixel.as_ref()
    }
    // 0x1411b28f0
    pub(crate) fn create_input_layout(&self, a2: *mut u32) -> usize {
        0
    }
}
*/

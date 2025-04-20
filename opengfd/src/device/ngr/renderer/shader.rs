use crate::{
    device::ngr::structures::StringHashed,
    graphics::shader::shader::ShaderID,
    utility::reference::Reference
};
use std::marker::PhantomPinned;
use windows::Win32::Graphics::Direct3D11::{
    ID3D11ComputeShader,
    ID3D11InputLayout,
    ID3D11GeometryShader,
    ID3D11PixelShader,
    ID3D11VertexShader
};
use windows::core::Interface;

#[repr(C)]
#[derive(Debug)]
pub struct ShaderPlatformBase {
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    name: StringHashed,
    field50: usize,
}

pub trait ShaderPlatform {
    type Shader;
    fn get_shader_ptr(&mut self) -> Option<*mut Option<Self::Shader>>;
    fn get_shader_ref(&self) -> Option<&Self::Shader>;
    fn get_shader_as_raw(&self) -> *mut std::ffi::c_void;
}

#[repr(C)]
#[derive(Debug)]
pub struct VertexShader {
    field00: i32,
    id: ShaderID,
    pub data: *mut VertexShaderPlatform,
    ref_: Reference,
    prev: *mut Self,
    next: *mut Self,
    _pinned: PhantomPinned
}

impl VertexShader {
    pub fn get_shader_id(&self) -> &ShaderID {
        &self.id
    }
    pub fn get_next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
    }
    pub fn get_prev(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VertexShaderPlatform {
    super_: ShaderPlatformBase,
    field58: [u32; 3],
    field64: u32,
    field68: usize,
    field70: u32,
    d3d_vertex: Option<ID3D11VertexShader>,
    d3d_input_layout: Option<ID3D11InputLayout>
}

impl VertexShaderPlatform {
    pub fn get_input_layout(&self) -> Option<&ID3D11InputLayout> {
        self.d3d_input_layout.as_ref()
    }
    pub fn get_vertex_shader_ptr(&self) -> *mut std::ffi::c_void {
        match &self.d3d_vertex { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

impl ShaderPlatform for VertexShaderPlatform {
    type Shader = ID3D11VertexShader;
    fn get_shader_ptr(&mut self) -> Option<*mut Option<Self::Shader>> {
        Some(&raw mut self.d3d_vertex)
    }
    fn get_shader_ref(&self) -> Option<&Self::Shader> {
        self.d3d_vertex.as_ref()
    }
    fn get_shader_as_raw(&self) -> *mut std::ffi::c_void { 
        match &self.d3d_vertex { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PixelShader {
    field00: i32,
    id: ShaderID,
    pub data: *mut PixelShaderPlatform,
    ref_: Reference,
    prev: *mut PixelShader,
    next: *mut PixelShader,
    _pinned: PhantomPinned
}

#[repr(C)]
#[derive(Debug)]
pub struct PixelShaderPlatform {
    super_: ShaderPlatformBase,
    field58: [u32; 3],
    field64: u32,
    d3d_pixel: Option<ID3D11PixelShader>,
}

impl ShaderPlatform for PixelShaderPlatform {
    type Shader = ID3D11PixelShader;
    fn get_shader_ptr(&mut self) -> Option<*mut Option<Self::Shader>> {
        Some(&raw mut self.d3d_pixel)
    }
    fn get_shader_ref(&self) -> Option<&Self::Shader> {
        self.d3d_pixel.as_ref()
    }
    fn get_shader_as_raw(&self) -> *mut std::ffi::c_void { 
        match &self.d3d_pixel { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GeometryShaderPlatform {
    super_: ShaderPlatformBase,
    field58: [u32; 3],
    field64: u32,
    d3d_geo: Option<ID3D11GeometryShader>
}

impl ShaderPlatform for GeometryShaderPlatform {
    type Shader = ID3D11GeometryShader;
    fn get_shader_ptr(&mut self) -> Option<*mut Option<Self::Shader>> {
        Some(&raw mut self.d3d_geo)
    }
    fn get_shader_ref(&self) -> Option<&Self::Shader> {
        self.d3d_geo.as_ref()
    }
    fn get_shader_as_raw(&self) -> *mut std::ffi::c_void { 
        match &self.d3d_geo { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GeometryShader {
    field00: i32,
    id: ShaderID,
    pub data: *mut GeometryShaderPlatform,
    ref_: Reference,
    prev: *mut GeometryShader,
    next: *mut GeometryShader,
    _pinned: PhantomPinned
}

#[repr(C)]
#[derive(Debug)]
pub struct ComputeShaderPlatform {
    super_: ShaderPlatformBase,
    field58: u32,
    d3d_cmp: Option<ID3D11ComputeShader>
}

impl ShaderPlatform for ComputeShaderPlatform {
    type Shader = ID3D11ComputeShader;
    fn get_shader_ptr(&mut self) -> Option<*mut Option<Self::Shader>> {
        Some(&raw mut self.d3d_cmp)
    }
    fn get_shader_ref(&self) -> Option<&Self::Shader> {
        self.d3d_cmp.as_ref()
    }
    fn get_shader_as_raw(&self) -> *mut std::ffi::c_void { 
        match &self.d3d_cmp { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ComputeShader {
    field00: i32,
    id: ShaderID,
    pub data: *mut ComputeShaderPlatform,
    ref_: Reference,
    prev: *mut ComputeShader,
    next: *mut ComputeShader,
    _pinned: PhantomPinned
}
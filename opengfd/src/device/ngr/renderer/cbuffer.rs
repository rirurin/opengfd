use crate::{
    device::ngr::hint::MemHint,
    utility::reference::Reference
};
use windows::Win32::Graphics::Direct3D11::ID3D11Buffer;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BufferType {
    Vertex = 0,
    Geometry = 1,
    Pixel = 2,
    Compute = 3
}

impl TryFrom<u32> for BufferType {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= BufferType::Compute as u32 {
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(())
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConstantBuffer {
    _cpp_vtable: usize,
    ref_: Reference,
    field10: usize,
    fields: CbufferFields,
    byte_width: u32,
    field54: u32,
    field58: u32,
    pub(super) slot: i32,
    d3d11_cbuffer: [Option<ID3D11Buffer>; 3],
    field78: i32,
    field7c: i32,
    pub(super) active_buffers: u32,
}

impl ConstantBuffer {
    pub fn vtable_3(&self) -> bool { self.field58 == 0 }
    pub unsafe fn vtable_21(&self, index: u32) -> &Option<ID3D11Buffer> {
        let real_index = if self.vtable_3() { 0 } else { index } as usize;
        unsafe { self.d3d11_cbuffer.get_unchecked(real_index + 1) }
    }
    pub unsafe fn get_buffer_unchecked(&self, index: usize) -> &Option<ID3D11Buffer> {
        self.d3d11_cbuffer.get_unchecked(index)
    }
    pub unsafe fn get_buffer_unchecked_mut(&mut self, index: usize) -> &mut Option<ID3D11Buffer> {
        self.d3d11_cbuffer.get_unchecked_mut(index)
    }
    pub fn get_active_buffers(&self) -> u32 { self.active_buffers }
}

#[repr(C)]
#[derive(Debug)]
pub struct CbufferFields {
    field0: usize,
    field8: usize,
    field10: usize,
    field18: usize,
    field20: usize,
    _hint: MemHint
}

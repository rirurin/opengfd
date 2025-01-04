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
    resource_count: u32,
    pub(super) slot: i32,
    buffer: Option<ID3D11Buffer>,
    resources: [*const std::ffi::c_void; 3],
    // pub(super) active_buffers: u32,
    pub active_buffers: u32,
}

impl ConstantBuffer {
    pub fn has_resources(&self) -> bool { self.resource_count == 0 }
    pub unsafe fn get_resource(&self, index: u32) -> *const std::ffi::c_void {
        let real_index = if self.has_resources() { 0 } else { index } as usize;
        unsafe { *self.resources.get_unchecked(real_index) }
    }
    pub unsafe fn get_buffer(&self) -> Option<&ID3D11Buffer> {
        self.buffer.as_ref()
    }
    pub unsafe fn get_buffer_as_slice(&self) -> &[Option<ID3D11Buffer>] {
        std::slice::from_raw_parts(&self.buffer, 1)
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

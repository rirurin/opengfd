use crate::{
    device::ngr::{
        allocator::AllocatorHook,
        // hint::MemHint,
        renderer::hint::{ BufferFieldType, BufferFieldRustType, BufferFieldHint },
        structures::{ Array, StringHashed }
    },
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
    _cpp_vtable: *mut std::ffi::c_void,
    ref_: Reference,
    field10: usize,
    // fields: CbufferFields,
    fields: Array<*mut ConstantBufferField, AllocatorHook>,
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
    pub fn get_resource_flag(&self, upd: u32) -> u32 {
        if self.has_resources() { 1 << (upd & 0x1f) } else { 1 }
    } 
    pub unsafe fn get_buffer(&self) -> Option<&ID3D11Buffer> {
        self.buffer.as_ref()
    }
    pub unsafe fn get_buffer_as_slice(&self) -> &[Option<ID3D11Buffer>] {
        std::slice::from_raw_parts(&self.buffer, 1)
    }
    pub fn get_active_buffers(&self) -> u32 { self.active_buffers }
    pub fn get_slot(&self) -> usize { self.slot as usize }

    // VTABLE ENTRIES
    // 0x1422a7790
    // vtable + 0x10
    pub fn has_platform_buffer(&self) -> bool { self.buffer.is_some() }
    // vtable + 0x18
    pub fn has_resources(&self) -> bool { self.resource_count != 0 }
    // vtable + 0x20
    // vtable + 0x28
    pub fn get_field_count(&self) -> usize { self.fields.get_length() }
    // vtable + 0xa0
    pub unsafe fn get_or_create_resource(&self, index: usize) -> *mut u8 {
        unsafe {
            let vtable_offset = self._cpp_vtable.add(0xa0);
            let state_func = *std::mem::transmute::<
                *mut std::ffi::c_void, 
                *const fn(&Self, u32) -> *mut u8
            >(vtable_offset);
            (state_func)(self, index as u32)
        }
    }
    // vtable + 0xb8
    pub unsafe fn get_resource(&self, index: u32) -> *const std::ffi::c_void {
        let real_index = if self.has_resources() { index } else { 0 } as usize;
        unsafe { *self.resources.get_unchecked(real_index) }
    }
    pub fn set_field<T>(&self, frame: usize, index: usize, value: T) -> bool 
    where T: BufferFieldRustType
    {
        if (unsafe { &*self.fields[index] }).ty != T::get_type() { return false; }
        unsafe { self.set_field_unchecked(frame, index, value); }
        true
    }
    pub unsafe fn set_field_unchecked<T>(&self, frame: usize, index: usize, value: T) {
        let data = self.get_or_create_resource(frame);
        let field = unsafe { &*self.fields[index] };
        std::ptr::write(data.add(field.offset as usize) as *mut T, value);
    }
}
/*
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
*/
#[repr(C)]
#[derive(Debug)]
pub struct ConstantBufferField {
    _cpp_vtable: usize,
    name: StringHashed,
    ty: BufferFieldType,
    offset: i32
}

// impl ConstantBufferField {
//     pub fn from_field_hint_array<const C: usize>(data: &[BufferFieldHint; C]) -> Array<ConstantBufferField, AllocatorHook> {
// 
//     }
// }

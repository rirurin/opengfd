use crate::{
    device::ngr::hint::MemHint,
    utility::reference::{ GfdRcType, Reference }
};
use opengfd_proc::GfdRcAuto;
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct ShaderBytecode {
    _cpp_vtable: *const u8,
    ref_: Reference,
    ptr: Option<NonNull<u8>>,
    size: usize,
    hint: MemHint
}

impl ShaderBytecode {
    pub fn new(shader: &'static [u8]) -> Self {
        Self {
            _cpp_vtable: std::ptr::null(),
            ref_: Reference::new(),
            ptr: if shader.len() > 0 {
                Some(NonNull::new(shader.as_ptr() as *mut u8).unwrap())
            } else { None },
            size: shader.len(),
            hint: MemHint::new_value(2)
        }
    }
    /*
    pub fn as_slice(&self) -> Option<&[u8]> {
        match self.ptr {
            Some(v) => Some(unsafe { std::slice::from_raw_parts(v.as_ptr(), self.size) } ),
            None => None
        }
    }
    */
    pub fn as_slice(&self) -> &[u8] {
        match self.ptr {
            Some(v) => unsafe { std::slice::from_raw_parts(v.as_ptr(), self.size) },
            None => &[]
        }
    }
}

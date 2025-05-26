use crate::{
    device::ngr::renderer::platform::d3d::TextureResource,
    utility::name::Name
};
use std::{
    ffi::c_void,
    ptr::NonNull
};

#[repr(C)]
pub struct Texture {
    pub(crate) flags: i32,
    pub(crate) handle: NonNull<TextureResource>,
    pub(crate) ref_: i32,
    pub(crate) name: Name,
    pub(crate) min: u8,
    pub(crate) mag: u8,
    pub(crate) wraps: u8,
    pub(crate) wrapt: u8,
    pub(crate) prev: *mut Texture,
    pub(crate) next: *mut Texture,
    pub(crate) flags2: i32,
}

impl Texture {
    pub unsafe fn get_handle(&self) -> &TextureResource { unsafe { self.handle.as_ref() } }
    pub unsafe fn get_handle_mut(&mut self) -> &mut TextureResource { unsafe { self.handle.as_mut() } }
    pub fn get_next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
    }
    pub fn get_prev(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref() }
    }
    pub fn get_name(&self) -> &Name { &self.name }

    pub fn get_width(&self) -> u32 { unsafe { self.handle.as_ref().get_width() } }
    pub fn get_height(&self) -> u32 { unsafe { self.handle.as_ref().get_height() } }
    pub fn get_raw(&self) -> *mut c_void { unsafe { self.handle.as_ref().get_raw() }}
}

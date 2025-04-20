use crate::{
    device::ngr::renderer::platform::d3d::TextureResource,
    utility::name::Name
};

#[repr(C)]
pub struct Texture {
    pub(crate) flags: i32,
    pub(crate) handle: *mut TextureResource,
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
    pub unsafe fn get_handle(&self) -> &TextureResource { &*self.handle }
    pub unsafe fn get_handle_mut(&mut self) -> &mut TextureResource { &mut *self.handle }
    pub fn get_next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
    }
    pub fn get_prev(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref() }
    }
    pub fn get_name(&self) -> &Name { &self.name }
}

use crate::{
    device::ngr::renderer::platform::d3d::TextureResource,
    utility::name::Name
};
use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 80usize)]
pub struct Texture {
    #[field_offset(0usize)]
    pub flags: i32,
    #[field_offset(8usize)]
    pub handle: *mut TextureResource,
    #[field_offset(16usize)]
    pub ref_: i32,
    #[field_offset(24usize)]
    pub name: Name,
    #[field_offset(48usize)]
    pub min: u8,
    #[field_offset(49usize)]
    pub mag: u8,
    #[field_offset(50usize)]
    pub wraps: u8,
    #[field_offset(51usize)]
    pub wrapt: u8,
    #[field_offset(56usize)]
    pub prev: *mut Texture,
    #[field_offset(64usize)]
    pub next: *mut Texture,
    #[field_offset(72usize)]
    pub flags2: i32,
}

impl Texture {
    pub unsafe fn get_handle(&self) -> &TextureResource { &*self.handle }
    pub unsafe fn get_handle_mut(&mut self) -> &mut TextureResource { &mut *self.handle }
}

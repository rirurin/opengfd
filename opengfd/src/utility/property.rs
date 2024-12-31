use super::{ mutex::Mutex, name::Name, reference::Reference };
use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 40usize)]
pub struct Property {
    #[field_offset(0usize)]
    pub flags: i16,
    #[field_offset(2usize)]
    pub access: i16,
    #[field_offset(8usize)]
    pub head: *mut PropertyChunk,
    #[field_offset(16usize)]
    pub tail: *mut PropertyChunk,
    #[field_offset(24usize)]
    pub mutex: Mutex,
    #[field_offset(32usize)]
    pub ref_: Reference,
}

#[ensure_layout(size = 72usize)]
pub struct PropertyChunk {
    #[field_offset(0usize)]
    pub type_: i32,
    #[field_offset(8usize)]
    pub name: Name,
    #[field_offset(32usize)]
    pub data: [u8; 16usize],
    #[field_offset(48usize)]
    pub size: i32,
    #[field_offset(56usize)]
    pub prev: *mut PropertyChunk,
    #[field_offset(64usize)]
    pub next: *mut PropertyChunk,
}

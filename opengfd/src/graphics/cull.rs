#[cfg(feature = "v1-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CullObject {
    todo: [u8; 0x20]
}

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct CullObject {
    object: u32,
    shadow: u32,
    reflect: u32,
    viewz: f32,
}

impl CullObject {
    pub fn new(object: u32, shadow: u32, reflect: u32, viewz: f32) -> Self {
        Self { object, shadow, reflect, viewz }
    }
    pub fn get_object(&self) -> u32 { self.object }
    pub fn get_shadow(&self) -> u32 { self.shadow }
    pub fn get_reflect(&self) -> u32 { self.reflect }
    pub fn get_view_z(&self) -> f32 { self.viewz }
}
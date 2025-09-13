use crate::kernel::graphics::GraphicsStateSteam;

pub(crate) const RENDER_LISTS: usize = 2;
pub(crate) const RENDER_STATES: usize = 33;
pub(crate) const SCENE_LISTS: usize = 2;
pub(crate) const MATERIAL_LISTS: usize = 8;
pub(crate) const SHADER_SOURCE: usize = 146;
pub(crate) const FIXED_VERTEX_SHADERS: usize = 48;
pub(crate) const FIXED_PIXEL_SHADERS: usize = 146;

#[repr(C)]
#[derive(Debug)]
pub struct Global { // (GlobalSteam, default state)
    controller_disconnected: Option<fn()>,
    callback_8: Option<fn()>,
    controller_connected: Option<fn()>,
    tick: u64,
    timestamp: u64,
    frames1: u64,
    graphics: GraphicsStateSteam,
    physics: [u8; 0x50],
    tasks: [u8; 0xa8],
    field2ce8: [u8; 0x1b8]
}
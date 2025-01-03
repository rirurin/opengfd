use crate::{
    device::ngr::renderer::state::BufferBlendMode,
    globals
};
use riri_mod_tools_proc::ensure_layout;

pub const BLEND_MODE_FUNCTIONS: [ [i32; 4]; 6 ] = [
    [ 1, 0, 1, 0 ],
    [ 4, 5, 4, 5 ],
    [ 4, 1, 4, 1 ],
    [ 4, 5, 4, 5 ],
    [ 4, 1, 4, 1 ],
    [ 4, 1, 4, 1 ],
];

#[repr(C)]
#[derive(Debug)]
pub struct BlendModePkt {
    cb: unsafe fn(*mut BlendModePktSetParams) -> (),
    blend_type: u32,
    field10: usize
}

impl BlendModePkt {
    pub fn new(blend_type: u32) -> &'static mut Self {
        let cmd_buffer = unsafe { globals::get_gfd_global_unchecked().graphics.get_current_cmd_buffer() };
        let new = unsafe { cmd_buffer.alloc_ex::<Self>() };
        new.cb = set_blend_mode_pkt;
        new.blend_type = blend_type;
        new.field10 = 0;
        new
    }
}

#[ensure_layout(size = 0x20)]
pub struct BlendModePktSetParams {
    #[field_offset(0x8)] data: *mut u8,
    #[field_offset(0x18)] buffer_index: i32
}

pub unsafe fn set_blend_mode_pkt(this: *mut BlendModePktSetParams) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    let blend_type = *((&*this).data.add(8) as *mut i32) as usize;
    let ivar9 = if blend_type == 5 { 2 } else { 0 };
    let new_blend = BufferBlendMode {
        field00: BLEND_MODE_FUNCTIONS[blend_type][0],
        field04: BLEND_MODE_FUNCTIONS[blend_type][1],
        field08: ivar9,
        field0c: BLEND_MODE_FUNCTIONS[blend_type][2],
        field10: BLEND_MODE_FUNCTIONS[blend_type][3],
        field14: ivar9,
    };
    let buf_id = (&*this).buffer_index as usize;
    if new_blend != draw_state.basicBuffers[buf_id].blend_mode {
        draw_state.basicBuffers[buf_id].blend_mode = new_blend;
        draw_state.basicBuffers[buf_id].flags |= 2;
    }
    (&mut *this).data = (&*this).data.add(16);
}

use crate::{
    device::ngr::renderer::state::{ 
        BlendKey,
        BlendType,
        BlendTypeOperation,
        BufferFlags
    },
    globals
};
use riri_mod_tools_proc::ensure_layout;

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

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BufferBlendMode {
    pub(crate) source_blend: BlendType,
    pub(crate) dest_blend: BlendType,
    pub(crate) blend_op: BlendTypeOperation,
    pub(crate) source_blend_alpha: BlendType,
    pub(crate) dest_blend_alpha: BlendType,
    pub(crate) blend_op_alpha: BlendTypeOperation,
}

impl BufferBlendMode {
    pub fn from_pkt_id(index: usize) -> Self {
        let op = if index == 5 { BlendTypeOperation::ReverseSubtract } else { BlendTypeOperation::Add };
        Self {
            source_blend: [ 
                BlendType::One, BlendType::SourceAlpha, BlendType::SourceAlpha,
                BlendType::SourceAlpha, BlendType::SourceAlpha, BlendType::SourceAlpha,
            ][index],
            dest_blend: [
                BlendType::Zero, BlendType::InverseSourceAlpha, BlendType::One,
                BlendType::InverseSourceAlpha, BlendType::One, BlendType::One
            ][index],
            blend_op: op,
            source_blend_alpha: [ 
                BlendType::One, BlendType::SourceAlpha, BlendType::SourceAlpha,
                BlendType::SourceAlpha, BlendType::SourceAlpha, BlendType::SourceAlpha,
            ][index],
            dest_blend_alpha: [
                BlendType::Zero, BlendType::InverseSourceAlpha, BlendType::One,
                BlendType::InverseSourceAlpha, BlendType::One, BlendType::One
            ][index],
            blend_op_alpha: op,
        }
    }
}

impl PartialEq<BlendKey> for BufferBlendMode {
    fn eq(&self, other: &BlendKey) -> bool {
        self.source_blend == other.source_blend &&
        self.dest_blend == other.dest_blend &&
        self.blend_op == other.blend_op &&
        self.source_blend_alpha == other.source_blend_alpha &&
        self.dest_blend_alpha == self.dest_blend_alpha &&
        self.blend_op_alpha == self.blend_op_alpha
    }
}

impl BlendKey {
    fn set_from_pkt(&mut self, pkt: BufferBlendMode) {
        self.source_blend = pkt.source_blend;
        self.dest_blend = pkt.dest_blend;
        self.blend_op = pkt.blend_op;
        self.source_blend_alpha = pkt.source_blend_alpha;
        self.dest_blend_alpha = pkt.dest_blend_alpha;
        self.blend_op_alpha = pkt.blend_op_alpha;
    }
}

pub unsafe fn set_blend_mode_pkt(this: *mut BlendModePktSetParams) {
    set_blend_key_preset((&*this).buffer_index as usize, *((&*this).data.add(8) as *mut i32) as usize);
    (&mut *this).data = (&*this).data.add(16);
}

pub unsafe fn set_blend_key_preset(buf_id: usize, blend_id: usize) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    let new_blend = BufferBlendMode::from_pkt_id(blend_id);
    if new_blend != draw_state.basicBuffers[buf_id].blend_key {
        draw_state.basicBuffers.get_unchecked_mut(buf_id).blend_key.set_from_pkt(new_blend);
        draw_state.basicBuffers.get_unchecked_mut(buf_id).flags |= BufferFlags::USING_BLEND;
    }
}

// 0x141090b90
// see gfdDeviceRenderEffectBrushStrokeFiltering
pub unsafe fn set_blend_key_preset2(buf_id: usize, blend_id: usize, set_blend_key: bool) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    if set_blend_key {
        set_blend_key_preset(buf_id, blend_id); 
    }
    let buf = &mut draw_state.basicBuffers.get_unchecked_mut(buf_id);
    if buf.blend_key.enable_blending != set_blend_key {
        buf.flags |= BufferFlags::USING_BLEND;
        buf.blend_key.enable_blending = set_blend_key;
    }
    let global = globals::get_gfd_global_unchecked_mut();
    if buf_id == 3 { *global.graphics.render_state_current.get_unchecked_mut(9) = set_blend_key as usize }
}

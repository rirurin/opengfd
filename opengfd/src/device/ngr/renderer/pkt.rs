#![allow(dead_code)]
use crate::{
    device::ngr::renderer::{
        cbuffer::BufferType,
        state::{
            BlendKey,
            BlendType,
            BlendTypeOperation,
            BufferFlags,
            ComparisonFunc,
            FilterMode,
            StencilOperation
        }
    },
    globals,
    graphics::texture::Texture
    // graphics::render::cmd_buffer::CmdBuffer
};

use super::state::DeferredContext;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct PktData<T>
where T: PktFunction
{
    field00: usize,
    data: *mut T,
    field10: usize,
    buffer_index: i32
}

pub(crate) trait PktFunction {
    fn as_raw(&self) -> *const Self;
}

impl<T> PktData<T>
where T: PktFunction
{
    fn allocate_for_pkt() -> &'static mut T {
        let cmd_buffer = unsafe { globals::get_gfd_global_unchecked().graphics.get_current_cmd_buffer() };
        let new = unsafe { cmd_buffer.alloc_type::<T>(std::mem::size_of::<usize>()) };
        unsafe {
            libc::memset(
                new.add(1) as *mut libc::c_void,
                0 as libc::c_int,
                std::mem::size_of::<usize>() as libc::size_t
            );
            &mut *new
        }
    }
    unsafe fn get_data(&self) -> &T { &*self.data }
    unsafe fn move_to_end(&mut self) {
        self.data = self.data.add(1);
    }
    unsafe fn move_to(&mut self, offset: usize) {
        self.data = (self.data as *mut u8).add(offset) as *mut T;
    }
}

// PKT Function List:
// gfdDevCmdMakeImmediateRenderIndexedPrimitivePkt (TODO)
// gfdDevCmdMakeImmediateRenderPrimitivePkt (TODO)
// gfdDevCmdMakeSetAlphaFuncPkt
// gfdDevCmdMakeSetAlphaTestEnablePkt
// gfdDevCmdMakeSetBlendFuncPkt
// gfdDevCmdMakeSetBlendModePkt (DONE)
// gfdDevCmdMakeSetBlendOpPkt
// gfdDevCmdMakeSetColorMaskPkt (DONE)
// gfdDevCmdMakeSetStencilFuncPkt (DONE)
// gfdDevCmdMakeSetStencilOpPkt (DONE)
// gfdDevCmdMakeSetStencilTestEnablePkt (DONE)
// gfdDevCmdMakeSetTexturePkt (TODO)

// SetBlendModePkt

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

#[repr(C)]
#[derive(Debug)]
pub struct BlendModePkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    blend_type: u32,
}

impl PktFunction for BlendModePkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl BlendModePkt {
    pub fn new(blend_type: u32) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<BlendModePkt>::set_blend_mode_pkt;
        new.blend_type = blend_type;
        new
    }
}

#[allow(dead_code)]
impl PktData<BlendModePkt> {
    fn set_blend_mode_pkt(&mut self) {
        unsafe {
            set_blend_key_preset(self.buffer_index as usize, self.get_data().blend_type as usize);
            self.move_to_end();
        }
    }
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

// SetColorMaskPkt
#[repr(C)]
#[derive(Debug)]
pub struct ColorMaskPkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    render_mask: u32,
}

impl PktFunction for ColorMaskPkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl ColorMaskPkt {
    pub fn new(mask: u32) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<ColorMaskPkt>::set_color_mask_pkt;
        new.render_mask = mask;
        new
    }
    // 0x1410948f0
    pub fn set_color_mask(&self, buffer_index: usize) {
        let buffer = unsafe { globals::get_ngr_draw_state_unchecked_mut().basicBuffers.get_unchecked_mut(buffer_index) };
        buffer.set_blend_render_mask(self.render_mask);
    }
}
impl PktData<ColorMaskPkt> {
    fn set_color_mask_pkt(&mut self) {
        unsafe {
            self.get_data().set_color_mask(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}

// StencilFuncPkt
#[repr(C)]
#[derive(Debug)]
pub struct StencilFuncPkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    func: ComparisonFunc,
    state_ref: u8,
    read_mask: u8,
    write_mask: u8
}

impl PktFunction for StencilFuncPkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl StencilFuncPkt {
    pub fn new(func: ComparisonFunc, state_ref: u8, read_mask: u8, write_mask: u8) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<StencilFuncPkt>::set_stencil_func;
        new.func = func;
        new.state_ref = state_ref;
        new.read_mask = read_mask;
        new.write_mask = write_mask;
        new
    }
    // 0x141101910
    pub fn set_stencil_func(&self, buffer_index: usize) {
        let buffer = unsafe { globals::get_ngr_draw_state_unchecked_mut().basicBuffers.get_unchecked_mut(buffer_index) };
        buffer.set_depth_stencil_face_function(self.func, self.read_mask, self.write_mask);
        buffer.set_depth_stencil_state_ref(self.state_ref);
    }
}
impl PktData<StencilFuncPkt> {
    fn set_stencil_func(&mut self) {
        unsafe {
            self.get_data().set_stencil_func(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}
// StencilOpPkt
#[repr(C)]
#[derive(Debug)]
pub struct StencilOpPkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    fall_op: StencilOperation,
    depth_fall_op: StencilOperation,
    pass_op: StencilOperation
}

impl PktFunction for StencilOpPkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl StencilOpPkt {
    pub fn new(fall_op: StencilOperation, depth_fall_op: StencilOperation, pass_op: StencilOperation) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<StencilOpPkt>::set_stencil_op;
        new.fall_op = fall_op;
        new.depth_fall_op = depth_fall_op;
        new.pass_op = pass_op;
        new
    }
    // 0x141101870
    pub fn set_stencil_op(&self, buffer_index: usize) {
        let buffer = unsafe { globals::get_ngr_draw_state_unchecked_mut().basicBuffers.get_unchecked_mut(buffer_index) };
        buffer.set_depth_stencil_face_operation(self.fall_op, self.depth_fall_op, self.pass_op);
    }
}
impl PktData<StencilOpPkt> {
    fn set_stencil_op(&mut self) {
        unsafe {
            self.get_data().set_stencil_op(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}
// StencilTestEnablePkt
#[repr(C)]
#[derive(Debug)]
pub struct StencilTestEnablePkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    enable: bool
}

impl PktFunction for StencilTestEnablePkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl StencilTestEnablePkt {
    pub fn new(enable: bool) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<StencilTestEnablePkt>::set_enable;
        new.enable = enable;
        new
    }
    // 0x141101820
    pub fn set_enable(&self, buffer_index: usize) {
        let buffer = unsafe { globals::get_ngr_draw_state_unchecked_mut().basicBuffers.get_unchecked_mut(buffer_index) };
        buffer.set_depth_stencil_depth_enable(self.enable);
    }
}
impl PktData<StencilTestEnablePkt> {
    fn set_enable(&mut self) {
        unsafe {
            self.get_data().set_enable(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}
// SetTexturePkt

#[repr(C)]
#[derive(Debug)]
pub struct TexturePkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    stage: u32,
    tex: *const Texture,
    min: u8,
    mag: u8,
    wraps: u8,
    wrapt: u8
}

impl PktFunction for TexturePkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl TexturePkt {
    pub fn new(stage: u32, texture: &Texture) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<TexturePkt>::set_texture_pkt;
        new.stage = stage;
        new.tex = &raw const *texture;
        new.min = texture.min;
        new.mag = texture.mag;
        new.wraps = texture.wraps;
        new.wrapt = texture.wrapt;
        new
    }
    unsafe fn get_texture(&self) -> &Texture { &*self.tex }
    // 0x141101150
    pub fn set_texture(&self, buffer_index: usize) {
        let sampler_chk = 1 << (self.stage & 0x1f);
        let draw_state = unsafe { globals::get_ngr_draw_state_unchecked_mut() };
        let frame_id = draw_state.get_ot_frame_id() as usize;
        let buffer = unsafe { draw_state.basicBuffers.get_unchecked_mut(buffer_index) };
        if self.tex.is_null() {
            if (buffer.sampler_mask & sampler_chk) != 0 {
                buffer.sampler_flag |= sampler_chk;
                buffer.sampler_mask &= sampler_chk;
            }
        } else {
            if (buffer.sampler_mask & sampler_chk) == 0 {
                buffer.sampler_flag |= sampler_chk;
                buffer.sampler_mask |= sampler_chk;
            }
            let filter = if self.min == 1 {
                if self.mag == 1 {
                    FilterMode::Anisotropic
                } else if self.mag != 0 {
                    FilterMode::MinPointMagMipLinear
                } else {
                    FilterMode::MinLinearMagMipPoint
                }
            } else if self.min == 0 && self.mag == 0 {
                FilterMode::MinMagMipPoint
            } else {
                FilterMode::MinPointMagMipLinear
            };
            buffer.set_sampler_filter(self.stage as usize, filter);
            buffer.set_sampler_address2d(self.stage as usize, self.wraps.try_into().unwrap(), self.wrapt.try_into().unwrap());
        }
        let resource = if self.tex.is_null() { None } else { Some(unsafe { self.get_texture().get_handle()} ) };
        let ctx = buffer.get_deferred_context_mut(frame_id);
        unsafe { ctx.set_shader_resource_view(BufferType::Pixel, self.stage as usize, resource); }
    }
}

#[allow(dead_code)]
impl PktData<TexturePkt> {
    fn set_texture_pkt(&mut self) {
        unsafe {
            self.get_data().set_texture(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}

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
            IATopology,
            StencilOperation,
            VertexBuffer
        }
    },
    globals,
    graphics::{
        draw2d::ImmediateRenderType,
        render::cmd_buffer::CmdBufferInterface,
        texture::Texture
    },
    kernel::graphics::GraphicsGlobal
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
        let cmd_buffer = GraphicsGlobal::get_gfd_graphics_global_mut().get_current_cmd_buffer().unwrap();
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

// ImmediateRenderPkt
#[repr(C)]
#[derive(Debug)]
pub struct ImmediateRenderPkt {
    set_buffers: ImmediateRenderSetBuffers,
    draw: ImmediateRenderDraw
}

#[repr(C)]
#[derive(Debug)]
pub struct ImmediateRenderSetBuffers {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    slot: u32,
    buffer: *const VertexBuffer,
    count: u32,
    stride: u32,
    offset: u32,
    buffer_index: u32
}

impl PktFunction for ImmediateRenderSetBuffers {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl ImmediateRenderSetBuffers {
    fn set_vertex_buffers(&self, buffer_index: usize) {
        let state = unsafe { globals::get_ngr_draw_state_unchecked_mut() };
        let frame_id = state.get_ot_frame_id();
        unsafe {
            let buf = if self.buffer.is_null() { None } else { Some(&*self.buffer) };
            state.basicBuffers.get_unchecked_mut(buffer_index)
                .get_deferred_context_mut(frame_id).set_vertex_buffers(
                self.slot, buf, self.count as usize, self.stride, self.offset, self.buffer_index as usize
            );
        }
    }
}

impl PktData<ImmediateRenderSetBuffers> {
    fn set_buffers(&mut self) {
        unsafe {
            self.get_data().set_vertex_buffers(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ImmediateRenderDraw {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    ty: ImmediateRenderType,
    count: u32,
    start: u32
}

impl PktFunction for ImmediateRenderDraw {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl ImmediateRenderDraw {
    fn draw(&self, buffer_index: usize) {
        let state = unsafe { globals::get_ngr_draw_state_unchecked_mut() };
        unsafe { state.update_vertex_buffers(buffer_index); }
        let frame_id = state.get_ot_frame_id();
        let ctx = unsafe { state.basicBuffers.get_unchecked_mut(buffer_index)
                .get_deferred_context_mut(frame_id) };
        if self.ty == ImmediateRenderType::Indexed { // draw indexed
            let buffer = unsafe { globals::get_ngr_draw_state_unchecked_mut().get_index_buffer() };
            unsafe { ctx.set_index_buffer(buffer, 0, 0); }
            ctx.draw_indexed(IATopology::TriangleList, (self.count - 2) * 3, 0, self.start as i32);
        } else { ctx.draw(self.ty.into(), self.count, self.start); }
    }
}

impl PktData<ImmediateRenderDraw> {
    fn draw(&mut self) {
        unsafe {
            self.get_data().draw(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}

impl ImmediateRenderPkt {
    pub fn new(ty: ImmediateRenderType, count: u32, verts: *mut u8, stride: u32, _fvf: u32) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        let frame_id = GraphicsGlobal::get_gfd_graphics_global_mut().get_frame_id();
        let renderer = unsafe { globals::get_ngr_dx11_renderer_unchecked() };
        let buf = unsafe { renderer.get_command_buffer_unchecked() };
        new.set_buffers.cb = PktData::<ImmediateRenderSetBuffers>::set_buffers;
        new.set_buffers.slot = 0;
        new.set_buffers.count = count;
        new.set_buffers.buffer_index = frame_id as u32;
        new.set_buffers.stride = stride;
        new.set_buffers.offset = unsafe { verts.sub(buf.bufStart as usize) as u32 };
        new.set_buffers.buffer = buf.buffers as *const VertexBuffer;
        new.draw.cb = PktData::<ImmediateRenderDraw>::draw;
        new.draw.count = count;
        new.draw.ty = ty;
        new.draw.start = 0;
        new
    }
}

impl PktFunction for ImmediateRenderPkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
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

// AlphaFuncPkt
#[repr(C)]
#[derive(Debug)]
pub struct AlphaFuncPkt {
    cb: unsafe fn(&mut PktData<Self>) -> (),
    alpha_test_func: u16,
    alpha_test_ref: u32,
}

impl PktFunction for AlphaFuncPkt {
    fn as_raw(&self) -> *const Self { &raw const *self }
}

impl AlphaFuncPkt {
    pub fn new(alpha_func: u16, alpha_ref: u32) -> &'static mut Self {
        let new: &mut Self = PktData::allocate_for_pkt();
        new.cb = PktData::<AlphaFuncPkt>::set_alpha;
        new.alpha_test_ref = alpha_ref;
        new.alpha_test_func = alpha_func;
        new
    }
    // 0x1411019c0
    pub fn set_alpha(&self, buffer_index: usize) {
        let draw_state = unsafe { globals::get_ngr_draw_state_unchecked_mut() };
        let frame_id = draw_state.get_ot_frame_id();
        let buffer = unsafe { draw_state.basicBuffers.get_unchecked_mut(buffer_index) };
        if buffer.alpha_test_ref != self.alpha_test_ref 
        || buffer.alpha_test_func != self.alpha_test_func {
            let aref = match self.alpha_test_func {
                1 => ((self.alpha_test_ref as f32) - 1f32) * 0.003921569,
                4 => ((self.alpha_test_ref as f32) + 1f32) * 0.003921569,
                _ => (self.alpha_test_ref as f32) * 0.003921569
            };
            let alpha_fn = match self.alpha_test_func {
                1 => 1f32, 2 => 2f32, 3 => 1f32,
                4 => 3f32, 5 => 4f32, 6 => 3f32,
                _ => 0f32
            };
            let buf = buffer.get_alpha_test_constant_buffer_mut();
            unsafe {
                buf.set_field_unchecked(frame_id, 0, alpha_fn);
                buf.set_field_unchecked(frame_id, 1, aref);
            }
            buffer.alpha_test_ref = self.alpha_test_ref;
            buffer.alpha_test_func = self.alpha_test_func;
            buffer.flags |= BufferFlags::USING_ALPHA_TEST;
        }
    }
}

#[allow(dead_code)]
impl PktData<AlphaFuncPkt> {
    fn set_alpha(&mut self) {
        unsafe {
            self.get_data().set_alpha(self.buffer_index as usize);
            self.move_to_end();
        }
    }
}

use crate::{
    device::ngr::renderer::{
        cbuffer::BufferType,
        shader::{
            PixelShaderPlatform,
            PixelShader,
            VertexShaderPlatform,
            VertexShader
        },
        state::{
            BufferFlags,
            BufferFlags2,
            DepthWriteMask,
            DeferredContext,
            IATopology,
            TextureAddressMode
        }
    },
    globals,
    graphics::cull::CullObject
};
use std::mem::size_of;
use windows::Win32::Graphics::Direct3D11::D3D11_VIEWPORT;

// Render OT linked list
#[repr(C)]
#[derive(Debug)]
pub struct RenderOtListInner {
    head: *mut RenderOt,
    tail: *mut RenderOt,
}

impl RenderOtListInner {
    pub unsafe fn insert_entry(&mut self, new: &mut RenderOt) {
        // Are we the first entry in the linked list?
        if self.head.is_null() {
            self.head = &raw mut *new;
        } else {
            // Append onto the end of the list
            (&mut *self.tail).next = &raw mut *new;
        }
        self.tail = &raw mut *new;
    }

    pub unsafe fn insert_entry_culled(&mut self, new: &mut RenderOt) {
        // Are we the first entry in the linked list?
        if self.head.is_null() {
            self.head = &raw mut *new;
        } else {
            // Append onto the end of the list
            (&mut *self.tail).next_cull = &raw mut *new;
        }
        self.tail = &raw mut *new;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderOtList {
    field0: [u8; 0x18],
    render: RenderOtListInner,
    cull: RenderOtListInner,
    field38: [u8; 8]
}

type RenderOtReturn<T> = Result<T, ()>;
#[cfg(feature = "v1-core")]
type RenderOtCallback = fn(*mut RenderOt, *mut u8) -> ();
#[cfg(feature = "v2-core")]
type RenderOtCallback = unsafe extern "C" fn(*mut RenderOt, *mut u8, *mut u8) -> ();

pub unsafe trait RenderOtBase {
    fn set_data<T>(&mut self, data: *const T);
    fn set_pre_cb(&mut self, cb: RenderOtCallback);
    fn set_pre_cb_data(&mut self, data: *const u8);
    fn set_post_cb(&mut self, cb: RenderOtCallback);
    fn set_post_cb_data(&mut self, data: *const u8);
    unsafe fn link(&mut self, prio: u32);
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RenderOt {
    data: *mut u8,
    pre_cb_data: *mut u8,
    post_cb_data: *mut u8,
    pre_cb: Option<RenderOtCallback>,
    post_cb: Option<RenderOtCallback>,
    next: *mut RenderOt,
    next_cull: *mut RenderOt,
    geometry_cull: *mut CullObject
}


impl RenderOt {
    /// (Original function: gfdRenderOtSetup)
    pub unsafe fn setup(extra: usize) -> *mut Self {
        let cmd_buffer = globals::get_gfd_global_unchecked_mut().graphics.get_current_cmd_buffer();
        let res = cmd_buffer.alloc_type::<Self>(extra); 
        (&mut *res).geometry_cull = globals::get_gfd_global_unchecked().graphics.get_geometry_cull();
        res
    } 
}

unsafe impl RenderOtBase for RenderOt {
    fn set_data<T>(&mut self, data: *const T) {
        self.data = data as *mut u8;
    }
    fn set_pre_cb(&mut self, cb: RenderOtCallback) {
        self.pre_cb = Some(cb);
    }
    fn set_pre_cb_data(&mut self, data: *const u8) {
        self.pre_cb_data = data as *mut u8;
    }
    fn set_post_cb(&mut self, cb: RenderOtCallback) {
        self.post_cb = Some(cb);
    }
    fn set_post_cb_data(&mut self, data: *const u8) {
        self.post_cb_data = data as *mut u8;
    }
    unsafe fn link(&mut self, prio: u32) {
        let glb = globals::get_gfd_global_unchecked_mut();
        let ot_list = glb.graphics.get_ot_render_list(glb.graphics.get_frame_id(), prio as usize);
        if !self.geometry_cull.is_null() {
            self.next_cull = std::ptr::null_mut();
            ot_list.cull.insert_entry_culled(self);
        } else {
            self.next = std::ptr::null_mut();
            ot_list.render.insert_entry(self);
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RenderOtEx<const D: usize = 0> {
    base: RenderOt,
    data: [u8; D],
}

impl<const D: usize> RenderOtEx<D> {
    pub fn new() -> &'static mut Self {
        let cmd_buffer = unsafe { globals::get_gfd_global_unchecked_mut().graphics.get_current_cmd_buffer() };
        let res = unsafe { cmd_buffer.alloc_ex::<Self>() };
        unsafe {
            libc::memset(
                res as *mut RenderOtEx<D> as *mut libc::c_void, 
                0 as libc::c_int, 
                std::mem::size_of::<Self>() as libc::size_t
            );
        } 
        res.base.geometry_cull = unsafe { globals::get_gfd_global_unchecked().graphics.get_geometry_cull() };
        res
    }

    pub unsafe fn get<T: Copy>(&self, offset: usize) -> RenderOtReturn<T> {
        if offset + size_of::<T>() <= D { 
            Ok(*((&raw const self.data[offset]) as *const T))
        } else {
            Err(())
        }
    }

    pub unsafe fn data_raw(&self) -> *const u8 { self.data.as_ptr() }

    pub unsafe fn set<T: Copy>(&mut self, offset: usize, value: T) -> RenderOtReturn<()> {
        if offset + size_of::<T>() <= D { 
            *((&raw mut self.data[offset]) as *mut T) = value;
            Ok(())
        } else {
            Err(())
        } 
    }
}

unsafe impl<const D: usize> RenderOtBase for RenderOtEx<D> {
    fn set_data<T>(&mut self, data: *const T) {
        self.base.set_data(data)
    }
    fn set_pre_cb(&mut self, cb: RenderOtCallback) {
        self.base.set_pre_cb(cb)
    }
    fn set_pre_cb_data(&mut self, data: *const u8) {
        self.base.set_pre_cb_data(data)
    }
    fn set_post_cb(&mut self, cb: RenderOtCallback) {
        self.base.set_post_cb(cb)
    }
    fn set_post_cb_data(&mut self, data: *const u8) {
        self.base.set_post_cb_data(data)
    }
    unsafe fn link(&mut self, prio: u32) {
        self.base.link(prio)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderOtGroup {
    prio_ot: [u32; 7]
}

/// Original function: gfdRenderStateSetOtPreCallback
pub(super) unsafe extern "C" fn set_state_pre_callback(_ot: *mut RenderOt, buffer: *mut u8, data: *mut u8) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    draw_state.set_ot_state(buffer as i32, *(data as *const u32), data.add(8));
}
/// Original function: gfdRenderStatePushOtPreCallback
pub(super) unsafe extern "C" fn push_state_pre_callback(_ot: *mut RenderOt, _buffer: *mut u8, stack: *mut u8) {
    let stack = stack as u32;
    let global = globals::get_gfd_global_unchecked_mut();
    global.graphics.render_state_stack[stack as usize][1] = global.graphics.render_state_stack[stack as usize][0];
    global.graphics.render_state_stack[stack as usize][0] = global.graphics.render_state_current[stack as usize];
}
/// Original function: gfdRenderStatePopOtPreCallback
pub(super) unsafe extern "C" fn pop_state_pre_callback(_ot: *mut RenderOt, buffer: *mut u8, fun: *mut u8) {
    let fun = fun as u32;
    let buffer = buffer as i32;
    let global = globals::get_gfd_global_unchecked_mut();
    let popped = *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(0);
    *global.graphics.render_state_stack.get_unchecked_mut(fun as usize).get_unchecked_mut(0) = 
        *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(1);
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    draw_state.set_ot_state(buffer, fun, popped as *mut u8);
}

/// Original function: gfdShaderVertexBindOtPreCallback
pub(super) unsafe extern "C" fn bind_vertex_shader_pre_callback(_ot: *mut RenderOt, id: *mut u8, shader: *mut u8) {
    let id = (id as u32) as usize;
    let global = globals::get_gfd_global_unchecked_mut();
    if *global.graphics.get_current_vertex_shader_ptr() != shader as *mut VertexShader {
        std::ptr::write(global.graphics.get_current_vertex_shader_ptr(), shader as *mut VertexShader);
        let shader_data = if shader.is_null() { None } else {
            let shader = &*(shader as *mut VertexShader);
            if shader.data != std::ptr::null_mut() { Some(&*shader.data) } else { None }
        };
        // gfdDeviceShaderVertexBind
        let draw_state = globals::get_ngr_draw_state_unchecked_mut();
        let frame_id = draw_state.get_ot_frame_id();
        draw_state.basicBuffers.get_unchecked_mut(id)
            .get_deferred_context_mut(frame_id).set_vertex_shader(shader_data);
    }
}

/// Original function: gfdShaderFragmentBindOtPreCallback 
pub(super) unsafe extern "C" fn bind_pixel_shader_pre_callback(_ot: *mut RenderOt, id: *mut u8, shader: *mut u8) {
    let id = (id as u32) as usize;
    let global = globals::get_gfd_global_unchecked_mut();
    if *global.graphics.get_current_pixel_shader_ptr() != shader as *mut PixelShader {
        std::ptr::write(global.graphics.get_current_pixel_shader_ptr(), shader as *mut PixelShader);
        let shader_data = if shader.is_null() { None } else {
            let shader = &*(shader as *mut PixelShader);
            if shader.data != std::ptr::null_mut() { Some(&*shader.data) } else { None }
        };
        // gfdDeviceShaderVertexBind
        let draw_state = globals::get_ngr_draw_state_unchecked_mut();
        let frame_id = draw_state.get_ot_frame_id();
        draw_state.basicBuffers.get_unchecked_mut(id)
            .get_deferred_context_mut(frame_id).set_pixel_shader(shader_data);
    }
}

#[allow(dead_code, unused_variables)]
/// Original function: gfdDeviceRenderCloudsOtCallback
// TODO
pub(super) unsafe extern "C" fn render_clouds_pre_callback(_ot: *mut RenderOt, buffer: *mut u8, data: *mut u8) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    let gfd_global = globals::get_gfd_global_unchecked_mut();
    let frame_id = draw_state.get_ot_frame_id();
    let buffer_id = (buffer as u32) as usize;
    let upd = data as u32;
    let mut buffer = draw_state.basicBuffers.get_unchecked_mut(buffer_id);
    crate::device::ngr::renderer::pkt::set_blend_key_preset2(buffer_id, 0, false);
    crate::device::ngr::renderer::render::set_depth_stencil_key_less_equal(buffer_id, false, DepthWriteMask::MaskNone);
    buffer.flags2 |= BufferFlags2::FLAG11;
    buffer.flags |= BufferFlags::USING_BLEND;
    crate::device::ngr::renderer::render::set_sampler_key_values(buffer_id, 0, true, true, TextureAddressMode::Wrap, TextureAddressMode::Wrap);
    crate::device::ngr::renderer::render::set_sampler_key_values(buffer_id, 1, true, true, TextureAddressMode::Mirror, TextureAddressMode::Mirror);
    draw_state.update_vertex_buffers(buffer_id);
    buffer = draw_state.basicBuffers.get_unchecked_mut(buffer_id);
    // ...
    let ctx = buffer.get_deferred_context_mut(frame_id);
    ctx.set_constant_buffers(BufferType::Vertex, &mut *draw_state.cloud_buffer, upd);
    ctx.set_constant_buffers(BufferType::Pixel, &mut *draw_state.cloud_buffer, upd);
    // IASetVertexBuffers
    // OMSetRenderTargets
    // ...
    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0f32,
        TopLeftY: 0f32,
        Width: 0f32,
        Height: 0f32,
        MinDepth: 0f32,
        MaxDepth: 0f32
    };
    ctx.set_viewports(&viewport);
    ctx.set_shader_resource_view(BufferType::Pixel, 0, Some((&*draw_state.cloud_main).get_handle()));
    ctx.set_shader_resource_view(BufferType::Pixel, 1, Some((&*draw_state.cloud_sub).get_handle()));
    ctx.set_shader_resource_view(BufferType::Pixel, 2, Some((&*draw_state.cloud_2d).get_handle()));
    // ...
    ctx.set_shader_resource_view(BufferType::Pixel, 3, None);
    ctx.set_shader_sample(BufferType::Pixel, 2, if draw_state.sampler_620.is_null() { None } else { Some(&*draw_state.sampler_620)});
    // let v0 = ;
    ctx.set_vertex_shader(gfd_global.graphics.get_vertex_shader_platform(46));
    ctx.set_pixel_shader(gfd_global.graphics.get_pixel_shader_platform(130));
    ctx.draw(IATopology::TriangleStrip, 4, 0);
    ctx.set_shader_resource_view(BufferType::Pixel, 3, None);
    // OMSetRenderTargets
    let viewport2 = D3D11_VIEWPORT {
        TopLeftX: 0f32,
        TopLeftY: 0f32,
        Width: 0f32,
        Height: 0f32,
        MinDepth: 0f32,
        MaxDepth: 0f32
    };
    ctx.set_viewports(&viewport2);
    ctx.set_shader_resource_view(BufferType::Pixel, 0, None);
    ctx.set_vertex_shader(gfd_global.graphics.get_vertex_shader_platform(23));
    ctx.set_pixel_shader(gfd_global.graphics.get_pixel_shader_platform(105));
    crate::device::ngr::renderer::render::set_sampler_key_values(buffer_id, 2, true, true, TextureAddressMode::Clamp, TextureAddressMode::Clamp);
    crate::device::ngr::renderer::pkt::set_blend_key_preset2(buffer_id, 1, true);
    crate::device::ngr::renderer::render::set_depth_stencil_key_less_equal(buffer_id, true, DepthWriteMask::MaskNone);
    let ctx = buffer.get_deferred_context_mut(frame_id);
    ctx.draw(IATopology::TriangleStrip, 4, 0);
    for i in 0..4 { ctx.set_shader_resource_view(BufferType::Pixel, 0, None); }
    buffer.set_sampler_mask(0);
    buffer.set_sampler_mask(1);
    // ...
    crate::device::ngr::renderer::render::set_depth_stencil_key_less_equal(buffer_id, true, DepthWriteMask::MaskAll);
    crate::device::ngr::renderer::pkt::set_blend_key_preset2(buffer_id, 0, false);
    buffer.sampler_flag |= 4;
    buffer.flags2 &= BufferFlags2::FLAG11;
    buffer.flags |= BufferFlags::USING_BLEND;
    gfd_global.graphics.field44b8_clear();
}

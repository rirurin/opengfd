#![allow(dead_code, unused_imports)]

use crate::{
    device::ngr::renderer::{
        pkt::{ 
            BlendModePkt,
            ColorMaskPkt,
            StencilFuncPkt,
            StencilOpPkt,
            StencilTestEnablePkt,
            TexturePkt 
        },
        shader::{ PixelShader, VertexShader },
        state::{ ComparisonFunc, StencilOperation }
    },
    graphics::{
        render_ot::{ self, RenderOt, RenderOtBase, RenderOtEx },
        texture::Texture
    },
    kernel::global::{ Global, GraphicsGlobal },
    globals
};

pub struct Render;
impl Render {
    /// Original function: gfdRenderStateSet
    pub unsafe fn set_state(prio: u32, state: u32, value: *mut u8) {
        let ot = RenderOtEx::<16>::new();
        ot.set::<u32>(0, state).unwrap();
        ot.set::<*mut u8>(8, value).unwrap();
        ot.set_pre_cb(render_ot::set_state_pre_callback);
        ot.set_pre_cb_data(ot.data_raw());
        ot.link(prio)
    }
    /// Original function: gfdRenderStatePush
    pub unsafe fn push_state(prio: u32, state: u32) {
        let ot = RenderOtEx::<0>::new();
        ot.set_pre_cb(render_ot::push_state_pre_callback);
        ot.set_pre_cb_data(state as *mut u8);
        ot.link(prio)
    }
    /// Original function: gfdRenderStatePop
    pub unsafe fn pop_state(prio: u32, state: u32) {
        let ot = RenderOtEx::<0>::new();
        ot.set_pre_cb(render_ot::pop_state_pre_callback);
        ot.set_pre_cb_data(state as *mut u8);
        ot.link(prio)       
    }
    /// Original function: gfdShaderVertexBind
    pub unsafe fn bind_vertex_shader(prio: u32, shader: &VertexShader) {
        let ot = RenderOtEx::<0>::new();
        ot.set_pre_cb(render_ot::bind_vertex_shader_pre_callback);
        ot.set_pre_cb_data(&raw const *shader as *const u8);
        ot.link(prio);
    }
    /// Original function: gfdShaderFragmentBind
    pub unsafe fn bind_pixel_shader(prio: u32, shader: &PixelShader) {
        let ot = RenderOtEx::<0>::new();
        ot.set_pre_cb(render_ot::bind_pixel_shader_pre_callback);
        ot.set_pre_cb_data((&raw const *shader) as *const u8);
        ot.link(prio);
    }
    /// Original function: gfdRenderSetBlendMode
    pub unsafe fn set_blend_mode(prio: u32, blend: u32) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *BlendModePkt::new(blend));
        ot.link(prio);
    }
    /// Original function: gfdRenderTextureSet 
    pub unsafe fn set_texture(prio: u32, stage: u32, tex: &Texture) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *TexturePkt::new(stage, tex));
        ot.link(prio);
    }
    /// UNTESTED. Inside gfdGeometryRender
    pub unsafe fn set_color_mask(prio: u32, color_mask: u32) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *ColorMaskPkt::new(color_mask));
        ot.link(prio);
    }
    /// UNTESTED. Inside gfdGeometryRender
    pub unsafe fn set_stencil_func(prio: u32, func: ComparisonFunc, 
        state_ref: u8, read_mask: u8, write_mask: u8) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *StencilFuncPkt::new(
            func, state_ref, read_mask, write_mask
        ));
        ot.link(prio);
    }
    /// UNTESTED. Inside gfdGeometryRender
    pub unsafe fn set_stencil_op(prio: u32, fall: StencilOperation, 
        depth_fall: StencilOperation, pass: StencilOperation) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *StencilOpPkt::new(fall, depth_fall, pass));
        ot.link(prio);
    }
    /// UNTESTED. Inside gfdGeometryRender
    pub unsafe fn set_stencil_test_enable(prio: u32, enable: bool) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *StencilTestEnablePkt::new(enable));
        ot.link(prio);
    }
    /// UNTESTED. Inside gfdGeometryRender
    pub unsafe fn set_alpha_func(prio: u32, fall: StencilOperation, 
        depth_fall: StencilOperation, pass: StencilOperation) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *StencilOpPkt::new(fall, depth_fall, pass));
        ot.link(prio);
    }
    /// UNTESTED. Inside gfdGeometryRender
    pub unsafe fn set_alpha_test_enable(prio: u32, enable: bool) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *StencilTestEnablePkt::new(enable));
        ot.link(prio);
    }
}

#[allow(non_snake_case)]
pub mod ffi {
    use crate::graphics::render_ot::RenderOt;
    use super::Render;

    #[no_mangle]
    pub unsafe extern "C" fn gfdRenderStateSet(prio: u32, state: u32, value: *mut u8) {
        Render::set_state(prio, state, value);
    }

    #[no_mangle]
    pub unsafe extern "C" fn gfdRenderStatePush(prio: u32, state: u32) {
        Render::push_state(prio, state);
    }

    #[no_mangle]
    pub unsafe extern "C" fn gfdRenderStatePop(prio: u32, state: u32) {
        Render::pop_state(prio, state);
    }

    #[no_mangle]
    pub unsafe extern "C" fn gfdRenderSetBlendMode(prio: u32, blend: u32) {
        Render::set_blend_mode(prio, blend);
    }
}

#[cfg(test)]
pub mod tests {

}

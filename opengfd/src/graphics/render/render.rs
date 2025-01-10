#![allow(dead_code, unused_imports)]

use crate::{
    device::ngr::renderer::{
        blend::BlendModePkt,
        shader::VertexShader
    },
    graphics::render_ot::{ self, RenderOt, RenderOtBase, RenderOtEx },
    kernel::global::{ Global, GraphicsGlobal },
    globals
};
/*
pub unsafe fn vertex_shader_bind_ot_pre_callback(pOt: *mut RenderOt, id: i32, userdata: Option<*mut VertexShader>) {
    globals::get_gfd_global_unchecked()
    let global = globals::get_gfd_global().as_mut().unwrap();    
    if userdata.is_some() 
        && global.graphics.shader_current_vertex != userdata.unwrap() 

        {

    }
}
*/

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
    /// Original function: gfdRenderSetBlendMode
    pub unsafe fn set_blend_mode(prio: u32, blend: u32) {
        let ot = RenderOtEx::<0>::new();
        ot.set_data(&raw const *BlendModePkt::new(blend));
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

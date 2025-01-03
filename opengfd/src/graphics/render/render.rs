#![allow(dead_code, unused_imports)]

use crate::{
    device::ngr::renderer::vs::VertexShader,
    graphics::render_ot::RenderOt,
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

/*
pub struct Render;

impl Render {
    pub unsafe fn set_state(prio: u32, state: u32) {

    }
    pub unsafe fn push_state(prio: u32) {

    }
    pub unsafe fn pop_state(prio: u32) {

    }
}

impl Render {
    pub unsafe fn push_state_callback(_ot: *mut RenderOt, _a2: *mut u8, stack: *mut u8) {
        let stack = stack as u32;
        let global = globals::get_gfd_global_unchecked_mut();
        global.graphics.render_state_stack[stack as usize][1] = global.graphics.render_state_stack[stack as usize][0];
        global.graphics.render_state_stack[stack as usize][0] = global.graphics.render_state_current[stack as usize];
    }

    pub unsafe fn pop_state_callback(_ot: *mut RenderOt, buffer: *mut u8, fun: *mut u8) {
        let fun = fun as u32;
        let buffer = buffer as i32;
        let global = globals::get_gfd_global_unchecked_mut();
        let popped = *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(0);
        *global.graphics.render_state_stack.get_unchecked_mut(fun as usize).get_unchecked_mut(0) = 
            *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(1);
        // gfdDeviceRenderSetState(buffer, fun, popped as *mut u8);
    }
}

#[allow(non_snake_case)]
pub mod ffi {
    use crate::graphics::render_ot::RenderOt;
    use super::Render;
    #[no_mangle]
    pub unsafe extern "C" fn gfdRenderStatePushOtPreCallback(_ot: *mut RenderOt, _a2: *mut u8, stack: *mut u8) {
        Render::push_state_callback(_ot, _a2, stack)
    }
    pub unsafe extern "C" fn gfdRenderStatePopOtPreCallback(_ot: *mut RenderOt, buffer: *mut u8, fun: *mut u8) {
        Render::pop_state_callback(_ot, buffer, fun);
    }
}
*/

#[cfg(test)]
pub mod tests {

}

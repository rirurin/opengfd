use opengfd::{
    device::ngr::renderer::{
        vs::{ VertexShader, VertexShaderPlatform },
        state::{ DeferredContextBase, DeferredContextDX11 }
    },
    globals
};
use std::ptr::NonNull;
use windows::{
    core::Param,
    Win32::Graphics::Direct3D11::{ ID3D11PixelShader, ID3D11VertexShader }
};
use riri_mod_tools_proc::{ ensure_layout, riri_hook_fn, riri_hook_static };
use riri_mod_tools_rt::{ logln, sigscan_resolver };

#[no_mangle]
pub unsafe extern "C" fn set_gfd_global(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match riri_mod_tools_rt::sigscan_resolver::get_indirect_address_short2(ofs) {
        Some(v) => v,
        None => return None
    };
    let out = addr.sub(20);
    logln!(Information, "got gfdGlobal: 0x{:x}", out.as_ptr() as usize);
    globals::set_gfd_global(out.as_ptr() as *mut opengfd::kernel::global::Global);
    Some(out)
}

#[riri_hook_static(dynamic_offset(
    signature = "F7 05 ?? ?? ?? ?? 00 00 00 02",
    resolve_type = set_gfd_global,
    calling_convention = "microsoft",
))]
riri_static!(GFD_GLOBAL, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_draw_state(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let ngr_init_state = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    let addr = match sigscan_resolver::get_indirect_address_long_abs(ngr_init_state.add(0x26).as_ptr()) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_draw_state(addr.as_ptr() as 
        *mut *mut opengfd::device::ngr::renderer::state::DrawState);
    logln!(Information, "got ngrDrawState: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    signature = "48 83 EC 28 E8 ?? ?? ?? ?? B9 E0 17 00 00",
    resolve_type = set_ngr_draw_state,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DRAWSTATE, usize);
#[riri_hook_fn(static_offset(0x1192e20))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetVertexProgramLoad(p_ctx: *mut u8, p_shader: *mut u8) {
    if !p_shader.is_null() { 
        // logln!(Information, "ctx: 0x{:x}, shader: 0x{:x}", p_ctx as usize, p_shader as usize);
        let ctx = (p_ctx as *mut DeferredContextDX11).as_mut().unwrap();
        let shader = (p_shader as *mut VertexShaderPlatform).as_ref().unwrap();
        // compare by-reference (equality operator checks by-value, following PartialEq/Eq impl)
        if std::ptr::eq(&shader.d3d_vertex, &ctx.super_.target_vertex_shader) { return; }
        let device_ctx = &ctx.super_.device_context;
        device_ctx.IASetInputLayout(&shader.d3d_input_layout);
        device_ctx.VSSetShader(&shader.d3d_vertex, None);
        *&mut &ctx.super_.target_vertex_shader = &shader.d3d_vertex;
    }
}
// #[riri_hook_fn(static_offset(0x1192ee0))]

#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetPixelProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    // println!("ctx: 0x{:x}, shader: 0x{:x}", p_ctx as usize, p_shader as usize);
    let ctx = &mut *(p_ctx as *mut DeferredContextDX11);
    if p_shader.is_null() {
        *&mut &raw const ctx.super_.target_pixel_shader = std::ptr::null();
        return;
    }
    let shader = &(&*(p_shader as *const VertexShaderPlatform)).d3d_pixel;
    if std::ptr::eq(shader, &ctx.super_.target_pixel_shader) { return; }
    let device_ctx = &ctx.super_.device_context;
    device_ctx.PSSetShader(shader, None);
    *&mut &ctx.super_.target_pixel_shader = shader;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetPixelProgramLoad(p_ctx: *mut DeferredContextDX11, p_shader: Option<&VertexShaderPlatform>) {
    // println!("ctx: 0x{:x}, shader: 0x{:x}", p_ctx as usize, p_shader as usize);
    let ctx = &mut *(p_ctx as *mut DeferredContextDX11);
    match p_shader {
        Some(v) => {
            let shader = &v.d3d_pixel;
            if std::ptr::eq(shader, &ctx.super_.target_pixel_shader) { return; }
            let device_ctx = &ctx.super_.device_context;
            device_ctx.PSSetShader(shader, None);
            *&mut &ctx.super_.target_pixel_shader = shader;
        },
        None => {
            *&mut &raw const ctx.super_.target_pixel_shader = std::ptr::null();
            return;
        }
    }
}

#[riri_hook_fn(static_offset(0x114b210))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDeviceShaderPixelBindHook(id: i32, shader: *mut u8) {
    let draw = globals::get_ngr_draw_state_unchecked().as_mut();
    let shader = if shader.is_null() { None } else { Some(&*(shader as *mut VertexShaderPlatform)) };
    ngrSetPixelProgramLoad(
        draw.basicBuffers[id as usize].deferredContexts[draw.otFrameId as usize],
        shader
    );
}
/*
#[riri_hook_fn(static_offset(0x11031c0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdShaderPixelBindOtPreCallback(_ot: *mut u8, id: i32, data: *mut u8) {

}
*/


/*
#[riri_hook_fn(static_offset(0x1192e20))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetVertexProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    if !p_shader.is_null() { 
        // logln!(Information, "ctx: 0x{:x}, shader: 0x{:x}", p_ctx as usize, p_shader as usize);
        let ctx = &mut *(p_ctx as *mut DeferredContextDX11);
        let shader = &*(p_shader as *mut VertexShaderPlatform);
        // let ctx = (p_ctx as *mut DeferredContextDX11).as_mut().unwrap();
        // let shader = (p_shader as *mut VertexShaderPlatform).as_ref().unwrap();
        ngrSetVertexProgramLoad(ctx, shader);
    }
}
*/
/*
#[allow(non_snake_case)] // this works independently!
pub unsafe fn ngrSetVertexProgramLoad(ctx: &mut DeferredContextDX11, shader: &VertexShaderPlatform) {
    // logln!(Information, "ctx: 0x{:x}, shader: 0x{:x}", &raw const ctx as usize, &raw const shader as usize);
    // compare by-reference (equality operator checks by-value, following PartialEq/Eq impl)
    if std::ptr::eq(&shader.d3d_vertex, &ctx.super_.target_vertex_shader) { return; }
    let device_ctx = &ctx.super_.device_context;
    device_ctx.IASetInputLayout(&shader.d3d_input_layout);
    device_ctx.VSSetShader(&shader.d3d_vertex, None);
    *&mut &ctx.super_.target_vertex_shader = &shader.d3d_vertex;
}
*/
/*
#[riri_hook_fn(static_offset(0x114aec0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDeviceShaderVertexBindHook(id: i32, p_shader: *const u8) {
    let shader = p_shader as *const VertexShaderPlatform;
    if shader.is_null() { return; }
    let draw = &mut *globals::get_ngr_draw_state().unwrap();
    //logln!(Information, "Context: 0x{:x}, Shader: 0x{:x}", 
    /*
    println!("Context: 0x{:x}, Shader: 0x{:x}",
        draw.basicBuffers[id as usize].deferredContexts[draw.otFrameId as usize] as usize,
        shader as usize
    );
    */
    // original_function!(id, p_shader)
    ngrSetVertexProgramLoad(
        draw.basicBuffers[id as usize].deferredContexts[draw.otFrameId as usize].as_mut().unwrap(),
        shader.as_ref().unwrap());
}
*/
/*
#[allow(non_snake_case)]
pub unsafe fn gfdDeviceShaderVertexBind(id: i32, shader: &VertexShaderPlatform) {
    let draw = &mut*globals::get_ngr_draw_state().unwrap();
    ngrSetVertexProgramLoad(
        &mut *draw.basicBuffers[id as usize].deferredContexts[draw.otFrameId as usize], 
        shader);
}
#[riri_hook_fn(static_offset(0x1103150))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdShaderVertexBindOtPreCallback(_render_ot: *mut u8, id: i32, shader: *mut u8) {
    let global = &mut*globals::get_gfd_global();
    let new_shader = shader as *const VertexShader;
    let current_vertex_shader = global.graphics.shader_current_vertex;
    if !std::ptr::eq(current_vertex_shader, new_shader) &&
        !new_shader.is_null() && !(&*current_vertex_shader).data.is_null()
    {
        /*
        gfdDeviceShaderVertexBind(id, &*(&*current_vertex_shader).data);
        *&mut global.graphics.shader_current_vertex = new_shader as *mut VertexShader;
        */
    }
    original_function!(_render_ot, id, shader)
}
*/

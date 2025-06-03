/* 
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{ Task as GfdTask, UpdateTask }
};
*/
// use opengfd_inspector::state::Inspector as GfdInspector;
use riri_mod_tools_proc::riri_hook_fn;
// use riri_mod_tools_rt::logln;
// use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState, VK_F5 };

/*
#[riri_hook_fn(static_offset(0x1192e20))]
#[allow(non_snake_case)] // Verified
pub unsafe extern "C" fn ngrSetVertexProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    let ctx = &mut *(p_ctx as *mut DeferredContextBase);
    let shader = if p_shader.is_null() { None } else { Some(&*(p_shader as *const VertexShaderPlatform)) };
    ctx.set_vertex_shader(shader);
}

#[riri_hook_fn(static_offset(0x1192ee0))]
#[allow(non_snake_case)] // Verified
pub unsafe extern "C" fn ngrSetPixelProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    let ctx = &mut *(p_ctx as *mut DeferredContextBase);
    let shader = if p_shader.is_null() { None } else { Some(&*(p_shader as *const PixelShaderPlatform)) };
    ctx.set_pixel_shader(shader);
}
*/

/*
#[riri_hook_fn(static_offset(0x1072960))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStateSetHook(prio: u32, state: u32, value: *mut u8) {
    Render::set_state(prio, state, value);
}

#[riri_hook_fn(static_offset(0x1072a50))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePopHook(prio: u32, state: u32) {
    Render::pop_state(prio, state);
}

#[riri_hook_fn(static_offset(0x10729e0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePushHook(prio: u32, state: u32) {
    Render::push_state(prio, state);
}
#[riri_hook_fn(static_offset(0x1072d40))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetBlendModeHook(prio: u32, blend: u32) {
    Render::set_blend_mode(prio, blend);
}
*/

/*
#[riri_hook_fn(static_offset(0x1101030))]
#[allow(non_snake_case)] // Major graphical issues (still!)
pub unsafe extern "C" fn gfdShaderVertexBindHook(prio: u32, vertex: *mut u8) {
    Render::bind_vertex_shader(prio, &*(vertex as *mut VertexShader));
}
*/
// #[riri_hook_fn(static_offset(0x10e36c0))]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn gfdRenderOtLink(prio: u32, ot: *mut u8) {
//     let ot = &mut *(ot as *mut RenderOt);
//     ot.link(prio);
// }

/*
#[riri_hook_fn(static_offset(0x11947b0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetRasterizerStateInner(p_renderer: *mut u8, raster_key: *mut u8) -> *mut u8 {
    let renderer = &mut *(p_renderer as *mut ngrDX11Renderer);
    let key = &mut *(raster_key as *mut RasterizerKey);
    let hash = CrcHash::new(key);
    match renderer.rasterizers.find_by_predicate(|f| f == key && f == &hash) {
        Some(v) => (&raw const *v) as *mut RasterizerState as *mut u8,
        None => std::ptr::null_mut()
    }
}

#[riri_hook_fn(static_offset(0x11b56d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetRasterizer(p_state: *mut *mut u8, p_key: *mut u8) -> *mut *mut u8 {
    let state = &mut *(p_state as *mut *mut RasterizerState);
    let renderer = globals::get_ngr_dx11_renderer_unchecked_mut();
    let key = &mut *(p_key as *mut RasterizerKey);
    *state = GfdRc::into_raw(renderer.get_or_create_rasterizer(key)) as *mut RasterizerState;
    p_state
}
#[riri_hook_fn(static_offset(0x11ba3c0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrFreeListCreate(
    p_list: *mut *const u8, 
    element_size: usize, 
    entries_per_block: usize,
    alignment: usize,
    block_count: usize,
    a6: i32,
    p_hint: *const u8
) -> *mut *const u8 {
    let new_list: GfdRc<FreeList<FreeListBlockLink<u8>>, AllocatorHook> = 
        FreeList::new_inner(alignment, entries_per_block, block_count, a6, AllocatorHook);
    let p_newlist = GfdRc::into_raw(new_list);
    *p_list = p_newlist as *const u8;
    p_list
}

#[riri_hook_fn(static_offset(0x1192f80))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetConstantBuffers(p_this: *mut u8, p_ty: u32, p_buf: *mut u8, upd: u32) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buf = &mut *(p_buf as *mut ConstantBuffer);
    this.set_constant_buffers(ty, buf, upd);
}

#[riri_hook_fn(static_offset(0x1193240))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextDraw(p_this: *mut u8, ia_topo: u32, vtx_count: u32, vtx_start: u32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let topo = ia_topo.try_into().unwrap();
    this.draw(topo, vtx_count, vtx_start);
}

#[riri_hook_fn(static_offset(0x109a590))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetBlendKeyPreset2(buf_id: u32, blend_id: u32, set_blend_key: bool) {
    opengfd::device::ngr::renderer::pkt::set_blend_key_preset2(buf_id as usize, blend_id as usize, set_blend_key);
}

#[riri_hook_fn(static_offset(0x109a4f0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetDepthStencilKeyLessEqual(buf_id: u32, set_depth_stencil: bool, set_depth_write_mask: bool) {
    opengfd::device::ngr::renderer::render::set_depth_stencil_key_less_equal(
        buf_id as usize, set_depth_stencil, set_depth_write_mask.try_into().unwrap()
    );
}

#[riri_hook_fn(static_offset(0x109a6e0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetSamplerKeyValues(buf_id: u32, sampler_id: u32, a3: bool, a4: bool, addru: u8, addrv: u8) {
    opengfd::device::ngr::renderer::render::set_sampler_key_values(
        buf_id as usize, sampler_id as usize, a3, a4, addru.try_into().unwrap(), addrv.try_into().unwrap()
    );
}

#[riri_hook_fn(static_offset(0x1094060))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetEffectScaleWidth() -> usize {
    globals::get_ngr_dx11_renderer_unchecked().get_effect_scale_width()
}

#[riri_hook_fn(static_offset(0x10940d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetEffectScaleHeight() -> usize {
    globals::get_ngr_dx11_renderer_unchecked().get_effect_scale_height()
}

#[riri_hook_fn(static_offset(0x1192ae0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetShaderSampler(p_this: *mut u8, p_ty: u32, p_id: u32, p_state: *mut u8) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = if p_state.is_null() { None } else { Some(&*(p_state as *mut SamplerState)) };
    this.set_shader_sample(ty, p_id as usize, state);
}

#[riri_hook_fn(static_offset(0x1193130))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetShaderResources(p_this: *mut u8, p_ty: u32, p_id: u32, p_state: *mut u8) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = if p_state.is_null() { None } else { Some(&*(p_state as *mut TextureResource)) };
    this.set_shader_resource_view(ty, p_id as usize, state);
}
/*
#[riri_hook_fn(static_offset(0x1192530))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrOMSetRenderTargets(p_this: *mut u8, p_rv: *const u8, p_rv2: *const u8) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let rv = if !p_rv.is_null() { Some(&*(p_rv as *const ResourceView)) } else { None };
    let rv2 = if !p_rv.is_null() { Some(&*(p_rv2 as *const ResourceView2)) } else { None };
    // original_function!(p_this, p_rv, p_rv2)
    this.om_set_render_targets(rv, rv2);
}
*/
#[riri_hook_fn(static_offset(0x1192340))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextSetViewports(p_this: *mut u8, viewports: *const f32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let viewport = &*(viewports as *const D3D11_VIEWPORT);
    this.set_viewports(viewport);
}

#[riri_hook_fn(static_offset(0x1192680))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrClearRenderTargetDepthStencil(p_this: *mut u8, flags: u32, p_colors: *const f32, depth: f32, stencil: bool) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let colors = std::mem::transmute::<*const f32, &[f32; 4]>(p_colors);
    this.clear_depth_stencil_or_render_target_view(flags, colors, depth, stencil);
}

#[riri_hook_fn(static_offset(0x11929d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrOMSetBlendState(p_this: *mut u8, p_state: *const u8) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = &*(p_state as *const BlendState);
    this.om_clear_blend_state(state);
}

#[riri_hook_fn(static_offset(0x1192a20))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrOMSetDepthStencilState(p_this: *mut u8, p_state: *const u8, stencil_ref: u8) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = &*(p_state as *const DepthStencilState);
    this.om_depth_stencil_state(state, stencil_ref);
}

#[riri_hook_fn(static_offset(0x1192bf0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextSetVertexBuffers(
    p_this: *mut u8, start_slot: u32, p_buffer: *mut u8,
    a4: usize, stride: u32, offset: u32, buffer_index: u32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buffer = if p_buffer.is_null() { None } else { Some(&*(p_buffer as *const VertexBuffer)) };
    this.set_vertex_buffers(start_slot, buffer, a4, stride, offset, buffer_index as usize);
}
#[riri_hook_fn(static_offset(0x1192cb0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextSetIndexBuffer(
    p_this: *mut u8, p_buffer: *mut u8, offset: u32, buffer_index: u32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buffer = if p_buffer.is_null() { None } else { Some(&*(p_buffer as *const IndexBuffer)) };
    this.set_index_buffer(buffer, offset, buffer_index as usize);
}
*/

#[riri_hook_fn(dynamic_offset(
    signature = "48 8B C4 53 48 83 EC 70 80 3D ?? ?? ?? ?? 00",
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdPostRender(a1: u32, a2: u32) {
    // line draw test
    // let glb = opengfd::kernel::graphics::GraphicsGlobal::get_gfd_graphics_global_mut();
    // logln!(Verbose, "cmd buffer: 0x{:x}", &raw const *glb.get_current_cmd_buffer().unwrap() as usize);
    // opengfd::debug::perf_meter::draw_test();
    original_function!(a1, a2)
}

/*
#[riri_hook_fn(static_offset(0x112ad60))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrUpdateVertexBuffers(p_this: *mut u8, buffer_index: usize) {
    let this = &mut *(p_this as *mut DrawState);
    this.update_vertex_buffers(buffer_index);
}
*/
/* crashes
#[riri_hook_fn(static_offset(0x1072ac0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderTextureSet(prio: u32, stage: u32, tex: *mut u8) {
    Render::set_texture(prio, stage, &*(tex as *mut Texture));
}
*/

/*
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdIm2DRenderPrimitive2D(prio: u32, prim: u8, data: usize, num: u32, flags: u32) {
    original_function!(prio. prim, data, num, flags)
}
*/
/*
#[riri_hook_fn(static_offset(0x1109580))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDevCmdMakeImmediateRenderPrimitivePkt(a1: usize, a2: u8, a3: u32, a4: u32, a5: u32) {
    original_function!(a1, a2, a3, a4, a5)
}
*/
/*
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetTransformWorldOtPreCallback
(_render_ot: *mut RenderOt, buffer: *mut u8, data: *mut u8) {
    let buf_id = buffer as i32;
    let data = data as *mut glam::Mat4;
    
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetTransformWorld(prio: u32, v: *mut u8) {
    let v = v as *mut glam::Mat4;
    let ot = RenderOtEx::<64>::new();
    ot.set::<glam::Mat4>(0, *v).unwrap();
    ot.set_pre_cb(gfdRenderSetTransformWorldOtPreCallback);
    ot.set_pre_cb_data(ot.data_raw());
    ot.link(prio);
}
*/

#[allow(dead_code)]
fn object_tests() {
    /* 
    let glb = Global::get_gfd_global_mut();
    let scene = glb.graphics.get_current_scene();
    if let Some(n) = scene.get_root_node() {
        let arch = n.find_by_name("mesh_arc_0006").unwrap();

        // let children = start.get_children_limited_depth(1000);
        // for ch in children {
        //     logln!(Verbose, "{}{}", "\t".repeat(ch.get_depth()), &*ch);
        // }
        for attach in opengfd::object::node::RecursiveObjectIterator::<
            GfdAllocator, opengfd::object::node::StandardNodeIterator
        >::from_node(arch) {
            logln!(Verbose, "{:?}", attach);
        }
        /* 
        let children = n.get_children_limited_depth(1000);
        for ch in children {
            logln!(Verbose, "{}{}", "\t".repeat(ch.get_depth()), &*ch);
        }
        */
        /* 
        let prop_nodes = n.collect_by_predicate(|n| n.get_property().is_some());
        for p in prop_nodes {
            logln!(Verbose, "{}", p.get_property().unwrap());
        }
        */
        /* 
        match n.find_by_name("mesh_arc_0006__combined1") {
            Some(n) => {
                for o in n.iter_object() {
                    logln!(Verbose, "{:?}", o);
                }
                // logln!(Verbose, "{}", n2);
                // logln!(Verbose, "{:?}", n2);
            },
            None => logln!(Verbose, "Could not find node mesh_arc_0006__combined1")
        };
        */
        /* 
        if let Some(n) = n.find_by_name("c_pelvis") {
            logln!(Verbose, "{}", n.fmt_hierarchy());
        }
        */
        /* 
        n.for_each_by_predicate(|v| v.get_property_entry("gfdHelperID").is_some(), |v| {
            let gfdHelperId = v.get_property().unwrap().find("gfdHelperID").unwrap();
            logln!(Verbose, "{} -> {}", v.get_name_platform(), gfdHelperId);
        });
        */
        /* 
        match n.find_by_name("mesh_arc_0006__combined1") {
            Some(n2) => {
                logln!(Verbose, "{}", n2);
                logln!(Verbose, "{:?}", n2);
            },
            None => logln!(Verbose, "Could not find node mesh_arc_0006__combined1")
        };
        */
        /* 
        let children = n.get_children_limited_depth(3);
        logln!(Verbose, "Scene graph limit = 3: {} children", children.len());
        let iter_count = 1000.min(children.len());
        for i in 0..iter_count {
            let c = &children[i];
            logln!(Verbose, "{}", &**c);
        }
        */
        /* 
        let children = n.get_children();
        logln!(Verbose, "Scene graph: {} children", children.len());
        let iter_count = 1000.min(children.len());
        for i in 0..iter_count {
            let c = children[i];
            logln!(Verbose, "<{}, T {:?} R {:?} S {:?}>", 
            c.get_name_platform(), c.get_translate(),
            c.get_rotate(), c.get_scale());
        }
        */
        /* 
        logln!(Verbose, "{} children", n.get_direct_child_count());
        for child in n.get_direct_children() {
            logln!(Verbose, "\t{}", child.get_name_platform());
            logln!(Verbose, "\t{:?}", child.get_translate());
            logln!(Verbose, "\t{:?}", child.get_rotate());
            logln!(Verbose, "\t{:?}", child.get_scale());
        }
        */
    }
    // logln!(Verbose, "test!");
    */
}


/* 
#[riri_hook_fn(dynamic_offset(
    signature = "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 30 83 0D ?? ?? ?? ?? 01",
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdExecuteActiveTasks(delta: f32) {
    if GetAsyncKeyState(VK_F5.0 as i32) & 1 != 0 
    && GfdTask::<GfdAllocator, GfdInspector>::find_by_str(GfdInspector::NAME).is_none() {
        let new_task = GfdTask::<GfdAllocator, GfdInspector>::new_update(10, 0, 0, 0, GfdAllocator);
        logln!(Verbose, "Inspector task: {}", new_task);
    }
    original_function!(delta)
}
*/
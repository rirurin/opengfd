#![allow(dead_code)]
use crate::{
    device::ngr::renderer::shader::{ 
        PixelShaderPlatform,
        PixelShader,
        VertexShaderPlatform,
        VertexShader
    },
    graphics::{
        cull::CullObject,
        render::cmd_buffer::CmdBuffer,
        render_ot::RenderOtList,
        scene::Scene
    },
    object::camera::Camera,
    kernel::global::GraphicsGlobal
};


// SHADER INFO COMMON
pub(crate) const RENDER_STATES: usize = 33;

#[repr(C)]
#[derive(Debug)]
pub struct VideoMode {
    flags: u32,
    width: i32,
    height: i32,
    depth: i32,
    ref_rate: i32,
    format: i32
}

impl GraphicsGlobal {
    /// Get a reference to the target scene graph from global state
    /// (Original function: gfdRenderGetScene)
    pub fn get_scene(&self, no: u32) -> &Scene {
        unsafe { &**self.scene.get_unchecked(no as usize) }
    }
    /// Get a mutable reference to the target scene graph from global state
    /// (Original function: gfdRenderGetScene)
    pub fn get_scene_mut(&self, no: u32) -> &mut Scene {
        unsafe { &mut **self.scene.get_unchecked(no as usize) }
    }
    /// Get a reference to the main camera for the target scene graph
    /// (Original function: gfdRenderGetSceneCamera)
    pub fn get_scene_camera(&self, no: u32) -> &Camera {
        unsafe { &*self.get_scene(no).camera }
    }
    /// Get a reference to the main camera for the target scene graph
    /// (Original function: gfdRenderGetSceneCamera)
    pub fn get_scene_camera_mut(&self, no: u32) -> &mut Camera {
        unsafe { &mut *self.get_scene(no).camera }
    }
    pub fn is_deferred_rendering_available(&self) -> bool { false }
}

impl GraphicsGlobal {
    /// Get the graphics command buffer
    /// (Original function: gfdGetCurrentCmdBuffer)
    pub unsafe fn get_current_cmd_buffer(&self) -> &mut CmdBuffer {
        &mut *self.cmd_buffer
    }
    pub unsafe fn get_geometry_cull(&self) -> &mut CullObject {
        &mut *self.ot_cull_object
    }
    pub unsafe fn get_ot_shadow_list(&self, id: usize, prio: usize) -> &mut RenderOtList {
        &mut *(*self.ot_shadow_list.get_unchecked(id)).add(prio)
    }
    pub unsafe fn get_ot_render_list(&self, id: usize, prio: usize) -> &mut RenderOtList {
        &mut *(*self.ot_render_list.get_unchecked(id)).add(prio)
    }
    pub unsafe fn get_ot_prepare_list(&self, id: usize, prio: usize) -> &mut RenderOtList {
        &mut *(*self.ot_prepare_list.get_unchecked(id)).add(prio)
    }
    pub unsafe fn get_frame_id(&self) -> usize { self.frame_id as usize }

    pub unsafe fn field44b8_clear(&mut self) {
        self.field44b8 = std::ptr::null_mut();
        self.field44c0 = std::ptr::null_mut();
    }
    pub unsafe fn get_vertex_shader_platform(&self, index: usize) -> Option<&VertexShaderPlatform> {
        (&**self.shader_vertex.get_unchecked(index)).data.as_ref()
    }
    pub unsafe fn get_pixel_shader_platform(&self, index: usize) -> Option<&PixelShaderPlatform> {
        (&**self.shader_pixel.get_unchecked(index)).data.as_ref()
    }
    pub unsafe fn get_current_vertex_shader(&self) -> Option<&VertexShader> {
        self.shader_current_vertex.as_ref()
    }
    pub unsafe fn get_current_pixel_shader(&self) -> Option<&PixelShader> {
        self.shader_current_fragment.as_ref()
    }
    pub unsafe fn get_current_vertex_shader_ptr(&mut self) -> *mut *mut VertexShader {
        &raw mut self.shader_current_vertex
    }
    pub unsafe fn get_current_pixel_shader_ptr(&mut self) -> *mut *mut PixelShader {
        &raw mut self.shader_current_fragment
    }
}

#![allow(dead_code, unused_imports)]
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
};


// SHADER INFO COMMON
pub(crate) const RENDER_STATES: usize = 33;
pub(crate) const SCENE_LISTS: usize = 2;
pub(crate) const MATERIAL_LISTS: usize = 8;
pub(crate) const RENDER_LISTS: usize = 3;
// pub(crate) const SHADER_SOURCE: usize = 219; // (before 1.0.13)
pub(crate) const SHADER_SOURCE: usize = 221;
pub(crate) const FIXED_VERTEX_SHADERS: usize = 70;
pub(crate) const FIXED_PIXEL_SHADERS: usize = 198; // (before 1.0.13)
// pub(crate) const FIXED_PIXEL_SHADERS: usize = 200;
pub(crate) const FIXED_GEOMETRY_SHADERS: usize = 1;
pub(crate) const FIXED_COMPUTE_SHADERS: usize = 30;
pub(crate) const OT_GROUP_COUNT: usize = 7;
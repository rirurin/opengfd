#![allow(unused_imports)]
use glam::{ Vec2, Vec3 };
use crate::{
    globals,
    graphics::render::cmd_buffer::CmdBuffer,
    utility::misc::RGBA
};
use std::mem::size_of;
/*
pub trait Draw2D {
    fn render(prio: u32, ty: u8, vtx: &[Self], flags: u32) where Self : Sized;
    fn render_indexed(prio: u32, ty: u8, vtx: &[Self], indices: &[u32], idxf: u16, flags: u32) where Self : Sized;
}

#[repr(C)]
#[derive(Debug)]
pub struct Im2DVertex {
    pos: Vec3,
    color: RGBA,
    texcoord: Vec2
}

#[repr(C)]
#[derive(Debug)]
pub struct Im2DVertexG4 {
    pos: Vec3,
    color: RGBA,
}

#[repr(C)]
#[derive(Debug)]
pub struct Im2DVertexM4 {
    pos: Vec3,
    color: RGBA,
    coord0: Vec2,
    coord1: Vec2
}

#[repr(C)]
#[derive(Debug)]
pub struct Im2DVertexT4 {
    pos: Vec3,
    texcoord: Vec2
}
*/
/*
pub struct Draw2D<T>;
impl<T> Draw2D<T> {
    // fn render(prio: i32, ty: u8, vtx: &[T], flags: u32)
}
*/

pub trait Draw2D {

}

#[repr(C)]
#[derive(Debug)]
pub struct ImmediateRenderPktPayload {
    input_assembler_cb: *mut u8,
    field08: u32,
    field10: usize,
    vertex_count: u32,
    vertex_stride: u32,
    buf_size: u32,
    frame_id: u32,
    draw_cb: *mut u8,
    draw_type: u8,
    field34: u32,
    field38: u32,
    field40: usize
}

impl ImmediateRenderPktPayload {
    pub unsafe fn new<T>(ty: u8, vtx: &[T]) -> &'static mut Self {
        let glb = globals::get_gfd_global_unchecked_mut();
        let out = glb.graphics.get_current_cmd_buffer().alloc_ex::<ImmediateRenderPktPayload>();
        out
    }
}

// original function: gfdDevCmdImmediateRenderPrimitivePkt
/*
pub unsafe fn make_immediate_render_pkt<T>(ty: u8, vtx: &[T]) -> &'static mut ImmediateRenderPktPayload {
    let glb = globals::get_gfd_global_unchecked_mut();
    let out = glb.graphics.get_current_cmd_buffer().alloc_ex::<ImmediateRenderPktPayload>();
}
*/

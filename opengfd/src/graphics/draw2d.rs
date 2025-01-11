#![allow(unused_imports)]
use bitflags::bitflags;
use glam::{ Vec2, Vec3, Vec4 };
use crate::{
    device::ngr::renderer::{
        platform::d3d::PlatformCmdBuffer,
        state::IATopology
    },
    globals,
    graphics::render::{
        cmd_buffer::{ CmdBuffer, CmdBufferInterface },
        render::Render
    },
    utility::misc::RGBA
};
use std::mem::size_of;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RenderFlag: u8 {
        const ALREADY_COPIED_TO_BUFFER = 1 << 0;
        const FLAG1 = 1 << 1;
        const FLAG2 = 1 << 2;
        const FLAG3 = 1 << 3;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImmediateRenderType {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
    Indexed = 5
}

impl From<ImmediateRenderType> for IATopology {
    fn from(value: ImmediateRenderType) -> Self {
        match value {
            ImmediateRenderType::PointList => IATopology::PointList,
            ImmediateRenderType::LineList => IATopology::LineList,
            ImmediateRenderType::LineStrip => IATopology::LineStrip,
            ImmediateRenderType::TriangleList => IATopology::TriangleList,
            ImmediateRenderType::TriangleStrip => IATopology::TriangleStrip,
            ImmediateRenderType::Indexed => IATopology::Undefined
        }
    }
}

pub trait Draw2D {
    fn render(prio: u32, ty: ImmediateRenderType, vtx: &[Self], flags: RenderFlag) where Self : Sized;
    fn set_positions(vtx: &mut [Self], pos: &[Vec3]) where Self : Sized;
    fn set_colors(vtx: &mut [Self], color: &[RGBA]) where Self : Sized;
    fn set_color(vtx: &mut [Self], color: RGBA) where Self: Sized;
    // fn render_indexed(prio: u32, ty: u8, vtx: &[Self], indices: &[u32], idxf: u16, flags: RenderFlag) where Self : Sized;
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

impl Draw2D for Im2DVertexG4 {
    fn render(prio: u32, ty: ImmediateRenderType, vtx: &[Self], flags: RenderFlag) where Self : Sized {
        let vtx = if flags.contains(RenderFlag::ALREADY_COPIED_TO_BUFFER) {
            vtx } else {
            let renderer = unsafe { globals::get_ngr_dx11_renderer_unchecked_mut() };
            unsafe { renderer.get_command_buffer_unchecked_mut().copy_from_slice(vtx) }
        };
        let global = unsafe { globals::get_gfd_global_unchecked() };
        unsafe {
            Render::bind_vertex_shader(prio, global.graphics.get_vertex_shader_unchecked(4));
            Render::bind_pixel_shader(prio, global.graphics.get_pixel_shader_unchecked(7));
            Render::make_immediate_render(prio, ty, vtx.len() as u32, vtx.as_ptr() as *mut u8, size_of::<Self>() as u32, 0x42);
        }
    }
    fn set_positions(vtx: &mut [Self], pos: &[Vec3]) where Self : Sized {
        for (v, p) in vtx.iter_mut().zip(pos.iter()) { v.pos = *p; }
    }
    fn set_colors(vtx: &mut [Self], color: &[RGBA]) where Self : Sized {
        for (v, c) in vtx.iter_mut().zip(color.iter()) { v.color = *c; }
    }
    fn set_color(vtx: &mut [Self], color: RGBA) where Self: Sized {
        for v in vtx.iter_mut() { v.color = color; }
    }
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

pub struct Draw<'a, const P: u32, T = Im2DVertexG4, A = PlatformCmdBuffer>
where A: CmdBufferInterface,
      T: Draw2D
{
    alloc: &'a mut A,
    draw_type: std::marker::PhantomData<T>
}
impl<'a, const P: u32, T, A> Draw<'a, P, T, A>
where A: CmdBufferInterface,
      T: Draw2D + std::fmt::Debug
{
    pub fn new(alloc: &'a mut A) -> Self {
        Self { alloc, draw_type: std::marker::PhantomData }
    }
    /// Original function: gfdPrimLine2D
    pub fn draw_line_solid_color(&mut self, color: RGBA, p0: Vec3, p1: Vec3) {
        let vtx: &mut [T] = unsafe { self.alloc.alloc_to_slice_mut(2) };
        T::set_positions(vtx, &[p0, p1]);
        T::set_color(vtx, color);
        T::render(P, ImmediateRenderType::LineList, vtx, RenderFlag::ALREADY_COPIED_TO_BUFFER);
    }
    /// Original function: gfdPrimRectangle2D
    pub fn draw_rectangle_solid_color(&mut self, color: RGBA, x: f32, y: f32, width: f32, height: f32) {
        let vtx = unsafe { self.alloc.alloc_to_slice_mut(4) };
        let positions = [ 
            Vec3::new(x, y, 0f32), Vec3::new(x + width, y, 0f32),
            Vec3::new(x, y + height, 0f32), Vec3::new(x + width, y + height, 0f32),
        ];
        T::set_positions(vtx, &positions);
        T::set_color(vtx, color);
        T::render(P, ImmediateRenderType::TriangleStrip, vtx, RenderFlag::ALREADY_COPIED_TO_BUFFER);
    }
}

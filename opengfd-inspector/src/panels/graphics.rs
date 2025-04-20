use opengfd_inspector_components::{
    panel::InspectorPanel,
    table::{ InspectorTable, TableDraw }
};
use imgui::{ TableFlags, Ui };
use opengfd::{
    device::ngr::renderer::shader::VertexShader,
    graphics::{
        texture::Texture,
        render::cmd_buffer::DEFAULT_CMD_BUFFER_SIZE
    },
    kernel::graphics::{ GraphicsGlobal, GraphicsFlags }
};
use std::ops::Deref;

pub struct TextureTableEntry<'a>(&'a Texture);
impl<'a> Deref for TextureTableEntry<'a> {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> TableDraw<GraphicsPanel> for TextureTableEntry<'a> {
    fn draw_contents(&self, ui: &mut Ui, _ctx: &mut GraphicsPanel, index: usize) {
        match index {
            0 => ui.text(format!("{}", self.get_name())),
            1 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct GraphicsFlagEntry(usize);
impl Deref for GraphicsFlagEntry {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0 
    }
}
impl TableDraw<GraphicsFlags> for GraphicsFlagEntry {
    fn draw_contents(&self, ui: &mut Ui, ctx: &mut GraphicsFlags, index: usize) {
        match index {
            0 => GraphicsPanel::checkbox_for_graphics_flag(ctx, **self << 1, ui),
            1 => GraphicsPanel::checkbox_for_graphics_flag(ctx, (**self << 1) + 1, ui),
            _ => ()
        }
    }
}

pub struct VertexShaderTableEntry<'a>(&'a VertexShader);
impl<'a> Deref for VertexShaderTableEntry<'a> {
    type Target = VertexShader;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> TableDraw<GraphicsPanel> for VertexShaderTableEntry<'a> {
    fn draw_contents(&self, ui: &mut Ui, _ctx: &mut GraphicsPanel, index: usize) {
        match index {
            0 => ui.text("TODO"),
            // 0 => ui.text(format!("{}", self.get_name())),
            1 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

#[derive(Debug)]
pub struct GraphicsPanel;
impl InspectorPanel for GraphicsPanel {
    fn get_panel_name(&self) -> &'static str { "Graphics" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        let self_ptr = unsafe { &mut *(&raw mut *self) };
        let graphics = GraphicsGlobal::get_gfd_graphics_global_mut();
        let frame_id = graphics.get_frame_id();
        // graphics flags
        if ui.tree_node("Graphics Flags").is_some() {
            ui.text("WARNING: Many of these flags will crash the game if you toggle them!");
            let mut flags = graphics.get_flags_mut();
            let mut flag_table = InspectorTable::<GraphicsFlagEntry, GraphicsFlags, 2>::new(
                "Graphics Flags",
                None,
                TableFlags::SCROLL_Y,
                opengfd_inspector_components::table::default_height(),
            );
            let flag_entries: Vec<GraphicsFlagEntry> = (0..(u32::BITS as usize) >> 1).into_iter().map(|v| GraphicsFlagEntry(v)).collect();
            flag_table.draw_table(ui, &mut flags, flag_entries.as_slice());
            
        }
        // no need to show video mode, that stuff is handled by ngr
        ui.separator();
        // command buffer
        if let Some(v) = graphics.get_current_cmd_buffer() {
            let buf_base = v.get_buffer_val(frame_id) as usize;
            let alloc_size = buf_base - v.get_ptr_val();
            let usage = (alloc_size as f32 / DEFAULT_CMD_BUFFER_SIZE as f32) * 100.;
            ui.text(format!("frame {}, buffer start: 0x{:x} (used: 0x{:x} - usage {}%)", frame_id, buf_base, alloc_size, usage));
        } else { ui.text("Command Buffer is not initialized!"); }
        ui.separator();
        // texture
        let mut tex = InspectorTable::<TextureTableEntry, Self, 2>::new(
            "Texture List",
            Some([
                "Name",
                "Address",
            ]),
            opengfd_inspector_components::table::default_flags(),
            opengfd_inspector_components::table::default_height(),
        );
        let tex_mutex = graphics.lock_texture_mutex();
        let mut tex_entries = vec![];
        if (*tex_mutex) != std::ptr::null_mut() {
            let mut tex = unsafe { (*tex_mutex).as_ref() };
            // let mut tex = graphics.get_texture_head();
            while let Some(t) = tex {
                tex_entries.push(TextureTableEntry(t));
                tex = t.get_next();
            }
        }
        tex.draw_table(ui, self_ptr, tex_entries.as_slice());
        ui.text(format!("{} texture entries", tex_entries.len()));
        drop(tex_mutex);
        ui.separator();
        // vertex shdaers
        let mut vtx_shader = InspectorTable::<VertexShaderTableEntry, Self,2>::new(
            "Vertex Shader List",
            Some([
                "Name",
                "Address",
            ]),
            opengfd_inspector_components::table::default_flags(),
            opengfd_inspector_components::table::default_height(),
        );
        let vtx_mutex = graphics.lock_vertex_shader_mutex();
        let mut vtx_entries = vec![];
        if (*vtx_mutex) != std::ptr::null_mut() {
            let mut vtx = unsafe { (*vtx_mutex).as_ref() };
            // let mut tex = graphics.get_texture_head();
            while let Some(t) = vtx {
                vtx_entries.push(VertexShaderTableEntry(t));
                vtx = t.get_next();
            }
        }
        vtx_shader.draw_table(ui, self_ptr, vtx_entries.as_slice());
        ui.text(format!("{} vertex shaders", vtx_entries.len()));
        drop(vtx_mutex);
        ui.separator();
        // ot data
        ui.text(format!("OT Priority: max {}, widget: {}, debug font: {}, mouse: {}", 
            graphics.get_max_ot_priority(), graphics.get_widget_ot_priority(),
            graphics.get_debug_font_ot_priority(), graphics.get_mouse_ot_priority()
        ));
        ui.separator();
        ui.text("TODO: Shader data!");
    }
}
impl GraphicsPanel {
    pub(crate) fn new() -> Self { Self }

    fn checkbox_for_graphics_flag(flags: &mut GraphicsFlags, flag_id: usize, ui: &mut Ui) {
        let target_flag = GraphicsFlags::from_bits_truncate(1 << flag_id as u32);
        let mut value = flags.contains(target_flag);
        if ui.checkbox(format!("{:?}", target_flag), &mut value) {
            flags.set(target_flag, value);
        }
    }
}
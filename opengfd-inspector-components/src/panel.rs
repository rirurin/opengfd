use glam::Vec2;
use imgui::{
    Condition as ImCond,
    TabItemFlags,
    Ui
};
use std::fmt::Debug;
pub trait InspectorPanel : std::fmt::Debug {
    fn get_panel_name(&self) -> &'static str;
    fn draw(&mut self, ui: &mut Ui) {
        let ui_new = unsafe { &mut *(&raw mut *ui) };
        if let Some(_) = ui.tab_item_with_flags(self.get_panel_name(), Some(&mut true), TabItemFlags::empty()) {
            self.draw_contents(ui_new);
        }
    }
    fn draw_contents(&mut self, ui: &mut Ui);
}

pub trait BasicPanel {
    fn draw(&mut self, ui: &mut Ui);
}

pub trait InspectorWindow : Debug {
    fn get_name(&self) -> &str;
    fn get_open_state(&mut self) -> &mut bool;
    fn get_default_size(&self) -> Vec2 { Vec2::new(100., 200.) }
    fn get_default_position(&self) -> Vec2 { Vec2::new(30., 30.,) }
    fn draw(&mut self, ui: &mut Ui) {
        let size: [f32; 2] = self.get_default_size().into();
        let pos: [f32; 2] = self.get_default_position().into();
        let ui_into = unsafe { &mut *(&raw mut *ui) };
        let self_into = unsafe { &mut *(&raw mut *self) };
        if let Some(_) = ui.window(self.get_name())
            .size(size, ImCond::FirstUseEver)
            .position(pos, ImCond::FirstUseEver)
            .opened(self_into.get_open_state())
            .begin() {
            self.draw_contents(ui_into);
        }
    }
    fn draw_contents(&mut self, ui: &mut Ui);
}
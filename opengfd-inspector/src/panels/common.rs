use imgui::{
    TabItemFlags,
    Ui
};
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
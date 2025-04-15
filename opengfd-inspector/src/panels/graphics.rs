use crate::panels::common::InspectorPanel;
use imgui::Ui;

#[derive(Debug)]
pub struct GraphicsPanel;
impl InspectorPanel for GraphicsPanel {
    fn get_panel_name(&self) -> &'static str { "Graphics" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        ui.text("TODO: Graphics");
    }
}
impl GraphicsPanel {
    pub(crate) fn new() -> Self { Self }
}
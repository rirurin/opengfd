use crate::panels::common::InspectorPanel;
use imgui::Ui;

#[derive(Debug)]
pub struct ResourcesPanel;
impl InspectorPanel for ResourcesPanel {
    fn get_panel_name(&self) -> &'static str { "Resources" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        ui.text("TODO: Resources");
    }
}
impl ResourcesPanel {
    pub(crate) fn new() -> Self { Self }
}
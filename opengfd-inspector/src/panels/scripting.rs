use crate::panels::common::InspectorPanel;
use imgui::Ui;

#[derive(Debug)]
pub struct ScriptingPanel;
impl InspectorPanel for ScriptingPanel {
    fn get_panel_name(&self) -> &'static str { "Scripting" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        ui.text("TODO: Scripting");
    }
}
impl ScriptingPanel {
    pub(crate) fn new() -> Self { Self }
}
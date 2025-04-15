use crate::panels::common::InspectorPanel;
use imgui::Ui;

#[derive(Debug)]
pub struct SceneGraphPanel;
impl InspectorPanel for SceneGraphPanel {
    fn get_panel_name(&self) -> &'static str { "Scene Graph" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        ui.text("TODO: Scene Graph");
    }
}
impl SceneGraphPanel {
    pub(crate) fn new() -> Self { Self }
}
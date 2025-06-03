use riri_inspector_components::panel::InspectorPanel;
use imgui::Ui;

#[derive(Debug)]
pub struct AboutPanel;
impl InspectorPanel for AboutPanel {
    fn get_panel_name(&self) -> &'static str { "About" }
    fn draw_contents(&mut self, ui: &Ui) {
        ui.text("OpenGFD Inspector by Rirurin");
        ui.text("Tested with Metaphor: Refantazio");
        ui.text("To report any issues, either make a new issue on the repository at ");
        ui.text("https://github.com/rirurin/opengfd/issues");
        ui.text("Or create a user-help thread on Persona Modding at https://discord.gg/naoto and ping me");
    }
}
impl AboutPanel {
    pub(crate) fn new() -> Self { Self }
}
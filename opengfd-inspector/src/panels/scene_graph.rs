use crate::{
    components::{
        searchbar::Searchbar,
        table::{ InspectorTable, TableDraw }
    },
    panels::common::InspectorPanel
};
use imgui::Ui;
use opengfd::object::object::Object;
use std::ops::Deref;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TaskObjectEntry(&'static Object);
impl Deref for TaskObjectEntry {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl TableDraw<SceneGraphPanel> for TaskObjectEntry {
    fn draw_contents(&self, ui: &mut Ui, _ctx: &mut SceneGraphPanel, index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            1 => ui.text("TODO: Type"),
            _ => ()
        }
    }
}

#[derive(Debug)]
pub struct SceneGraphPanel {
    search: Searchbar,
    hierarchy: InspectorTable<'static, TaskObjectEntry, SceneGraphPanel, 2>
}
impl SceneGraphPanel {
    /* 
    fn traverse_scene_graph(&self) {
        // let glb = Global::get_gfd_global();
        // glb.graphics.get_scene()
    }
    */
}
impl InspectorPanel for SceneGraphPanel {
    fn get_panel_name(&self) -> &'static str { "Scene Graph" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        let self_ptr = unsafe { &mut *(&raw mut *self) };
        ui.text(&format!("want keyboard: {}", ui.io().want_capture_keyboard));
        self.search.draw(ui);
        self.hierarchy.draw_table(ui, self_ptr);
        ui.text("TODO: Scene Graph");
    }
}
impl SceneGraphPanel {
    pub(crate) fn new() -> Self { 
        Self {
            search: Searchbar::new("SearchSceneGraph", false),
            hierarchy: InspectorTable::new(
                "Scene Graph List",
                Some([
                    "Name",
                    "Type"
                ]),
                crate::components::table::default_flags(),
                crate::components::table::default_height()
            )
        }
    }
}
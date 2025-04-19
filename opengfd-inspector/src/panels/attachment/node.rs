use imgui::Ui;
use opengfd::object::{
    object::Object,
    node::Node
};
use std::ops::{ Deref, DerefMut };
#[derive(Debug)]
pub struct NodeProperties(&'static mut Node);
impl Deref for NodeProperties {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        self.0
    }
} 
impl DerefMut for NodeProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
impl NodeProperties {
    pub unsafe fn new(obj: &'static Object) -> Self {
        let node = &mut *(&raw const *obj as *mut Node);
        Self(node)
    }
    pub(crate) fn draw_contents(&mut self, ui: &mut Ui) {
        ui.input_float3("Local Translation", self.get_translate_mut_f32()).build();
        ui.input_float4("Rotate", self.get_rotate_mut_f32()).build();
        ui.input_float3("Scale", self.get_scale_mut_f32()).build();
        ui.separator();
    }
}
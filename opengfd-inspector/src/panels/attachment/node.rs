use crate::panels::attachment::property::UserPropertyDetails;
use riri_inspector_components::panel::BasicPanel;
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
        // Self(unsafe { std::mem::transmute::<_, &mut Node>(obj) })
        Self(unsafe { &mut *(&raw const *obj as *mut Node) })
    }
}
impl BasicPanel for NodeProperties {
    fn draw(&mut self, ui: &Ui) {
        ui.input_float3("World Translation", self.get_world_transform_mut_f32()).build();
        ui.input_float3("Local Translation", self.get_translate_mut_f32()).build();
        ui.input_float4("Rotate", self.get_rotate_mut_f32()).build();
        ui.input_float3("Scale", self.get_scale_mut_f32()).build();
        ui.separator();
        let mut user_prop_details = UserPropertyDetails::from_node(&mut **self);
        user_prop_details.draw(ui);
    }
}
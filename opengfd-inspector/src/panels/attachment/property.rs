use riri_inspector_components::{
    panel::BasicPanel,
    table::{ InspectorTable, TableDraw }
};
use imgui::Ui;
use opengfd::{
    object::node::Node,
    utility::property::{ Property, PropertyChunk }
};
use std::ops::Deref;
#[derive(Debug)]
// pub struct UserPropertyDetails(UnsafeCell<&'static mut Node>);
pub struct UserPropertyDetails<'a>(Option<&'a mut Property>);
// #[derive(Debug)]
pub struct UserPropertyTableEntry<'a>(&'a PropertyChunk);
impl<'a> Deref for UserPropertyTableEntry<'a> {
    type Target = PropertyChunk;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> TableDraw<()> for UserPropertyTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => match self.get_name() {
                Some(v) => ui.text(format!("{}", v)),
                None => ui.text("NO NAME")
            },
            1 => ui.text(format!("{:?}", self.get_property_type())),
            2 => ui.text(format!("{}", self.format_data())),
            _ => ()
        }
    }
}
impl<'a> UserPropertyDetails<'a> {
    pub fn from_node(node: &'a mut Node) -> Self {
        Self(node.get_property_mut())
    }
}
impl<'a> BasicPanel for UserPropertyDetails<'a> {
    fn draw(&mut self, ui: &Ui) {
        match self.0.as_mut() {
            Some(p) => {
                ui.text(format!("{} properties:", p.len()));
                let table_name = format!("Property Table## PropTable for 0x{:x}", &raw const **p as usize);
                let mut prop_table = InspectorTable::<UserPropertyTableEntry, (), 3>::new(
                    &table_name,
                    Some([
                        "Name",
                        "Type",
                        "Value"
                    ]),
                    riri_inspector_components::table::default_flags(),
                    150.,
                );
                let entries: Vec<UserPropertyTableEntry> = p.into_iter().map(|v| UserPropertyTableEntry(v)).collect();
                prop_table.draw_table(ui, &mut (), entries.as_slice());
            },
            None => {
                ui.text("No properties");
            }
        }
    }
}
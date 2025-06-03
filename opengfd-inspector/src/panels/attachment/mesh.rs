use opengfd_inspector_components::bounding_box::{ 
    draw_bounding_box, 
    draw_bounding_circle,
};
use riri_inspector_components::{
    panel::BasicPanel,
    table::{ InspectorTable, TableDraw }
};
use imgui::Ui;
use opengfd::{
    graphics::material::Material,
    // kernel::allocator::GfdAllocator,
    object::{
        camera::Camera,
        epl::EPL,
        geometry::Geometry,
        light::Light,
        mesh::Mesh,
        morph::MorphController,
        node::Node,
        object::Object,
    }
};
use std::ops::{ Deref, DerefMut };
#[derive(Debug)]
pub struct MeshProperties(&'static mut Mesh);
impl Deref for MeshProperties {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        self.0
    }
} 
impl DerefMut for MeshProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
impl MeshProperties {
    pub unsafe fn new(obj: &'static Object) -> Self {
        Self(unsafe { &mut *(&raw const *obj as *mut Mesh) })
    }
}

pub trait MeshTableEntry<'a> : Deref {
    const DISP_NAME: &'static str;
    fn new(val: &'a <Self as Deref>::Target) -> Self;
}

pub struct NodeTableEntry<'a>(&'a Node);
impl<'a> Deref for NodeTableEntry<'a> {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for NodeTableEntry<'a> {
    const DISP_NAME: &'static str = "Node";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for NodeTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct GeometryTableEntry<'a>(&'a Geometry);
impl<'a> Deref for GeometryTableEntry<'a> {
    type Target = Geometry;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for GeometryTableEntry<'a> {
    const DISP_NAME: &'static str = "Geometry";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for GeometryTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct MaterialTableEntry<'a>(&'a Material);
impl<'a> Deref for MaterialTableEntry<'a> {
    type Target = Material;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for MaterialTableEntry<'a> {
    const DISP_NAME: &'static str = "Material";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for MaterialTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct MorphTableEntry<'a>(&'a MorphController);
impl<'a> Deref for MorphTableEntry<'a> {
    type Target = MorphController;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for MorphTableEntry<'a> {
    const DISP_NAME: &'static str = "Morph";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for MorphTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct CameraTableEntry<'a>(&'a Camera);
impl<'a> Deref for CameraTableEntry<'a> {
    type Target = Camera;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for CameraTableEntry<'a> {
    const DISP_NAME: &'static str = "Camera";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for CameraTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct LightTableEntry<'a>(&'a Light);
impl<'a> Deref for LightTableEntry<'a> {
    type Target = Light;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for LightTableEntry<'a> {
    const DISP_NAME: &'static str = "Light";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for LightTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

pub struct EPLTableEntry<'a>(&'a EPL);
impl<'a> Deref for EPLTableEntry<'a> {
    type Target = EPL;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> MeshTableEntry<'a> for EPLTableEntry<'a> {
    const DISP_NAME: &'static str = "EPL";
    fn new(val: &'a <Self as Deref>::Target) -> Self {
        Self(val)
    }
}
impl<'a> TableDraw<()> for EPLTableEntry<'a> {
    fn draw_contents(&self, ui: &Ui, _ctx: &mut (), index: usize) {
        match index {
            0 => ui.text(format!("0x{:x}", &raw const **self as usize)),
            _ => ()
        }
    }
}

impl MeshProperties {
    fn draw_object_list<'a, TContent>(
        &'a self, 
        ui: &'a  Ui, 
        cb: fn(&'a <Self as Deref>::Target) -> &'a [*mut <TContent as Deref>::Target]
    ) where TContent: TableDraw<()> + MeshTableEntry<'a> {
        let table_name = format!("{}## {}Table for 0x{:x}", TContent::DISP_NAME, TContent::DISP_NAME, &raw const **self as usize);
        let mut table = InspectorTable::<TContent, (), 1>::new(
            &table_name, 
            None, 
            riri_inspector_components::table::default_flags(), 
            // crate::components::table::default_flags(), 
            100. 
        );
        let entries: Vec<TContent> = cb(self).iter().map(|v| TContent::new(unsafe { &**v })).collect();
        ui.text(format!("{}: {} entries", TContent::DISP_NAME, entries.len()));
        if entries.len() > 0 {
            table.draw_table(ui, &mut (), entries.as_slice());
        }
    }
}

impl BasicPanel for MeshProperties {
    fn draw(&mut self, ui: &Ui) {
        ui.text(format!("Mesh flags: {:?}", self.get_flags()));
        ui.separator();
        // object lists
        self.draw_object_list::<NodeTableEntry>(ui, <Self as Deref>::Target::get_node_list);
        self.draw_object_list::<GeometryTableEntry>(ui, <Self as Deref>::Target::get_geometry_list);
        self.draw_object_list::<MaterialTableEntry>(ui, <Self as Deref>::Target::get_material_list);
        self.draw_object_list::<MorphTableEntry>(ui, <Self as Deref>::Target::get_morph_list);
        self.draw_object_list::<CameraTableEntry>(ui, <Self as Deref>::Target::get_camera_list);
        self.draw_object_list::<LightTableEntry>(ui, <Self as Deref>::Target::get_light_list);
        self.draw_object_list::<EPLTableEntry>(ui, <Self as Deref>::Target::get_effect_list);
        ui.separator();
        // bounding box
        draw_bounding_box(self.get_bounding_box_mut(), ui);
        draw_bounding_circle(self.get_bounding_sphere_mut(), ui);
        ui.separator();
        // animation data
        ui.text("Animation data TODO");
        ui.separator();
        // skinning data
        ui.text("Skinning data TODO");
        ui.separator();
        // gradation data
        ui.text("Gradation data TODO");
    }
}
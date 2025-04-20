use bitflags::bitflags;
use crate::panels::attachment::{
    camera::CameraProperties,
    mesh::MeshProperties,
    node::NodeProperties
};
use opengfd_inspector_components::{
    panel::{
        BasicPanel,
        InspectorPanel
    },
    searchbar::Searchbar,
    table::{ InspectorTable, TableDraw }
};
use imgui::{ Direction, Ui };
use opengfd::{
    graphics::scene::SceneField,
    kernel::{
        allocator::GfdAllocator,
        graphics::GraphicsGlobal
    },
    object::{
        node::{ 
            Node, 
            RecursiveObjectIterator,
            StandardNodeIterator
        },
        object::{ Object, ObjectId }
    }
};
use std::{
    collections::HashSet,
    ops::Deref
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TaskObjectEntry {
    object: &'static Object,
    name: String,
    depth: usize,
}
impl Deref for TaskObjectEntry {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        self.object
    }
}
impl TaskObjectEntry {
    fn get_object_name(object: &'static Object) -> String {
        match object.get_id() {
            ObjectId::Node => {
                let node= unsafe { &*(&raw const *object as *const Node) };
                match node.get_name() {
                    Some(v) => format!("{}##@ 0x{:x}", v, &raw const *node as usize),
                    None => format!("{:?} @ 0x{:x}", object.get_id(), &raw const *node as usize)
                }
            },
            _ => format!("{:?} @ 0x{:x}", object.get_id(), &raw const *object as usize)
        }
    }
    fn new(object: &'static Object) -> Self {
        Self { object, name: Self::get_object_name(object), depth: 0 }
    }
    fn new_with_depth(object: &'static Object, depth: usize) -> Self {
        Self { object, name: Self::get_object_name(object), depth }
    }
    fn new_field(field: &'static SceneField) -> Self {
        let name = match field.get_name().get_string() {
            Some(v) => v.to_owned(),
            None => Self::get_object_name(field.get_object().unwrap())
        };
        Self { object: field.get_object().unwrap(), name, depth: 0 }
    }
    fn add_hierarchy(
        vec: &mut Vec<Self>, 
        obj: &'static Object, 
        opened_nodes: &HashSet<&'static Object>,
        depth: usize
    ) {
        if obj.get_id() != ObjectId::Node { return; }
        let node = unsafe { &*(&raw const *obj as *const Node) };
        vec.push(TaskObjectEntry::new_with_depth(&obj, depth));
        if opened_nodes.contains(&obj) {
            for ch in node.get_direct_children() {
                let ch_obj = unsafe { &*(&raw const *ch as *const Object) };
                Self::add_hierarchy(vec, ch_obj, opened_nodes, depth + 1);
            }
        }
    }
}
impl TableDraw<SceneGraphPanel> for TaskObjectEntry {
    fn draw_contents(&self, ui: &mut Ui, ctx: &mut SceneGraphPanel, index: usize) {
        match index {
            // OBJECT NAME
            0 => {
                let obj= unsafe { &*(&raw const **self) };
                let is_leaf = match self.get_id() {
                    ObjectId::Node => {
                        let node= unsafe { &*(&raw const **self as *const Node) };
                        node.get_child_count() == 0
                    },
                    _ => true
                };
                let is_open = !is_leaf && ctx.nodes_opened.contains(&obj);
                ui.same_line_with_spacing(0., 10. * self.depth as f32);
                if is_leaf {
                    opengfd_inspector_components::bullet::bullet_ex(ui, self.depth);
                } else {
                    let arrow_dir = match is_open {
                        true => Direction::Down,
                        false => Direction::Right
                    };
                    opengfd_inspector_components::bullet::arrow_ex(ui, arrow_dir, self.depth);
                } 
                if ui.selectable_config(&self.name).span_all_columns(true).build() {
                    // check nodes opened for table tree
                    if obj.get_id() == ObjectId::Node {
                        if is_open {
                            ctx.nodes_opened.remove(&obj);
                        } else {
                            ctx.nodes_opened.insert(obj);
                        }
                    }
                    // check objects opened for details
                    if !ctx.selected_attachment_history.contains(&obj) {
                        ctx.selected_attachment_history.push(obj);
                    }
                }
            },
            // OBJECT TYPE
            1 => ui.text(format!("{:?}", self.get_id())),
            _ => ()
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DisjointAttachmentFlags: u8 {
        const Meshes = 1 << 0;
        const EPL = 1 << 1;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SceneGraphPanel {
    search: Searchbar,
    hierarchy: InspectorTable<'static, TaskObjectEntry, SceneGraphPanel, 2>,
    scene_graph_visible: Vec<TaskObjectEntry>,
    nodes_opened: HashSet<&'static Object>,
    selected_attachment_history: Vec<&'static Object>,
    show_root_node_hierarchy: bool,
    show_default_camera: bool,
    show_disjoint_meshes: bool,
    show_disjoint_epls: bool,
    root_node: Option<&'static Object>,
    default_camera: Option<&'static Object>,
}
impl SceneGraphPanel {
    fn get_disjoint_attachment(id: ObjectId, vec: &mut Vec<TaskObjectEntry>) {
        let glb = GraphicsGlobal::get_gfd_graphics_global();
        if let Some(s) = glb.get_current_scene() {
            for mesh in s.iter_attachment_type(id) {
                vec.push(TaskObjectEntry::new_field(mesh));
            }
        }
    }
    fn traverse_scene_graph(&self) -> Vec<TaskObjectEntry> {
        // update scene graph nodes
        let mut entries = vec![];
        if self.show_root_node_hierarchy && self.root_node.is_some() {
            TaskObjectEntry::add_hierarchy(&mut entries, self.root_node.unwrap(), &self.nodes_opened,0 );
        }
        if self.show_default_camera && self.default_camera.is_some() {
            entries.push(TaskObjectEntry::new(self.default_camera.unwrap()));
        }
        if self.show_disjoint_meshes {
            Self::get_disjoint_attachment(ObjectId::Mesh, &mut entries);
        }
        if self.show_disjoint_epls {
            Self::get_disjoint_attachment(ObjectId::EPL, &mut entries);
        }
        entries
    }
}
impl SceneGraphPanel {

    fn is_object_default_camera(&self, obj: &Object) -> bool {
        match self.default_camera.as_ref() {
            Some(v) => std::ptr::addr_eq(*v, obj),
            None => false
        }
    }

    // check global scene graph
    fn history_last_valid_object(&mut self) {
        while self.selected_attachment_history.len() > 0 {
            let att = self.selected_attachment_history.pop();
            if let Some(p) = att {
                if !self.is_object_invalid(p) { break; }
            }
        }
    }

    fn is_object_invalid(&self, obj: &Object) -> bool {
        let root = match self.root_node.map(|v| unsafe { 
            std::mem::transmute::<_, &Node>(v) }) {
            Some(v) => v,
            None => return true
        };
        let mut invalid = true;
        if self.show_default_camera {
            invalid = !self.is_object_default_camera(obj);
        }
        if self.show_root_node_hierarchy && invalid {
            invalid = RecursiveObjectIterator::<GfdAllocator, StandardNodeIterator>::from_node(root)
                .find(|v| std::ptr::addr_eq(obj, *v)).is_none()
        }
        let scene = GraphicsGlobal::get_gfd_graphics_global().get_current_scene().unwrap();
        if self.show_disjoint_meshes && invalid {
            invalid = scene.iter_attachment_type(ObjectId::Mesh).find(|v| {
                match v.get_object() {
                    Some(v) => std::ptr::addr_eq(v, obj),
                    None => false
                }}).is_none();
        }
        if self.show_disjoint_epls && invalid {
            invalid = scene.iter_attachment_type(ObjectId::EPL).find(|v| {
                match v.get_object() {
                    Some(v) => std::ptr::addr_eq(v, obj),
                    None => false
                }}).is_none();
        }
        invalid
    }
}
impl InspectorPanel for SceneGraphPanel {
    fn get_panel_name(&self) -> &'static str { "Scene Graph" }
    
    fn draw_contents(&mut self, ui: &mut Ui) {
        // debug
        let self_ptr = unsafe { &mut *(&raw mut *self) };
        // settings
        ui.checkbox("Show Node Hierarchy", &mut self.show_root_node_hierarchy);
        ui.same_line_with_spacing(0., 10.);
        ui.checkbox("Show Default Camera", &mut self.show_default_camera);
        ui.same_line_with_spacing(0., 10.);
        ui.checkbox("Show Root Meshes", &mut self.show_disjoint_meshes);
        ui.same_line_with_spacing(0., 10.);
        ui.checkbox("Show Root EPL", &mut self.show_disjoint_epls);
        // search nodes
        self.search.draw(ui);
        // get root node and camera if they exist
        let glb = GraphicsGlobal::get_gfd_graphics_global();
        if let Some(e) = glb.get_current_scene() {
            self.root_node = e.get_root_node().map(|v| unsafe { &*(&raw const *v as *const Object) });
        }
        if let Some(e) = glb.get_current_scene() {
            self.default_camera = e.get_current_camera().map(|v| unsafe { &*(&raw const *v as *const Object) });
        }
        // node table
        let hier = self.traverse_scene_graph();
        self.hierarchy.draw_table(ui, self_ptr, hier.as_slice());
        ui.separator();
        // node details
        if self.selected_attachment_history.is_empty() {
            ui.text("No object selected");
        } else {
            ui.text(format!("TODO: Selected attachment history (len: {})", self.selected_attachment_history.len()));
            let att = self.selected_attachment_history.last().unwrap();
            let att_addr = &raw const **att as usize;
            if ui.button("Go back")
            || self.is_object_invalid(*att) {
                self.history_last_valid_object();
            } 
            if let Some(att) = self.selected_attachment_history.last() {
                ui.same_line_with_spacing(0., 10.);
                ui.text(format!("Address: 0x{:x}", att_addr));
                match att.get_id() {
                    ObjectId::Mesh => unsafe { MeshProperties::new(att).draw(ui) },
                    ObjectId::Node => unsafe { NodeProperties::new(att).draw(ui) },
                    ObjectId::Camera => unsafe { CameraProperties::new(att).draw(ui) },
                    _ => ui.text(format!("Object type {:?} TODO", att.get_id()))
                }
            }
        }
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
                opengfd_inspector_components::table::default_flags(),
                opengfd_inspector_components::table::default_height(),
            ),
            scene_graph_visible: vec![],
            nodes_opened: HashSet::new(),

            selected_attachment_history: vec![],
            show_root_node_hierarchy: true,
            show_default_camera: true,
            show_disjoint_meshes: false,
            show_disjoint_epls: false,
            root_node: None,
            default_camera: None
        }
    }
}
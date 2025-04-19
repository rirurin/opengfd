use bitflags::bitflags;
use crate::{
    components::{
        searchbar::Searchbar,
        table::{ InspectorTable, TableDraw }
    },
    panels::{
        attachment::node::NodeProperties,
        common::InspectorPanel
    }
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
use riri_mod_tools_rt::logln;
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
                    crate::components::bullet::bullet_ex(ui, self.depth);
                } else {
                    let arrow_dir = match is_open {
                        true => Direction::Down,
                        false => Direction::Right
                    };
                    crate::components::bullet::arrow_ex(ui, arrow_dir, self.depth);
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
    show_disjoint_attachments: DisjointAttachmentFlags,
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
        if let Some(n) = self.root_node {
            TaskObjectEntry::add_hierarchy(&mut entries, n, &self.nodes_opened,0 );
        }
        if let Some(n) = self.default_camera {
            entries.push(TaskObjectEntry::new(n));
        }
        if self.show_disjoint_attachments.contains(DisjointAttachmentFlags::Meshes) {
            Self::get_disjoint_attachment(ObjectId::Mesh, &mut entries);
        }
        if self.show_disjoint_attachments.contains(DisjointAttachmentFlags::EPL) {
            Self::get_disjoint_attachment(ObjectId::EPL, &mut entries);
        }
        entries
    }
}
impl SceneGraphPanel {
    // check global scene graph
    fn history_last_valid_object(&mut self) {
        let root = match self.root_node.map(|v| unsafe { 
            std::mem::transmute::<_, &Node>(v) }) {
            Some(v) => v,
            None => return
        };
        
        while self.selected_attachment_history.len() > 0 {
            let att = self.selected_attachment_history.pop();
            if RecursiveObjectIterator::<GfdAllocator, StandardNodeIterator>::from_node(root)
                .find(|v| std::ptr::addr_eq(att.unwrap(), &**v)).is_some() {
                    break;
                }
        }
    }

    // fn get_root_node(&self) -> Option<&Node> {
    //     self.root_node.map(|v| unsafe { std::mem::transmute::<_, &Node>(v) })
    // }
}
impl InspectorPanel for SceneGraphPanel {
    fn get_panel_name(&self) -> &'static str { "Scene Graph" }
    
    fn draw_contents(&mut self, ui: &mut Ui) {
        // debug
        let self_ptr = unsafe { &mut *(&raw mut *self) };
        ui.text(&format!("want keyboard: {}, want mouse: {}", ui.io().want_capture_keyboard, ui.io().want_capture_mouse));
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
        let root = match self.root_node.map(|v| unsafe { 
            std::mem::transmute::<_, &Node>(v) }) {
            Some(v) => v,
            None => return
        };
        if self.selected_attachment_history.is_empty() {
            ui.text("No object selected");
        } else {
            // ui.text(format!("TODO: Selected attachment history (len: {})", self.selected_attachment_history.len()));
            // let att = self.selected_attachment_history.last().unwrap();
            // ui.text(format!("TODO: object 0x{:x}", &raw const **att as usize));
            for o in RecursiveObjectIterator::<GfdAllocator, StandardNodeIterator>::from_node(root) {
                // .find(|v| std::ptr::addr_eq(*att, *v)) {
                // let obj = unsafe { std::mem::transmute::<_, &Object>(o) };
                logln!(Verbose, "Object 0x{:x} (type {:?})", &raw const *o as usize, o.get_id());
            }
            self.selected_attachment_history.clear();
            /* 
            if RecursiveObjectIterator::<GfdAllocator, StandardNodeIterator>::from_node(root)
                .find(|v| std::ptr::addr_eq(*att, *v)).is_none() {
            // || ui.button("Go back") {
                self.history_last_valid_object();
            }
            */

            /*
            let v = self.selected_attachment_history.last().unwrap();
            ui.same_line_with_spacing(0., 10.);
            ui.text(format!("Address: 0x{:x}", &raw const **v as usize));
            match v.get_id() {
                ObjectId::Node => unsafe { NodeProperties::new(v).draw_contents(ui) },
                _ => ui.text(format!("Object type {:?} TODO", v.get_id()))
            }
            */
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
                crate::components::table::default_flags(),
                crate::components::table::default_height()
            ),
            scene_graph_visible: vec![],
            nodes_opened: HashSet::new(),

            selected_attachment_history: vec![],
            show_disjoint_attachments: DisjointAttachmentFlags::empty(),
            root_node: None,
            default_camera: None
        }
    }
}
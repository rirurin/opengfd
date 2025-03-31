use crate::{
    graphics::quake::Quake,
    object::{
        camera::Camera,
        light::{ Light, LightContainer },
        node::Node,
        object::Object
    },
    utility::{
        misc::{ RGBAFloat, LinkedList, LinkedListNode },
        mutex::Mutex,
        name::Name
    }
};
use glam::Mat4;

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaphorSceneParams {
    field0: RGBAFloat,
    field8: RGBAFloat,
    scene_ambient_sky: RGBAFloat,
    scene_sky_color: RGBAFloat,
    scene_env_color: RGBAFloat,
    scene_env_color_toon: RGBAFloat,
    env_field630: f32,
    env_field634: f32,
    water_deep_color_no_skybox_influence: RGBAFloat,
    water_scatter_color_no_skybox_influence: RGBAFloat,
    water_reflection_color_no_skybox_influence: RGBAFloat,
    water_foam_color_no_skybox_influence: RGBAFloat,
    water_deep_color_skybox_influenced: RGBAFloat,
    water_scatter_color_skybox_influenced: RGBAFloat,
    water_reflection_color_skybox_influenced: RGBAFloat,
    water_foam_color_skybox_influenced: RGBAFloat,
    scene_sky_fog_color: RGBAFloat,
    env_map_rotate_y: f32,
    ibl_map_rotate_y: f32,
    env_field6c8: f32,
    field104: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct SceneField {
    object: *mut Object,
    name: Name,
    file: Name,
    params: *mut u8,
    sync: *mut Object,
    prio_grp: u32,
    light: SceneFieldLight,
    link: LinkedListNode<SceneField>,
    link_all: LinkedListNode<SceneField>,
    dirty: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct SceneFieldLight {
    illuminate: [bool; 3],
    individiual: *mut Light,
    container: *mut LightContainer
}

#[repr(C)]
#[derive(Debug)]
pub struct SceneLightPlacement {
    point: LinkedList<Light>,
    spot: LinkedList<Light>,
    count: u32,
    mutex: Mutex
}

#[repr(C)]
#[derive(Debug)]
pub struct Scene {
    id: u32,
    flags: u32,
    #[cfg(feature = "v2-core")]
    env: MetaphorSceneParams,
    pub(crate) camera: *mut Camera,
    pub(crate) light: [*mut Light; 3],
    pub(crate) shadow: *mut Light,
    pub(crate) independence: *mut Light,
    pub(crate) hierarchy: *mut Node,
    terrain: *mut u8,
    ends: [LinkedList<SceneField>; 10],
    ends_all: LinkedList<SceneField>,
    dirty: u32,
    placement: SceneLightPlacement,
    operation_fn: Option<fn(*mut Scene, *mut u8, f32) -> ()>,
    operation_arg: *mut u8,
    frequency: f32,
    render_pre_cb_fn: Option<fn(*mut Scene, *mut u8) -> ()>,
    render_pre_cb_userdata: *mut u8,
    render_post_cb_fn: Option<fn(*mut Scene, *mut u8) -> ()>,
    render_post_cb_userdata: *mut u8,
    view_cb_fn: Option<fn(*mut Scene, *mut Mat4, *mut u8) -> ()>,
    view_cb_userdata: *mut u8,
    fn278: *mut u8,
    fn278_data: *mut u8,
    quake: *mut Quake,
    field290: [u8; 0x40]
}

impl Scene {
    pub fn get_root_node(&self) -> Option<&Node> {
        unsafe { self.hierarchy.as_ref() }
    }
    pub fn get_root_node_mut(&mut self) -> Option<&mut Node> {
        unsafe { self.hierarchy.as_mut() }
    }
}
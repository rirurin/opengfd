use allocator_api2::alloc::Allocator;
use crate::{
    graphics::quake::Quake,
    kernel::allocator::GfdAllocator,
    object::{
        camera::Camera,
        light::{ Light, LightContainer },
        node::Node,
        object::{ Object, ObjectId }
    },
    utility::{
        misc::{ RGBAFloat, LinkedList, LinkedListNode },
        mutex::Mutex,
        name::Name
    }
};
use glam::Mat4;
use std::ptr::NonNull;

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
pub struct SceneField<A = GfdAllocator> 
where A: Allocator + Clone
{
    object: *mut Object<A>,
    name: Name<A>,
    file: Name<A>,
    params: *mut u8,
    sync: *mut Object<A>,
    prio_grp: u32,
    light: SceneFieldLight,
    link: LinkedListNode<Self>,
    link_all: LinkedListNode<Self>,
    dirty: u32,
    _allocator: A
}

impl<A> SceneField<A>
where A: Allocator + Clone
{
    pub fn get_object(&self) -> Option<&Object<A>> {
        unsafe { self.object.as_ref() }
    }
    pub fn get_object_mut(&mut self) -> Option<&mut Object<A>> {
        unsafe { self.object.as_mut() }
    }
    pub fn get_name(&self) -> &Name<A> {
        &self.name
    }
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
pub struct Scene<A = GfdAllocator> 
where A: Allocator + Clone
{
    id: u32,
    flags: u32,
    #[cfg(feature = "v2-core")]
    env: MetaphorSceneParams,
    pub(crate) camera: *mut Camera<A>,
    pub(crate) light: [*mut Light; 3],
    pub(crate) shadow: *mut Light,
    pub(crate) independence: *mut Light,
    pub(crate) hierarchy: *mut Node<A>,
    terrain: *mut u8,
    ends: [LinkedList<SceneField<A>>; 10],
    ends_all: LinkedList<SceneField<A>>,
    dirty: u32,
    placement: SceneLightPlacement,
    operation_fn: Option<fn(*mut Self, *mut u8, f32) -> ()>,
    operation_arg: *mut u8,
    frequency: f32,
    render_pre_cb_fn: Option<fn(*mut Self, *mut u8) -> ()>,
    render_pre_cb_userdata: *mut u8,
    render_post_cb_fn: Option<fn(*mut Self, *mut u8) -> ()>,
    render_post_cb_userdata: *mut u8,
    view_cb_fn: Option<fn(*mut Self, *mut Mat4, *mut u8) -> ()>,
    view_cb_userdata: *mut u8,
    fn278: *mut u8,
    fn278_data: *mut u8,
    quake: Option<NonNull<Quake>>,
    field290: [u8; 0x40],
    _allocator: A
}

impl<A> Scene<A> 
where A: Allocator + Clone
{
    pub fn get_root_node(&self) -> Option<&Node<A>> {
        unsafe { self.hierarchy.as_ref() }
    }
    pub fn get_root_node_mut(&mut self) -> Option<&mut Node<A>> {
        unsafe { self.hierarchy.as_mut() }
    }
    pub fn get_quake(&self) -> Option<&Quake> {
        self.quake.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_quake_mut(&mut self) -> Option<&mut Quake> {
        self.quake.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn iter_attachment_type(&self, ty: ObjectId) -> SceneFieldIterator<'_, A> {
        let curr = unsafe { self.ends[ty as usize].head.as_ref() };
        let curr_rev = unsafe { self.ends[ty as usize].tail.as_ref() };
        SceneFieldIterator { curr, curr_rev }
    }
    pub fn iter_attachment_all(&self) -> SceneFieldIterator<'_, A> {
        let curr = unsafe { self.ends_all.head.as_ref() };
        let curr_rev = unsafe { self.ends_all.tail.as_ref() };
        SceneFieldIterator { curr, curr_rev }
    }
    pub fn get_current_camera(&self) -> Option<&Camera<A>> {
        unsafe { self.camera.as_ref() }
    }

    pub fn get_current_camera_mut(&mut self) -> Option<&mut Camera<A>> {
        unsafe { self.camera.as_mut() }
    }
}

pub struct SceneFieldIterator<'a, A> 
where A: Allocator + Clone
{
    curr: Option<&'a SceneField<A>>,
    curr_rev: Option<&'a SceneField<A>>
}
impl<'a, A> SceneFieldIterator<'a, A>
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const _ };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const _ };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, A> Iterator for SceneFieldIterator<'a, A>
where A: Allocator + Clone
{
    type Item = &'a SceneField<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = match self.collided() {
                false => unsafe { v.link.next.as_ref() },
                true => None
            };
            v
        })
    }
}

impl<'a, A> DoubleEndedIterator for SceneFieldIterator<'a, A>
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            self.curr_rev = match self.collided() {
                false => unsafe { v.link.prev.as_ref() },
                true => None
            };
            v
        })
    }
}
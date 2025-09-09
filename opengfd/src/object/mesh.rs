use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    anim::{
        anim_controller::AnimController,
        anim_effector::AnimEffector,
        animation::AnimInterpolator,
    },
    graphics::{ 
        cull::CullObject,
        material::Material,
        skin::SkinBoneObject
    },
    kernel::allocator::GfdAllocator,
    utility::{ 
        item_array::ItemArray, 
        misc::{ BoundingBox, BoundingSphere },
        property::Property,
        reference::{ GfdRcType, Reference },
    }
};
use glam::Vec3A;
use super::{
    camera::Camera,
    epl::EPL,
    geometry::Geometry,
    light::{ Light, LightContainer },
    morph::MorphController,
    node::Node, 
    object::Object
};
#[cfg(feature = "serialize")]
use std::{
    fmt::Debug,
    io::{ Read, Seek, Write },
};
use std::{
    error::Error,
    ptr::NonNull
};
use std::fmt::Formatter;
use std::io::SeekFrom;
use std::time::Instant;
use opengfd_proc::GfdRcAuto;
use crate::device::ngr::allocator::AllocatorHook;
use crate::graphics::texture::{Texture, TextureFormat, TextureSerializationContext};
use crate::kernel::version::GfdVersion;
use crate::object::object::{CastFromObject, ObjectId};
use crate::utility::misc::RGB;
use crate::utility::name::{Name, NameSerializationContext, NameSerializationNoHash};

#[cfg(feature = "serialize")]
use crate::utility::stream::{
    ChunkHeader,
    ChunkType,
    DeserializationHeap,
    DeserializationStrategy,
    GfdSerialize,
    GfdSerializationUserData,
    SerializationSingleAllocator,
    Stream,
    StreamError,
    StreamIODevice
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MeshFlags: u32 {
        const Flag0  = 1 << 0;
        const Flag1  = 1 << 1;
        const Flag2  = 1 << 2;
        const Flag3  = 1 << 3;
        const Flag4  = 1 << 4;
        const Flag5  = 1 << 5;
        const Flag6  = 1 << 6;
        const Flag7  = 1 << 7;
        const Flag8  = 1 << 8;
        const Flag9  = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const Flag15 = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const Flag18 = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}

const NECK_INTERPOLATOR_COUNT: usize = 2;
const LOCAL_OBB_COUNT: usize = 8;
const CULL_OBJECT_COUNT: usize = 3;

#[repr(C)]
// #[derive(Debug)]
#[derive(GfdRcAuto)]
pub struct Mesh<A = GfdAllocator> 
where A: Allocator + Clone
{
    _super: Object,
    flags: MeshFlags,
    hierarchy: Option<NonNull<Node<A>>>,
    node_array: *mut ItemArray<*mut Node<A>>,
    geometry_array: *mut ItemArray<*mut Geometry<A>>,
    material_array: *mut ItemArray<*mut Material<A>>,
    morph_array: *mut ItemArray<*mut MorphController>,
    camera_array: *mut ItemArray<*mut Camera<A>>,
    light_array: *mut ItemArray<*mut Light>,
    effect_array: *mut ItemArray<*mut EPL>,
    anim_interpolator: Option<NonNull<AnimInterpolator>>,
    anim_controller: Option<NonNull<AnimController>>,
    anim_effector: Option<NonNull<AnimEffector>>,
    neck_interpolator: [Option<NonNull<AnimInterpolator>>; NECK_INTERPOLATOR_COUNT],
    // For Bullet Physics, unused in Metaphor
    physics_sector: usize,
    bounding_box: BoundingBox,
    bounding_sphere: BoundingSphere,
    local_obb: *mut [Vec3A; LOCAL_OBB_COUNT],
    cull: [CullObject; CULL_OBJECT_COUNT],
    skin_bone_matrix_array: *mut ItemArray<usize>, // TODO
    skin_bone_object: *mut SkinBoneObject,
    light_container: *mut LightContainer,
    sync: Option<NonNull<MeshSync<A>>>,
    property: Option<NonNull<Property>>,
    // job data START
    #[cfg(feature = "v1-core")]
    field_140: *mut P5RMeshField140,
    #[cfg(feature = "v2-core")]
    gradation: Option<NonNull<Gradation<A>>>,
    // job data END
    reference: Reference,
    dirty: u32,
    _allocator: A
}

impl<A> Debug for Mesh<A>
where A: Allocator + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MESH TODO")
    }
}

impl<A> CastFromObject for Mesh<A>
where A: Allocator + Clone
{
    const OBJECT_ID: ObjectId = ObjectId::Mesh;
}

impl<A> Mesh<A>
where A: Allocator + Clone
{
    pub fn get_flags(&self) -> MeshFlags { self.flags }
    pub fn get_root_node(&self) -> Option<&Node<A>> {
        self.hierarchy.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_root_node_mut(&mut self) -> Option<&mut Node<A>> {
        self.hierarchy.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_node_list(&self) -> &[*mut Node<A>] {
        match unsafe { self.node_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_geometry_list(&self) -> &[*mut Geometry<A>] {
        match unsafe { self.geometry_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_material_list(&self) -> &[*mut Material<A>] {
        match unsafe { self.material_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_morph_list(&self) -> &[*mut MorphController] {
        match unsafe { self.morph_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_camera_list(&self) -> &[*mut Camera<A>] {
        match unsafe { self.camera_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_light_list(&self) -> &[*mut Light] {
        match unsafe { self.light_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_effect_list(&self) -> &[*mut EPL] {
        match unsafe { self.effect_array.as_ref() } {
            Some(a) => a.as_slice(),
            None => &[]
        }
    }
    pub fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
    pub fn get_bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }
    pub fn get_bounding_sphere(&self) -> &BoundingSphere {
        &self.bounding_sphere
    }
    pub fn get_bounding_sphere_mut(&mut self) -> &mut BoundingSphere {
        &mut self.bounding_sphere
    }
    pub fn get_anim_interpolator(&self) -> Option<&AnimInterpolator> {
        self.anim_interpolator.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_anim_interpolator_mut(&mut self) -> Option<&mut AnimInterpolator> {
        self.anim_interpolator.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_anim_controller(&self) -> Option<&AnimController> {
        self.anim_controller.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_anim_controller_mut(&mut self) -> Option<&mut AnimController> {
        self.anim_controller.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_anim_effector(&self) -> Option<&AnimEffector> {
        self.anim_effector.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_anim_effector_mut(&mut self) -> Option<&mut AnimEffector> {
        self.anim_effector.map(|mut v| unsafe { v.as_mut() })
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Mesh<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.reference = Reference::new();
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Mesh<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug {
        let allocator = param.get_heap_allocator().unwrap();
        let start = Instant::now();
        loop {
            let chunk = ChunkHeader::stream_read(stream, &mut ());
            if chunk.is_err() {
                break;
            }
            let chunk = chunk?.into_raw();
            // let chunk = ChunkHeader::stream_read(stream, &mut ())?.into_raw();
            println!("{:?}", chunk);
            match chunk.get_chunk_id() {
                /*
                ChunkType::Model => {

                },
                ChunkType::ExtraProperties => {

                },
                ChunkType::PhysicsDictionary => {

                },
                */
                ChunkType::MaterialDictionary => {
                    let mat_count = stream.read_u32()? as usize;
                    println!("{} materials", mat_count);
                    for i in 0..mat_count {
                        let _ = Material::<AObject>::stream_read(stream, param)?.into_raw();
                    }
                },
                ChunkType::TextureDictionary => {
                    // TEMPORARY
                    let tex_count = stream.read_u32()? as usize;
                    println!("{} textures", tex_count);
                    for i in 0..tex_count {
                        let _ = Texture::<AObject>::stream_read(stream, &mut TextureSerializationContext::new(allocator.clone(), allocator.clone()))?.into_raw();
                    }
                    // let textures = ItemArray::<u32, GfdAllocator>::with_capacity(stream.read_u32()? as usize, self._allocator.clone())?;
                    // println!("texture count: {}", textures.capacity());
                },
                /*
                ChunkType::AnimationPack => {

                },
                ChunkType::ChunkType000100FE => {

                },
                */
                _ => break
            };
        }
        println!("Time: {} ms", start.elapsed().as_micros() as f64 / 1000.);
        Ok(())
    }
}

#[repr(C)]
pub struct MeshSync<A>
where A: Allocator + Clone {
    pub attachment: Option<NonNull<MeshSyncAttachmentObject<A>>>,
    pub entry: Option<NonNull<MeshSyncEntryObject<A>>>,
    pub field2_0x10: *mut std::ffi::c_void, // registry?
    pub field3_0x18: *mut std::ffi::c_void, // neck?
    pub field4_0x20: *mut std::ffi::c_void,
    pub field5_0x28: *mut std::ffi::c_void,
}

#[repr(C)]
pub struct MeshSyncEntryObject<A>
where A: Allocator + Clone {
    pub entry: Option<NonNull<Object<A>>>,
    pub mask: u32,
    pub callback: *mut std::ffi::c_void,
    pub prev: Option<NonNull<Self>>,
    pub next: Option<NonNull<Self>>,
    _allocator: A
}

#[repr(C)]
pub struct MeshSyncAttachmentObject<A>
where A: Allocator + Clone {
    pub object: Option<NonNull<Object<A>>>,
    pub id: u32,
    pub field3_0x10: *mut std::ffi::c_void,
    pub update: Option<fn(*mut u8, &mut Object<A>, f32)>, // gfdMeshAttachmentObjectUpdateFunc
    pub render: Option<fn(u32, *mut u8)>, // gfdMeshAttachmentObjectRenderFunc
    pub prev: Option<NonNull<Self>>,
    pub next: Option<NonNull<Self>>,
    _allocator: A
}

#[repr(C)]
pub struct MeshSyncNeckCallbackObject {
    func: Option<fn(*mut u8, *mut u8)>,
    userdata: *mut std::ffi::c_void
}

#[repr(C)]
pub struct MeshSyncRegistryCallbackObject {
    func: Option<fn(*mut u8, *mut u8)>,
    userdata: *mut std::ffi::c_void,
    mask: u32,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
}

#[repr(C)]
pub struct MeshSyncCallback<A>
where A: Allocator + Clone {
    owner: Option<NonNull<Object<A>>>,
    sync: [Option<NonNull<Object<A>>>; 2],
    func: Option<fn(*mut u8, *mut u8)>,
    dirty: u32,
    mask: u32,
    userdata: [*mut std::ffi::c_void; 2],
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _allocator: A
}

#[repr(C)]
pub struct Gradation<A>
where A: Allocator + Clone {
    field0: usize,
    pub root_node: Option<NonNull<Node<A>>>,
    pub field9_0x10: *mut std::ffi::c_void,
    pub hip_node: Option<NonNull<Node<A>>>,
    pub right_heel_node: Option<NonNull<Node<A>>>,
    pub left_heel_node: Option<NonNull<Node<A>>>,
    pub color: RGB,
    pub scale: f32,
    pub fade: f32,
    pub alpha: f32,
    pub field18_0x40: i32,
    field44: [u8; 60],
    _allocator: A
}

#[repr(C)]
pub struct P5RMeshField140 {
    data: [u8; 0x40]
}

pub mod ffi {

}

#[cfg(test)]
pub mod test {

}

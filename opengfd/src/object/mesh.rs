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
        reference::Reference
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
use std::ptr::NonNull;
use riri_mod_tools_proc::ensure_layout;

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
#[derive(Debug)]
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
    sync: *mut MeshSync,
    property: Option<NonNull<Property>>,
    // job data START
    #[cfg(feature = "v1-core")]
    field_140: *mut P5RMeshField140,
    #[cfg(feature = "v2-core")]
    gradation: Option<NonNull<Gradation>>,
    // job data END
    reference: Reference,
    dirty: u32,
    _allocator: A
}

impl<A> Mesh<A>
where A: Allocator + Clone
{
    pub fn get_flags(&self) -> MeshFlags { self.flags }
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
}

#[ensure_layout(size = 48usize)]
pub struct MeshSync {
    #[field_offset(0usize)]
    pub attachment: *mut MeshSyncAttachmentObject,
    #[field_offset(8usize)]
    pub entry: *mut MeshSyncEntryObject,
    #[field_offset(16usize)]
    pub field2_0x10: *mut ::std::os::raw::c_void,
    #[field_offset(24usize)]
    pub field3_0x18: *mut ::std::os::raw::c_void,
    #[field_offset(32usize)]
    pub field4_0x20: *mut ::std::os::raw::c_void,
    #[field_offset(40usize)]
    pub field5_0x28: *mut ::std::os::raw::c_void,
}

#[ensure_layout(size = 40usize)]
pub struct MeshSyncEntryObject {
    #[field_offset(0usize)]
    pub entry: *mut Object,
    #[field_offset(8usize)]
    pub mask: u32,
    #[field_offset(16usize)]
    pub callback: *mut ::std::os::raw::c_void,
    #[field_offset(24usize)]
    pub prev: *mut MeshSyncEntryObject,
    #[field_offset(32usize)]
    pub next: *mut MeshSyncEntryObject,
}

#[ensure_layout(size = 56usize)]
pub struct MeshSyncAttachmentObject {
    #[field_offset(0usize)]
    pub object: *mut Object,
    #[field_offset(8usize)]
    pub id: u32,
    #[field_offset(16usize)]
    pub field3_0x10: *mut ::std::os::raw::c_void,
    #[field_offset(24usize)]
    pub update: *mut ::std::os::raw::c_void,
    #[field_offset(32usize)]
    pub render: *mut ::std::os::raw::c_void,
    #[field_offset(40usize)]
    pub prev: *mut MeshSyncAttachmentObject,
    #[field_offset(48usize)]
    pub next: *mut MeshSyncAttachmentObject,
}

#[ensure_layout(size = 128usize)]
pub struct Gradation {
    #[field_offset(8usize)]
    pub root_node: *mut Node,
    #[field_offset(16usize)]
    pub field9_0x10: *mut ::std::os::raw::c_void,
    #[field_offset(24usize)]
    pub hip_node: *mut Node,
    #[field_offset(32usize)]
    pub right_heel_node: *mut Node,
    #[field_offset(40usize)]
    pub left_heel_node: *mut Node,
    #[field_offset(48usize)]
    pub color: [u8; 3usize],
    #[field_offset(52usize)]
    pub scale: f32,
    #[field_offset(56usize)]
    pub fade: f32,
    #[field_offset(60usize)]
    pub alpha: f32,
    #[field_offset(64usize)]
    pub field18_0x40: i32,
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

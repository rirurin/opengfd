use bitflags::bitflags;
use crate::{
    graphics::{ 
        cull::CullObject,
        material::Material,
        skin::SkinBoneObject
    },
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
use riri_mod_tools_proc::ensure_layout;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MeshFlags: u32 {
        const Blank = 1 << 0;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Mesh {
    _super: Object,
    flags: MeshFlags,
    hierarchy: *mut Node,
    node_array: *mut ItemArray<*mut Node>,
    geoemtry_array: *mut ItemArray<*mut Geometry>,
    material_array: *mut ItemArray<*mut Material>,
    morph_array: *mut ItemArray<*mut MorphController>,
    camera_array: *mut ItemArray<*mut Camera>,
    light_array: *mut ItemArray<*mut Light>,
    effect_array: *mut ItemArray<*mut EPL>,
    anim_interpolator: usize,
    anim_controller: usize,
    anim_effector: usize,
    neck_interpolator: [usize; 2],
    physics_sector: usize,
    bounding_box: BoundingBox,
    bounding_sphere: BoundingSphere,
    local_obb: *mut [Vec3A; 8],
    cull: [CullObject; 3],
    skin_bone_matrix_array: *mut ItemArray<usize>, // TODO
    skin_bone_object: *mut SkinBoneObject,
    light_container: *mut LightContainer,
    sync: *mut MeshSync,
    property: *mut Property,
    // job data START
    #[cfg(feature = "v1-core")]
    field_140: *mut P5RMeshField140,
    #[cfg(feature = "v2-core")]
    gradation: *mut Gradation,
    // job data END
    reference: Reference,
    dirty: u32
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

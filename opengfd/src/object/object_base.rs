use std::{
    marker::PhantomPinned,
    ptr::NonNull
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// All valid object types in GFD.
/// (Original enum: gfdObjectID)
pub enum ObjectId {
    // Invalid = 0,
    // Scene,
    Mesh = 2,
    Node,
    Geometry,
    Camera,
    Light,
    EPL,
    EPLLeaf,
    Morph
}

pub trait ObjectFunctionTable {
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectErrorID {
    Unknown = 0
}


/// Represents the basic object type shared by all object representations in GFD. In an object
/// oriented context, Object would be a base class, while Scene, Mesh etc. are all derived classes.
/// It's not possible to create a new object directly, you should instead create a new instance
/// using either new() or through deserialization. Objects are banned from semantic moves since
/// they are address sensitive type (prev/next fields create an intrusive linked list, parent
/// points to another node in hierarchy and if the type is node, it may have children itself).
/// (Original struct: gfdObject)
#[repr(C)]
#[derive(Debug)]
pub struct Object {
    id: ObjectId,
    parent: Option<NonNull<super::node::Node>>,
    prev: Option<NonNull<Object>>,
    next: Option<NonNull<Object>>,
    _pinned: PhantomPinned
}

impl TryFrom<*const Object> for &super::mesh::Mesh {
    type Error = ObjectErrorID;
    fn try_from(_value: *const Object) -> Result<Self, Self::Error> {

        Err(ObjectErrorID::Unknown)
    }
}

impl TryFrom<*mut Object> for &mut super::mesh::Mesh {
    type Error = ObjectErrorID;
    fn try_from(_value: *mut Object) -> Result<Self, Self::Error> {

        Err(ObjectErrorID::Unknown)
    }
}

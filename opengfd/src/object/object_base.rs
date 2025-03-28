use allocator_api2::alloc::Allocator;
use crate::{
    kernel::allocator::GfdAllocator,
    object::node::Node
};
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
pub struct Object<A = GfdAllocator> 
where A: Allocator + Clone 
{
    id: ObjectId,
    parent: Option<NonNull<Node<A>>>,
    prev: Option<NonNull<Object<A>>>,
    next: Option<NonNull<Object<A>>>,
    _pinned: PhantomPinned,
    _allocator: A
}

impl<A> Object<A> 
where A: Allocator + Clone
{

    // Original function: gfdObjectInitialize
    pub(crate) fn new(id: ObjectId, alloc: A) -> Self {
        Self {
            id,
            parent: None,
            prev: None,
            next: None,
            _pinned: PhantomPinned,
            _allocator: alloc
        }
    }

    pub fn get_parent(&self) -> Option<&Node<A>> {
        self.parent.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_parent_mut(&mut self) -> Option<&mut Node<A>> {
        self.parent.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_prev(&self) -> Option<&Self> {
        self.prev.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_next(&self) -> Option<&Self> {
        self.next.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_prev_mut(&mut self) -> Option<&mut Self> {
        self.prev.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_next_mut(&mut self) -> Option<&mut Self> {
        self.next.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_id(&self) -> ObjectId {
        self.id
    }
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

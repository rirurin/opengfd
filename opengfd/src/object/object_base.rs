#![allow(dead_code)]
use allocator_api2::alloc::Allocator;
use crate::{
    kernel::allocator::GfdAllocator,
    object::node::Node
};
use std::{
    fmt::Debug,
    hash::{ Hash, Hasher },
    marker::PhantomPinned,
    ptr::NonNull,
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
#[derive(Eq)]
// #[derive(Debug)]
pub struct Object<A = GfdAllocator> 
where A: Allocator + Clone 
{
    id: ObjectId,
    parent: Option<NonNull<Node<A>>>,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
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
    pub fn get_id(&self) -> ObjectId {
        self.id
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
    pub fn get_prev_ptr(&self) -> *mut Self {
        match self.prev {
            Some(p) => p.as_ptr(),
            None => std::ptr::null_mut()
        }
    }
    pub fn get_next_ptr(&self) -> *mut Self {
        match self.next {
            Some(p) => p.as_ptr(),
            None => std::ptr::null_mut()
        }
    }

    /// Original function: gfdObjectSetNode
    pub fn set_parent(&mut self, parent: Option<&mut Node<A>>) {
        if let Some(p) = &parent {
            /* 
            match self.next {
                Some(v) => {
                    v.prev = self.prev;
                },
                None => {

                }
            }
            */
            let _ = p.get_average_scale();
        }
        self.parent = parent.map(|v| unsafe { NonNull::new_unchecked(&raw mut *v) });
    }
}

impl<A> PartialEq for Object<A> 
where A: Allocator + Clone
{
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self, other)
    }
}
impl<A> Hash for Object<A> 
where A: Allocator + Clone
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr= &raw const *self as usize;
        state.write_usize(addr);
    }
}

impl<A> Debug for Object<A>
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent_fmt = match self.parent {
            Some(v) => format!("{}", unsafe { v.as_ref() }),
            None => "None".to_owned()
        };
        write!(f, "{:?} @ 0x{:x} {{ parent: {}, prev: 0x{:x}, next: 0x{:x} }}", 
        self.id, &raw const *self as usize, parent_fmt, 
        self.get_prev_ptr() as usize, self.get_next_ptr() as usize)
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

// gfdObjectFunctionTable
pub trait ObjectTable {
    /// Original function: gfdObjectGetIdName
    const ID: &'static str;
    const NUM: usize;
    /// Original function: gfdObjectAddRef
    fn add_ref(&mut self) -> u32;
    /// Original function: gfdObjectApplyKey
    fn apply_key(&mut self, key: u8);
    /// Original function: gfdObjectSetVisible
    fn set_visible(&mut self, visibility: bool);
    /// Original function: gfdObjectSetCulled
    fn set_culled(&mut self, cull: bool);
    fn set_dirty(&mut self, dirty: bool);
    fn clear_dirty(&mut self);
    /// Original function: gfdObjectCheckDirtied
    fn check_dirty(&self) -> bool;
}
#![allow(dead_code)]
use allocator_api2::alloc::Allocator;
use glam::{ Vec3A, Quat, Mat4 };
use crate::{
    kernel::allocator::GfdAllocator,
    object::object::{ Object, ObjectId },
    utility::{ 
        name::Name, 
        property::{ Property, PropertyChunk },
    }
};
use std::{
    fmt::{ Display, Debug },
    ops::{ Deref, DerefMut },
    ptr::NonNull
};
// use riri_mod_tools_rt::logln;

#[repr(C)]
// #[derive(Debug)]
pub struct Node<A = GfdAllocator> 
where A: Allocator + Clone 
{
    _super: Object<A>,
    world_tm: Mat4,
    local_tm: Mat4,
    transform: NodeTransform,
    link: NodeLink<A>,
    name: Name,
    visibility: f32,
    object_head: Option<NonNull<Object<A>>>,
    object_tail: Option<NonNull<Object<A>>>,
    property: Option<NonNull<Property<A>>>,
    _allocator: A
}

impl<A> Node<A>
where A: Allocator + Clone
{
    //pub fn iter_object()
    // pub fn iter_children()
}

// ==========================
// Standard Node Iterator
// ==========================

pub struct StandardNodeIterator;
impl ObjectIterationSettings for StandardNodeIterator {
    fn allow_child_iteration() -> bool {
        true
    }
}
pub struct DirectNodeIterator;
impl ObjectIterationSettings for DirectNodeIterator {
    fn allow_child_iteration() -> bool {
        false
    }
}

pub trait ObjectIterationSettings { 
    fn allow_child_iteration() -> bool;
}

pub struct NodeIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    parents: Vec<&'a Node<A>>,
    curr: Option<&'a Node<A>>,
    _alloc_marker: std::marker::PhantomData<A>,
    _settings_marker: std::marker::PhantomData<S>
}

impl<'a, A, S> Iterator for NodeIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    type Item = &'a Node<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            // depth first search, so check child first
            if S::allow_child_iteration() && v.link.child.is_some() {
                if let Some(cn) = v.link.next {
                    self.parents.push(unsafe { cn.as_ref() });
                }
                self.curr = Some(unsafe { v.link.child.unwrap().as_ref() });
            } else {
                // then check siblings
                self.curr = if let Some(s) = v.link.next {
                    Some(unsafe { s.as_ref() })
                // if at the end of the chain, go back to the parent
                } else if !self.parents.is_empty() {
                    Some(self.parents.pop().unwrap())
                } else {
                    None
                };
            }
            v
        })
    }
}

impl<'a, A, S> NodeIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    pub fn from_node(node: &'a Node<A>) -> Self {
        Self {
            parents: vec![],
            curr: Some(node),
            _alloc_marker: std::marker::PhantomData::<A>,
            _settings_marker: std::marker::PhantomData::<S>
        }
    }
}

pub struct NodeIteratorMut<'a, A>
where A: Allocator + Clone
{
    parents: Vec<&'a mut Node<A>>,
    curr: Option<&'a mut Node<A>>,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> Iterator for NodeIteratorMut<'a, A>
where A: Allocator + Clone
{
    type Item = &'a mut Node<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            // depth first search, so check child first
            self.curr = if let Some(mut c) = v.link.child {
                if let Some(mut cn) = v.link.next {
                    self.parents.push(unsafe { cn.as_mut() });
                }
                Some(unsafe { c.as_mut() })
            // then check siblings
            } else if let Some(mut s) = v.link.next {
                Some(unsafe { s.as_mut() })
            // if at the end of the chain, go back to the parent
            } else if !self.parents.is_empty() {
                Some(self.parents.pop().unwrap())
            } else {
                None
            };
            v
        })
    }
}

impl<'a, A> NodeIteratorMut<'a, A>
where A: Allocator + Clone
{
    pub fn from_node(node: &'a mut Node<A>) -> Self {
        Self {
            parents: vec![],
            curr: Some(node),
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }
}

// ==========================
// Depth Limited Node Iterator
// ==========================

pub struct NodeDepthResult<'a, A> 
where A: Allocator + Clone 
{
    value: &'a Node<A>,
    depth: usize,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> NodeDepthResult<'a, A>
where A: Allocator + Clone
{
    pub fn new_root(value: &'a Node<A>) -> Self {
        Self::new(value, 0)
    }

    pub fn new(value: &'a Node<A>, depth: usize) -> Self {
        Self {
            value, depth, 
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }

    pub fn get_depth(&self) -> usize { self.depth }
}

impl<'a, A> Deref for NodeDepthResult<'a, A> 
where A: Allocator + Clone
{
    type Target = Node<A>;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

pub struct NodeIteratorDepthLimited<'a, A>
where A: Allocator + Clone
{
    parents: Vec<NodeDepthResult<'a, A>>,
    curr: Option<NodeDepthResult<'a, A>>,
    limit: usize,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> Iterator for NodeIteratorDepthLimited<'a, A>
where A: Allocator + Clone
{
    type Item = NodeDepthResult<'a, A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            // depth first search, so check child first
            if v.link.child.is_some() && v.get_depth() < self.limit {
                if let Some(cn) = v.link.next {
                    self.parents.push(unsafe { NodeDepthResult::new(cn.as_ref(), v.get_depth()) });
                }
                let c = unsafe { v.link.child.unwrap().as_ref() };
                self.curr = Some(NodeDepthResult::new(c, v.get_depth() + 1))
            } else {
                self.curr = if let Some(s) = v.link.next {
                    Some(unsafe { NodeDepthResult::new(s.as_ref(), v.get_depth()) })
                // if at the end of the chain, go back to the parent
                } else if !self.parents.is_empty() {
                    Some(self.parents.pop().unwrap())
                } else {
                    None
                };
            }
            v
        })
    }
}

impl<'a, A> NodeIteratorDepthLimited<'a, A>
where A: Allocator + Clone
{
    pub fn from_node(node: &'a Node<A>) -> Self {
        Self::from_node_limited(node, usize::MAX)
    }
    pub fn from_node_limited(node: &'a Node<A>, limit: usize) -> Self {
        Self {
            parents: vec![],
            curr: Some(NodeDepthResult::new_root(node)),
            limit,
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }
}

pub struct NodeDepthResultMut<'a, A> 
where A: Allocator + Clone 
{
    value: &'a mut Node<A>,
    depth: usize,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> NodeDepthResultMut<'a, A>
where A: Allocator + Clone
{
    pub fn new_root(value: &'a mut Node<A>) -> Self {
        Self::new(value, 0)
    }

    pub fn new(value: &'a mut Node<A>, depth: usize) -> Self {
        Self {
            value, depth, 
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }

    pub fn get_depth(&self) -> usize { self.depth }
}

impl<'a, A> Deref for NodeDepthResultMut<'a, A> 
where A: Allocator + Clone
{
    type Target = Node<A>;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, A> DerefMut for NodeDepthResultMut<'a, A> 
where A: Allocator + Clone
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

pub struct NodeIteratorDepthLimitedMut<'a, A>
where A: Allocator + Clone
{
    parents: Vec<NodeDepthResultMut<'a, A>>,
    curr: Option<NodeDepthResultMut<'a, A>>,
    limit: usize,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> Iterator for NodeIteratorDepthLimitedMut<'a, A>
where A: Allocator + Clone
{
    type Item = NodeDepthResultMut<'a, A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            // depth first search, so check child first
            if v.link.child.is_some() && v.get_depth() < self.limit {
                if let Some(mut cn) = v.link.next {
                    self.parents.push(unsafe { NodeDepthResultMut::new(cn.as_mut(), v.get_depth()) });
                }
                let c = unsafe { v.link.child.unwrap().as_mut() };
                self.curr = Some(NodeDepthResultMut::new(c, v.get_depth() + 1))
            } else {
                self.curr = if let Some(mut s) = v.link.next {
                    Some(unsafe { NodeDepthResultMut::new(s.as_mut(), v.get_depth()) })
                // if at the end of the chain, go back to the parent
                } else if !self.parents.is_empty() {
                    Some(self.parents.pop().unwrap())
                } else {
                    None
                };
            }
            v
        })
    }
}

impl<'a, A> NodeIteratorDepthLimitedMut<'a, A>
where A: Allocator + Clone
{
    pub fn from_node(node: &'a mut Node<A>) -> Self {
        Self::from_node_limited(node, usize::MAX)
    }
    pub fn from_node_limited(node: &'a mut Node<A>, limit: usize) -> Self {
        Self {
            parents: vec![],
            curr: Some(NodeDepthResultMut::new_root(node)),
            limit,
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }
}

// ==========================
// Standard Object Iterator
// ==========================

/// Represents a basic object iterator for a node. This will only iterate through object attachments from the target node.
/// This does not include child nodes.
pub struct ObjectIterator<'a, A>
where A: Allocator + Clone
{
    curr: Option<&'a Object<A>>,
    curr_rev: Option<&'a Object<A>>,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> ObjectIterator<'a, A>
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const Object<A> };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const Object<A> };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, A> Iterator for ObjectIterator<'a, A>
where A: Allocator + Clone
{
    type Item = &'a Object<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = match self.collided() {
                false => v.get_next(),
                true => None
            };
            v  
        })
    }
}

impl<'a, A> DoubleEndedIterator for ObjectIterator<'a, A>
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            self.curr_rev = match self.collided() {
                false => v.get_prev(),
                true => None
            };
            v  
        })
    }
}

pub struct ObjectIteratorMut<'a, A>
where A: Allocator + Clone
{
    curr: Option<&'a mut Object<A>>,
    curr_rev: Option<&'a mut Object<A>>,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, A> ObjectIteratorMut<'a, A>
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const Object<A> };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const Object<A> };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, A> Iterator for ObjectIteratorMut<'a, A>
where A: Allocator + Clone
{
    type Item = &'a mut Object<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            let value = unsafe { &mut *(&raw const *v as *mut Object<A>) };
            self.curr = match self.collided() {
                false => v.get_next_mut(),
                true => None
            };
            value
        })
    }
}

impl<'a, A> DoubleEndedIterator for ObjectIteratorMut<'a, A>
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            let value = unsafe { &mut *(&raw const *v as *mut Object<A>) };
            self.curr_rev = match self.collided() {
                false => v.get_prev_mut(),
                true => None
            };
            value  
        })
    }
}

impl<A> Node<A>
where A: Allocator + Clone
{
    pub fn iter_object(&self) -> ObjectIterator<'_, A> {
        ObjectIterator {
            curr: self.object_head.map(|v| unsafe { v.as_ref() }),
            curr_rev: self.object_tail.map(|v| unsafe { v.as_ref() }),
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }
    pub fn iter_object_mut(&mut self) -> ObjectIteratorMut<'_, A> {
        ObjectIteratorMut {
            curr: self.object_head.map(|mut v| unsafe { v.as_mut() }),
            curr_rev: self.object_tail.map(|mut v| unsafe { v.as_mut() }),
            _alloc_marker: std::marker::PhantomData::<A>
        }
    }
}

// ==========================
// Recursive Object Iterator
// ==========================

/// Represents an iterator over a node that includes child nodes and object attachments. This will travel the entire
/// hierarchy from the starting node in order of [parent node] -> [parent node attachments] 
/// -> [first child node] -> [first child node attachments] -> ....
/// You can set whenever to recurse into child nodes with the settings trait S.
pub struct RecursiveObjectIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    parents: Vec<&'a Node<A>>,
    curr: Option<&'a Object<A>>,
    _alloc_marker: std::marker::PhantomData<A>,
    _settings: std::marker::PhantomData<S>
}

impl<'a, A, S> Iterator for RecursiveObjectIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    type Item = &'a Object<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = match v.get_id() {
                // node - check first object attachment, then fallback to checking next node
                ObjectId::Node => {
                    let node = unsafe { std::mem::transmute::<_, &Node<A>>(v) };
                    node.get_first_object().or_else(|| self.find_next_node().map(
                        |v| unsafe { std::mem::transmute::<_, &Object<A>>(v) }))
                },
                // other attachment type - check if next in attachment chain, else check next node
                _ => v.get_next().or_else(|| self.find_next_node().map(
                    |v| unsafe { std::mem::transmute::<_, &Object<A>>(v) }))
            };
            /* 
            self.curr = v.get_next().or_else(|| self.find_next_node().map(
                |v| v.get_first_object().unwrap()));
            */
            v
        })
    }
}

impl<'a, A, S> RecursiveObjectIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    pub fn find_next_node(&mut self) -> Option<&'a Node<A>> {
        let result = match self.parents.pop() {
            Some(v) => {
                if let Some(c) = v.link.child {
                    if let Some(mut cn) = v.link.next {
                        self.parents.push(unsafe { cn.as_mut() });
                    }
                    Some(unsafe { c.as_ref() })
                // then check siblings
                } else if let Some(s) = v.link.next {
                    Some(unsafe { s.as_ref() })
                // if at the end of the chain, go back to the parent
                } else if !self.parents.is_empty() {
                    Some(self.parents.pop().unwrap())
                } else {
                    None
                }
            },
            None => None
        };
        if let Some(n) = result {
            self.parents.push(n);
        }
        result
    }
}

impl<'a, A, S> RecursiveObjectIterator<'a, A, S>
where A: Allocator + Clone,
      S: ObjectIterationSettings
{
    pub fn from_node(node: &'a Node<A>) -> Self {
        let first = unsafe { std::mem::transmute::<_, &Object<A>>(node) };
        Self {
            parents: vec![node],
            curr: Some(first),
            _alloc_marker: std::marker::PhantomData::<A>,
            _settings: std::marker::PhantomData::<S>
        }
    }
}

impl<A> Node<A>
where A: Allocator + Clone
{

    /// Original function: gfdNodeCollectHelper
    pub fn collect_by_helper_id(&self, id: i32) -> Vec<&Self> {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).filter(|n| n.has_helper_id(id)).collect()
    }

    pub fn collect_by_name<F>(&self, name: &str) -> Vec<&Self> {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).filter(|v| v.has_name(name)).collect()
    }

    pub fn collect_by_predicate<C>(&self, cond: C) -> Vec<&Self>
    where C: Fn(&Self) -> bool {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).filter(|v| cond(v)).collect()
    }

    // Count the number of child nodes *including* the current node
    /// Original function: gfdNodeGetCount
    pub fn get_count(&self) -> usize {
        1 + self.get_child_count()
    }
    /// Get direct child nodes of the current node (one layer deep)
    pub fn get_direct_children(&self) -> Vec<&Self> {
        match self.link.child {
            Some(v) => {
                let first = unsafe { v.as_ref() };
                NodeIterator::<A, DirectNodeIterator>::from_node(first).collect()
            },
            None => vec![]
        }
    }
    /// Get the number of direct child nodes for the current node
    pub fn get_direct_child_count(&self) -> usize {
        match self.link.child {
            Some(v) => {
                let first = unsafe { v.as_ref() };
                NodeIterator::<A, DirectNodeIterator>::from_node(first).count()
            },
            None => 0
        }
    }
    /// Get the number of children nodes for the current node. This method is recursive
    /// and will return a Vec<&Node> in depth first order
    /// Original function: gfdNodeCollect
    pub fn get_children(&self) -> Vec<&Self> {
        match self.link.child {
            Some(v) => {
                let first = unsafe { v.as_ref() };
                NodeIterator::<A, StandardNodeIterator>::from_node(first).collect()
            },
            None => vec![]
        }
    }

    pub fn get_child_count(&self) -> usize {
        match self.link.child {
            Some(v) => {
                let first = unsafe { v.as_ref() };
                NodeIterator::<A, StandardNodeIterator>::from_node(first).count()
            },
            None => 0
        }
    }

    pub fn get_children_limited_depth(&self, limit: usize) -> Vec<NodeDepthResult<A>> {
        match self.link.child {
            Some(v) => {
                let first = unsafe { v.as_ref() };
                NodeIteratorDepthLimited::from_node_limited(first, limit).collect()
            },
            None => vec![]
        }
    }

    /// Check if the current node has a gfdHelperID with a matching ID
    pub fn has_helper_id(&self, id: i32) -> bool {
        if let Some(p) = self.get_property() {
            if let Some(h) = p.find("gfdHelperID") {
                if let Ok(n) = h.get_integer_value() {
                    n == id
                } else { false }
            } else { false }
        } else { false }
    }

    pub fn has_name(&self, name: &str) -> bool {
        match self.get_name() {
            Some(n) => n == name,
            None => false
        }
    }

    /// Original function: gfdNodeFindHierarchyByHelperID 
    /// and gfdNodeFindHierarchyByHelperIDRecursive
    pub fn find_by_helper_id(&self, id: i32) -> Option<&Self> {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).find(|n| n.has_helper_id(id))
    }

    /// Original function: gfdNodeFindHierarchyByName
    pub fn find_by_name(&self, name: &str) -> Option<&Self> {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).find(|n| n.has_name(name))
    }

    pub fn find_by_predicate<F>(&self, cb: F) -> Option<&Self> 
    where F: Fn(&Self) -> bool {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).find(|n| cb(*n))
    }


    pub fn for_each_by_helper_id<F>(&self, id: i32, cb: F) 
    where F: Fn(&Self) {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).filter(|v| v.has_helper_id(id)).for_each(|n| cb(n))
    }

    pub fn for_each_by_name<F>(&self, name: &str, cb: F) 
    where F: Fn(&Self) {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).filter(|v| v.has_name(name)).for_each(|n| cb(n))
    }

    pub fn for_each_by_predicate<C, F>(&self, cond: C, cb: F) 
    where C: Fn(&Self) -> bool,
          F: Fn(&Self) {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).filter(|v| cond(v)).for_each(|n| cb(n))
    }

    pub fn get_average_scale(&self) -> f32 {
        let scl = self.get_scale();
        (scl.x + scl.y + scl.z) / 3.
    }

    // Get a list of all nodes recursively, including child and sibling nodes
    pub fn get_nodes(&self) -> Vec<&Self> {
        NodeIterator::<A, StandardNodeIterator>::from_node(self).collect()
    }

    /// Get the local transformation matrix for the current node
    /// Original function: gfdNodeGetLocalTransform
    pub fn get_local_transform(&self) -> Mat4 {
        self.local_tm
    }
    /// Get the name of the current node
    pub fn get_name(&self) -> Option<&str> {
        self.name.get_string()
    }
    pub fn get_name_platform(&self) -> &Name {
        &self.name
    }

    pub fn get_first_object(&self) -> Option<&Object<A>> {
        self.object_head.map(|v| unsafe { v.as_ref() })
    }

    pub fn get_last_object(&self) -> Option<&Object<A>> {
        self.object_tail.map(|v| unsafe { v.as_ref() })
    }

    /// Get a immutable reference to the parent of the current node within the hierarchy
    /// Original function: gfdNodeGetParent
    pub fn get_parent(&self) -> Option<&Self> {
        self._super.get_parent()
    }
    /// Get a mutable reference to the parent of the current node within the hierarchy
    pub fn get_parent_mut(&mut self) -> Option<&mut Self> {
        self._super.get_parent_mut()
    }

    pub fn get_property(&self) -> Option<&Property<A>> {
        self.property.map(|v| unsafe { v.as_ref() })
    }

    pub fn get_property_entry(&self, name: &str) -> Option<&PropertyChunk<A>> {
        if let Some(p) = self.get_property() {
            p.find(name)
        } else { None }
    }

    pub fn get_property_mut(&mut self) -> Option<&mut Property<A>> {
        self.property.map(|mut v| unsafe { v.as_mut() })
    }

    pub fn get_property_entry_mut(&mut self, name: &str) -> Option<&mut PropertyChunk<A>> {
        if let Some(p) = self.get_property_mut() {
            p.find_mut(name)
        } else { None }
    }

    /// Get a immutable reference to the root of the node hierarchy that this node is within
    /// Original function: gfdNodeGetRoot
    pub fn get_root(&self) -> Option<&Self> {
        self.link.root.map(|v| unsafe { v.as_ref() })
    }
    /// Get a mutable reference to the root of the node hierarchy that this node is within
    pub fn get_root_mut(&mut self) -> Option<&mut Self> {
        self.link.root.map(|mut v| unsafe { v.as_mut() })
    }
    /// Get the position of the current node in world space
    /// Original function: gfdNodeGetRotate
    pub fn get_translate(&self) -> Vec3A {
        self.transform.translate
    }
    /// Get the rotation (as a quarternion) of the current node
    /// Original function: gfdNodeGetRotate
    pub fn get_rotate(&self) -> Quat {
        self.transform.rotate
    }
    /// Get the scale of the current node
    /// Original function: gfdNodeGetScale
    pub fn get_scale(&self) -> Vec3A {
        self.transform.scale
    }

    // For callbacks to edit node properties
    pub fn get_translate_mut(&mut self) -> &mut Vec3A {
        &mut self.transform.translate
    }
    pub fn get_rotate_mut(&mut self) -> &mut Quat {
        &mut self.transform.rotate
    }
    pub fn get_scale_mut(&mut self) -> &mut Vec3A {
        &mut self.transform.scale
    }

    // Adapted version of get_XXXX_mut for imgui-rs
    pub fn get_translate_mut_f32(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(&raw mut self.transform.translate as *mut [f32; 3]) }
    }
    pub fn get_rotate_mut_f32(&mut self) -> &mut [f32; 4] {
        unsafe { std::mem::transmute::<_, &mut [f32; 4]>(&mut self.transform.rotate) }
    }
    pub fn get_scale_mut_f32(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(&raw mut self.transform.scale as *mut [f32; 3]) }
    }
    pub fn get_world_transform_mut_f32(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(&raw mut self.world_tm.w_axis as *mut [f32; 3]) }
    }

    /// Get the world transformation matrix for the current node
    /// Original function: gfdNodeGetWorldTransform
    pub fn get_world_transform(&self) -> Mat4 {
        self.world_tm
    }

    pub fn get_world_translation(&self) -> Vec3A {
        Vec3A::from_vec4(self.world_tm.w_axis.into())
    }

    pub fn has_object(&self) -> bool {
        self.object_head.is_some()
    }

    pub fn is_leaf(&self) -> bool {
        self.get_child_count() == 0
    }

    /// Set the position of the current node in world space
    /// Original function: gfdNodeSetTranslate
    pub fn set_translate(&mut self, value: Vec3A) {
        self.transform.translate = value;
    }
    /// Set the rotation of the current node from a quarternion
    /// Original function: gfdNodeSetRotate
    pub fn set_rotate(&mut self, value: Quat) {
        self.transform.rotate = value;
    }
    /// Set the rotation of the current node from a normalized rotation axis and angle in radians.
    /// Original function: gfdNodeSetRotateAxis
    pub fn set_rotate_axis(&mut self, axis: Vec3A, angle: f32) {
        let quat = Quat::from_axis_angle(axis.into(), angle);
        self.transform.rotate = quat;
    }
    /// Set the rotation of the current node
    /// Original function: gfdNodeSetRotate
    pub fn set_scale(&mut self, value: Vec3A) {
        self.transform.scale = value;
    }
}

impl<A> Display for Node<A>
where A: Allocator + Clone
{
    /// Formats a simple display with the node's name and it's basic transform parameters (position,
    /// rotation and scale). impl Debug is used to reimplement gfdNodePrintf
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_string = format!("{} <T: {} R: {} S: {}>", 
            self.get_name_platform(), self.get_translate(),
            self.get_rotate(), self.get_scale());
        write!(f, "{}", fmt_string)
    }
}

impl<A> Node<A>
where A: Allocator + Clone
{
    pub fn fmt_hierarchy(&self) -> String {
        let mut root =  format!("{}\n", self);
        let children: Vec<_> = NodeIteratorDepthLimited::from_node(self).collect();
        for child in children {
            root += &format!("{}{}\n", "\t".repeat(child.get_depth()), &*child);
        }
        root
    }
}

impl<A> Debug for Node<A>
where A: Allocator + Clone
{
    /// Detailed description of the node's current state, including properties and attachments.
    /// Designed to closely resemble gfdNodePrintf
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_string = format!("-----------------------------------------------\n[{} @ 0x{:x}]\n", 
            self.get_name_platform(), &raw const *self as usize);
        fmt_string.push_str(&format!("\n(transform)\npos: {}\nrot: {}\nscl: {}\n", 
            self.get_translate(), self.get_rotate(), self.get_scale()));
        fmt_string.push_str(&format!("\n(worldTm)\n{}\n\n(localTm)\n{}\n\n", 
            self.get_world_transform(), self.get_local_transform()));
        write!(f, "{}", fmt_string)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct NodeTransform {
    translate: Vec3A,
    rotate: Quat,
    scale: Vec3A
}

#[repr(C)]
#[derive(Debug)]
pub struct NodeLink<A = GfdAllocator> 
where A: Allocator + Clone {
    root: Option<NonNull<Node<A>>>,
    child: Option<NonNull<Node<A>>>,
    next: Option<NonNull<Node<A>>>,
    skin_bone_index: u16,
    terminate: u16
}

impl<A> NodeLink<A>
where A: Allocator + Clone
{
    pub(crate) fn get_root(&self) -> Option<&Node<A>> {
        self.root.map(|v| unsafe { v.as_ref() })
    }
    pub(crate) fn get_child(&self) -> Option<&Node<A>> {
        self.child.map(|v| unsafe { v.as_ref() })
    }
    pub(crate) fn get_next(&self) -> Option<&Node<A>> {
        self.next.map(|v| unsafe { v.as_ref() })
    }
}
use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::marker::PhantomPinned;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::{
    kernel::asset::{ Asset, AssetTypeHandle },
    utility::reference::Reference
};
use crate::kernel::allocator::GfdAllocator;
use allocator_api2::boxed::Box as ABox;
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(C)]
#[derive(Debug)]
pub struct EPLMaterial<A = GfdAllocator>
where A: Allocator + Clone {
    asset: Option<NonNull<Asset<A>>>,
    handle: Option<NonNull<AssetTypeHandle>>,
    _allocator: A
}

#[repr(C)]
#[derive(Debug)]
pub struct EPLParameter<T = (), A = GfdAllocator>
where A: Allocator + Clone {
    // always points to self->data
    p_data: NonNull<T>,
    size: u32,
    ref_: Reference,
    _allocator: A,
    _pinned: PhantomPinned,
    data: T
}

impl<T, A> EPLParameter<T, A>
where A: Allocator + Clone {
    pub fn new(data: T, alloc: A) -> ABox<Self, A> {
        let start = Self {
            p_data: NonNull::dangling(),
            size: size_of::<T>() as u32,
            ref_: Reference::new(),
            _allocator: alloc.clone(),
            _pinned: PhantomPinned,
            data
        };
        let mut boxed = ABox::new_in(start, alloc.clone());
        boxed.p_data = NonNull::from(&boxed.data);
        boxed
    }
}

impl<AStream, AObject, TStream, TObject> GfdSerialize<AStream, TStream, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for EPLParameter<TObject, AObject>
where TStream: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone,
      TObject: GfdSerialize<AStream, TStream>
{
    fn stream_read(stream: &mut Stream<AStream, TStream>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.size = size_of::<TObject>() as u32;
        this.ref_ = Reference::new();
        this.p_data = NonNull::from(&this.data);
        this.data = TObject::stream_read(stream, &mut ())?.into_raw();
        Ok(this)
    }
}

impl<T, A> Deref for EPLParameter<T, A>
where A: Allocator + Clone {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, A> DerefMut for EPLParameter<T, A>
where A: Allocator + Clone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Reference(u32);

/*
pub trait ReferenceCounted {
}

#[allow(dead_code)]
pub struct ReferenceEx<T: ReferenceCounted> {
    count: u32,
    _phantom: std::marker::PhantomData<T>
}
*/

#[repr(C)]
#[derive(Debug)]
pub struct Reference(u32);
impl Reference {
    pub fn new() -> Self { Reference(1) }
}

/*
pub trait ReferenceCounted {
}

#[allow(dead_code)]
pub struct ReferenceEx<T: ReferenceCounted> {
    count: u32,
    _phantom: std::marker::PhantomData<T>
}
*/

pub trait GfdRcType {
    /// (Original function: gfdMutexAddRef)
    fn add_ref(&self) -> u32;
    fn release(&self) -> u32;
}

pub struct GfdRc<T> 
where T: GfdRcType
{
    inner: std::mem::ManuallyDrop<T>
}

impl<T> Drop for GfdRc<T> 
where T: GfdRcType
{
    fn drop(&mut self) {
        if self.inner.release() == 1 {
            unsafe { std::ptr::drop_in_place(&raw mut self.inner); }
        }
    }
}
/*
impl<T> Clone for GfdRc<T>
where T: GfdRcType
{
    fn clone(&self) -> Self {
        self.inner.add_ref()
    }
}
*/

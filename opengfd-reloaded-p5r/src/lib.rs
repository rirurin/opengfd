pub mod adx;
#[path = "globals_xrd744.rs"]
pub mod globals;
pub mod graphics;
pub mod io;
mod init;

pub mod util {
    #[repr(C)]
    pub struct Ptr<T>(*mut T);
    unsafe impl<T> Send for Ptr<T> {}
    unsafe impl<T> Sync for Ptr<T> {}
    impl<T> std::ops::Deref for Ptr<T> {
        type Target = *mut T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> Ptr<T> {
        pub fn new(val: *mut T) -> Self {
            Self(val)
        }
    }
}
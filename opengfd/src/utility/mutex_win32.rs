use std::{
    mem::MaybeUninit,
    sync::atomic::AtomicU32
};
use windows::Win32::System::Threading::{ 
    CRITICAL_SECTION, 
    DeleteCriticalSection, 
    EnterCriticalSection,
    InitializeCriticalSectionAndSpinCount,
    // LeaveCriticalSection
};

#[derive(Debug)]
pub struct RecursiveMutexGuard;
impl Drop for RecursiveMutexGuard {
    fn drop(&mut self) {
        
    }
}

#[derive(Debug)]
pub struct RecursiveMutex(CRITICAL_SECTION);
impl RecursiveMutex {
    /// (Original function: gfdRecursiveMutexInitialize)
    pub fn new() -> Self {
        let mut platform: MaybeUninit<CRITICAL_SECTION> = MaybeUninit::uninit(); 
        // SAFETY: Valid pointer to stack-allocated platform variable
        // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-initializecriticalsectionandspincount#return-value
        unsafe { InitializeCriticalSectionAndSpinCount(platform.as_mut_ptr(), 1500).unwrap() };
        Self(unsafe { platform.assume_init() })
    }
    pub fn lock(&mut self) {
        unsafe { EnterCriticalSection(&mut self.0 as *mut CRITICAL_SECTION) }
    }
    /*
    // unlock is implicitly called when the lock goes out of scope
    pub fn unlock(&mut self) {
        unsafe { LeaveCriticalSection(&mut self.0 as *mut CRITICAL_SECTION) }
    }
    */
}

impl Drop for RecursiveMutex {
    fn drop(&mut self) {
        // SAFETY: self.0 can't be accessed after this
        unsafe { DeleteCriticalSection(&mut self.0 as *mut CRITICAL_SECTION); }
    }
}

unsafe impl Send for RecursiveMutex {}
unsafe impl Sync for RecursiveMutex {}

#[derive(Debug)]
pub struct Mutex(AtomicU32);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    pub fn new() -> Self { Self(AtomicU32::new(0)) }
}

pub mod ffi {

}

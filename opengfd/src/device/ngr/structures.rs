use crate::{
    device::ngr::hint::MemHint,
    globals,
    utility::mutex::{ RecursiveMutex, RecursiveMutexGuard }
};
use std::ptr::NonNull;
use windows::Win32::System::Threading::CRITICAL_SECTION;
/*
#[repr(C)]
#[derive(Debug)]
// https://en.wikipedia.org/wiki/Left-leaning_red%E2%80%93black_tree
pub struct LLRBTree<K, V> {
    _cpp_vtable: *mut u8,
    _key: std::marker::PhantomData<V>,
    _value: std::marker::PhantomData<K>,
    size: usize
}
*/
#[repr(C)]
#[derive(Debug)]
pub struct List {
    _cpp_vtable: *mut u8,

}
#[repr(C)]
#[derive(Debug)]
pub struct PointerList<V> 
{
    _cpp_vtable: *mut u8,
    _head: Option<NonNull<PointerListEntry<V>>>,
    _tail: Option<NonNull<PointerListEntry<V>>>,
    size: usize,
    hint: MemHint
}

impl<V> PointerList<V> {
    pub fn find_by_predicate<F>(&self, entry: F) -> Option<&V>
        where F: Fn(&V) -> bool {
        let mut current = self._head;
        while let Some(e) = current {
            let data = unsafe { e.as_ref().data.as_ref() };
            if entry(data) { return Some(data); }
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
}

impl<V> PointerList<V> 
    where V: PartialEq
{
    pub fn find(&self, entry: &V) -> Option<&V> {
        let mut current = self._head;
        while let Some(e) = current {
            let data = unsafe { e.as_ref().data.as_ref() };
            if data == entry { return Some(data); }
            let next = unsafe { e.as_ref().next };
            current = match next {
                Some(v) => Some(v),
                None => break
            }
        }
        None
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PointerListEntry<V> 
    // where V: PartialEq
{
    next: Option<NonNull<PointerListEntry<V>>>,
    prev: Option<NonNull<PointerListEntry<V>>>,
    data: NonNull<V>,
}

#[repr(C)]
#[derive(Debug)]
pub struct CriticalSection {
    _cpp_vtable: *mut u8,
    crit_section: RecursiveMutex
}

impl CriticalSection {
    pub fn lock(&mut self) -> RecursiveMutexGuard<'_> {
        self.crit_section.lock()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CrcHash {
    _cpp_vtable: *const u8,
    hash: u32
}

impl CrcHash {
    pub fn new<T: std::hash::Hash>(val: &T) -> Self {
        let mut hasher = crc32fast::Hasher::new();
        val.hash(&mut hasher); 
        Self {
            _cpp_vtable: match globals::get_ngr_crchash_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            hash: hasher.finalize()
        }
    }
    pub fn get_hash(&self) -> u32 { self.hash }
}

#[cfg(test)]
pub mod tests {
    use crate::device::ngr::renderer::state::{
        CullMode,
        FillMode,
        RasterizerKey
    };
    use crate::tests::{ OpengfdError, TestReturn };
    use std::hash::Hash;

    #[test]
    pub fn hash_rasterizer_platform() -> TestReturn {
        // Sample value collected from Metaphor
        let key = RasterizerKey {
            field_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            is_front_counter_clockwise: true,
            scissor_enable: false,
            antialiased_line_enable: true,
            depth_bias: 0,
            depth_bias_clamp: 0f32,
            slope_scaled_depth_bias: 0f32,
            depth_clip_enable: true
        };
        let mut hasher = crc32fast::Hasher::new();
        key.hash(&mut hasher);
        let expected: u32 = 0x1815a91a;
        let result = hasher.finalize();
        if expected == result {
            Ok(())
        } else {
            Err(Box::new(OpengfdError::new(
                format!("Incorrect hash value. Got 0x{:x}, expected 0x{:x}", result, expected)
            )))
        }
    }
}

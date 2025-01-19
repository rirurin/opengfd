#![allow(dead_code, unused_imports)]
use allocator_api2::alloc::{ Allocator, Global };
use cpp_types::msvc::{
    hash::FNV1A,
    list::{ List, ListNode, ListSingleNode, ListDoubleNode },
    mutex::{ ConditionVariable, Mutex },
    shared_ptr::SharedPtr,
    string::String as CppString,
    unordered::Map,
};
use crate::tpl::resource::Resource;

#[repr(C)]
pub struct FileManager {
    bool_ptr: SharedPtr<bool, Global>,
    mutex: Mutex,
    mutex_ptr: SharedPtr<Mutex, Global>,
    cond: ConditionVariable,
    fieldb0: usize,
    fieldb8: u8,
    thread_info: [usize; 2],
    load_file_list: List<ListNode<SharedPtr<*const Resource, Global>>, SharedPtr<*const Resource, Global>, Global>,
    load_files: Map<FNV1A, CppString, *const Resource, Global>,
    field120: [u8; 0x40]
}

use crate::utility::item_array::ItemArray;

#[repr(C)]
#[derive(Debug)]
pub struct SkinBoneObject {
    pub offset_rev_matrix: *mut ItemArray<usize>,
    pub node_index: *mut ItemArray<usize>,
    pub field2_0x10: u8,
    pub ref_: i32,
}

use crate::graphics::cull::CullObject;

#[repr(C)]
#[derive(Debug)]
pub struct RenderOtList {
    field0: [u8; 0x18],
    head: *mut RenderOt,
    tail: *mut RenderOt,
    head_cull: *mut RenderOt,
    tail_cull: *mut RenderOt,
    field38: [u8; 8]
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderOt {
    data: *mut u8,
    pre_cb_data: *mut u8,
    post_cb_data: *mut u8,
    pre_cb: Option<fn(*mut RenderOt, *mut u8) -> ()>,
    post_cb: Option<fn(*mut RenderOt, *mut u8) -> ()>,
    next: *mut RenderOt,
    next_cull: *mut RenderOt,
    geometry_cull: *mut CullObject
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderOtGroup {
    prio_ot: [u32; 7]
}

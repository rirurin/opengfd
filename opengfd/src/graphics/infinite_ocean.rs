#[derive(Debug)]
#[repr(C)]
pub struct InfiniteOcean {
    p5_0: f32,
    p5_3: f32,
    p5_1: f32,
    tc_scale: f32,
    ocean_depth_scale: f32,
    disturbance_camera_scale: f32,
    disturbance_depth_scale: f32,
    scattering_camera_scale: f32,
    disturbance_tolerance: f32,
    foam_distance: f32,
    caustics_tolerance: f32,
    has_water_reflection: bool,
    outline_attenuation_invalid: bool
}
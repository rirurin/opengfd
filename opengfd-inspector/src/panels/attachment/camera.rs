use imgui::Ui;
use opengfd::object::{
    camera::Camera,
    object::Object
};
use opengfd_inspector_components::panel::BasicPanel;
use std::ops::{ Deref, DerefMut };
#[derive(Debug)]
pub struct CameraProperties(&'static mut Camera);
impl Deref for CameraProperties {
    type Target = Camera;
    fn deref(&self) -> &Self::Target {
        self.0
    }
} 
impl DerefMut for CameraProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
impl CameraProperties {
    pub unsafe fn new(obj: &'static Object) -> Self {
        Self(unsafe { &mut *(&raw const *obj as *mut Camera) })
    } 
}
impl BasicPanel for CameraProperties {
    fn draw(&mut self, ui: &mut Ui) {
        ui.text("From View Matrix:");
        // note: this isn't correct...
        let trs = self.get_scale_rotation_translation_mut();
        let mut trans: [f32; 3] = trs.2.into();
        let mut rot: [f32; 4] = trs.1.into();
        let mut scale: [f32; 3] = trs.0.into();
        ui.input_float3("Translation", &mut trans).build();
        ui.input_float4("Rotate", &mut rot).build();
        ui.input_float3("Scale", &mut scale).build();
        ui.separator();
        ui.text("Properties:");

        ui.slider_config("Near Clip##Camera Details", 0., 100000.)
            .display_format("%f")
            .build(self.get_near_clip_mut());
        ui.slider_config("Far Clip##Camera Details", self.get_near_clip(), 200000.)
            .display_format("%f")
            .build(self.get_far_clip_mut());

        ui.slider_config("FOV##Camera Details", 30., 70.)
            .display_format("%f")
            .build(self.get_fovy_mut());

        ui.slider_config("Aspect Ratio##Camera Details", 1., 2.5)
            .display_format("%f")
            .build(self.get_aspect_ratio_mut());

        ui.slider_config("Roll##Camera Details", 0., 50.)
            .display_format("%f")
            .build(self.get_roll_mut());

        ui.separator();
        ui.text("Unknown properties:");
        ui.input_float("Field198", self.get_field198_mut()).build();
        ui.input_float("Field19c", self.get_field19c_mut()).build();
        ui.input_float("Field1a0", self.get_field1a0_mut()).build();
    }
}
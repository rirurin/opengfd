use imgui::Ui;
use opengfd::utility::misc::{ BoundingBox, BoundingSphere };

pub fn draw_bounding_box(bb: &mut BoundingBox, ui: &Ui) {
    ui.text("Bounding Box");
    ui.input_float3(
        format!("min##BoundingBox_{:x}", &raw const *bb as usize),
        bb.get_min_mut_f32()
    ).build();
    ui.input_float3(
        format!("max##BoundingBox_{:x}", &raw const *bb as usize),
        bb.get_max_mut_f32()
    ).build();
}
pub fn draw_bounding_circle(bc: &mut BoundingSphere, ui: &Ui) {
    ui.text("Bounding Circle");
    ui.input_float3(
        format!("center##BoundingBox_{:x}", &raw const *bc as usize), 
        bc.get_center_mut_f32()
    ).build();
    ui.input_float(
        format!("center##BoundingBox_{:x}", &raw const *bc as usize),
        bc.get_radius_mut_f32()
    ).build();
}
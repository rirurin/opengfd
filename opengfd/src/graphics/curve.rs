use glam::Vec2;
use std::fmt::{ Debug, Formatter };

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurveType {
    Integer = 0,
    Float = 1,
    Color = 2,
    Vec2 = 3
}

#[repr(C)]
pub(crate) union CurveTarget {
    integer: i32,
    float: f32,
    color: u32,
    range: Vec2
}

/// Two-point bezier curve, with a start and end point. Mostly used for particle systems.
#[repr(C)]
pub struct Curve2 {
    curve_type: CurveType,
    start: CurveTarget,
    end: CurveTarget,
    point: [Vec2; 2],
    table: [u16; 23]
}

impl Debug for Curve2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Curve2 {{ type: {:?}, points: ({}, {}) }}", self.curve_type, self.point[0], self.point[1])
    }
}

/// Four-point bezier curve. Mostly used for particle systems.
#[repr(C)]
pub struct Curve4 {
    curve_type: u16,
    target: [CurveTarget; 4],
    point: [Vec2; 2],
    table: [u16; 23]
}

impl Debug for Curve4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Curve4 {{ type: {:?}, points: ({}, {}) }}", self.curve_type, self.point[0], self.point[1])
    }
}

pub mod ffi {

}

#[cfg(test)]
pub mod tests {

}

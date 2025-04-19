#![allow(dead_code)]
use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoundingBox {
    max: Vec3,
    min: Vec3
}
impl BoundingBox {
    pub fn get_max(&self) -> Vec3 { self.max }
    pub fn get_min(&self) -> Vec3 { self.min }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoundingSphere {
    center: Vec3,
    radius: f32
}
#[derive(Debug, Clone, Copy)]
pub struct RGB(glam::U8Vec3);

impl RGB {
    pub const fn from_rgb_u32(v: u32) -> Self {
        Self::from_rgb_u8(
            (v >> 0x10 & 0xff) as u8,
            (v >> 0x8 & 0xff) as u8,
            (v & 0xff) as u8,
        )
    }
    pub const fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self(glam::U8Vec3::new(r, g, b))
    }

    pub fn get_red(&self) -> u8 { self.0.x }
    pub fn get_green(&self) -> u8 { self.0.y }
    pub fn get_blue(&self) -> u8 { self.0.z }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBA(glam::U8Vec4);
impl RGBA {
    pub const fn from_argb_u32(v: u32) -> Self {
        Self::from_argb_u8(
            (v >> 0x18 & 0xff) as u8,
            (v >> 0x10 & 0xff) as u8,
            (v >> 0x8 & 0xff) as u8,
            (v & 0xff) as u8,
        )
    }
    pub const fn from_rgba_u32(v: u32) -> Self {
        Self::from_rgba_u8(
            (v & 0xff) as u8,
            (v >> 0x8 & 0xff) as u8,
            (v >> 0x10 & 0xff) as u8,
            (v >> 0x18 & 0xff) as u8,
        )
    }
    pub const fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(glam::U8Vec4::new(r, g, b, a))
    }
    pub const fn from_argb_u8(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(glam::U8Vec4::new(r, g, b, a))
    }
    pub const fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self { 
        Self::from_argb_u8(0xff, r, g, b) 
    }
    pub fn get_red(&self) -> u8 { self.0.x }
    pub fn get_green(&self) -> u8 { self.0.y }
    pub fn get_blue(&self) -> u8 { self.0.z }
    pub fn get_alpha(&self) -> u8 { self.0.w }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBFloat(glam::Vec3);

impl RGBFloat {
    pub const fn from_rgb_array_f32(val: [f32; 3]) -> Self { 
        RGBFloat(glam::Vec3::new(val[0], val[1], val[2])) 
    }
    pub const fn from_single_f32(val: f32) -> Self { RGBFloat(glam::Vec3::splat(val))}

    pub const fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self { Self::from_rgb_array_u8([r, g, b]) }
    pub const fn from_single_u8(v: u8) -> Self { Self::from_rgb_array_u8([v; 3])}
    pub const fn from_rgb_array_u8(val: [u8; 3]) -> Self {
        RGBFloat(glam::Vec3::new(
            val[0] as f32 / 255f32,
            val[1] as f32 / 255f32,
            val[2] as f32 / 255f32,
        ))
    }

    pub const fn from_rgb_u32(val: u32) -> Self {
        Self::from_rgb_array_u8([
            (val >> 0x10 & 0xff) as u8,
            (val >> 0x8 & 0xff) as u8,
            (val & 0xff) as u8,
        ])
    }
    pub const fn get_red(&self) -> u8 { (self.0.x * 255f32) as u8 }
    pub const fn get_green(&self) -> u8 { (self.0.y * 255f32) as u8 }
    pub const fn get_blue(&self) -> u8 { (self.0.z * 255f32) as u8 }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBAFloat([f32; 4]);
impl RGBAFloat {
    pub const fn from_rgba_array_f32(val: [f32; 4]) -> Self { RGBAFloat(val) }
    pub const fn from_argb_array_f32(val: [f32; 4]) -> Self { RGBAFloat([val[1], val[2], val[3], val[0]]) }
    pub const fn from_single_f32(val: f32) -> Self { RGBAFloat([val; 4])}

    pub const fn from_argb_u8(a: u8, r: u8, g: u8, b: u8) -> Self { Self::from_rgba_array_u8([r, g, b, a]) }
    pub const fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self { Self::from_rgba_array_u8([r, g, b, a]) }
    pub const fn from_single_u8(v: u8) -> Self { Self::from_rgba_array_u8([v; 4])}
    pub const fn from_argb_array_u8(val: [u8; 4]) -> Self {
        RGBAFloat([
            val[1] as f32 / 255f32,
            val[2] as f32 / 255f32,
            val[3] as f32 / 255f32,
            val[0] as f32 / 255f32,
        ])
    }
    pub const fn from_rgba_array_u8(val: [u8; 4]) -> Self {
        RGBAFloat([
            val[0] as f32 / 255f32,
            val[1] as f32 / 255f32,
            val[2] as f32 / 255f32,
            val[3] as f32 / 255f32,
        ])
    }

    pub const fn from_rgba_u32(val: u32) -> Self {
        Self::from_argb_array_u8([
            (val >> 0x18 & 0xff) as u8,
            (val >> 0x10 & 0xff) as u8,
            (val >> 0x8 & 0xff) as u8,
            (val & 0xff) as u8,
        ])
    }
    pub const fn get_red(&self) -> u8 { (self.0[0] * 255f32) as u8 }
    pub const fn get_green(&self) -> u8 { (self.0[1] * 255f32) as u8 }
    pub const fn get_blue(&self) -> u8 { (self.0[2] * 255f32) as u8 }
    pub const fn get_alpha(&self) -> u8 { (self.0[3] * 255f32) as u8 }
}
impl From<RGBAFloat> for RGBA {
    fn from(value: RGBAFloat) -> Self {
        Self::from_rgba_u8(
            (value.0[0] * 255f32) as u8,
            (value.0[1] * 255f32) as u8,
            (value.0[2] * 255f32) as u8,
            (value.0[3] * 255f32) as u8,
        )
    }
}
impl From<RGBAFloat> for [f32; 4] {
    fn from(value: RGBAFloat) -> Self {
        [value.0[0], value.0[1], value.0[2], value.0[3]]
    }
}
impl From<RGBAFloat> for RGBFloat {
    fn from(value: RGBAFloat) -> Self {
        Self::from_rgb_array_f32([value.0[0], value.0[1], value.0[2]])
    }
}

/// See https://www.w3.org/TR/css-color-4/#named-colors
pub mod web_colors {
    pub const TRANSPARENT: super::RGBA = super::RGBA::from_argb_u32(0x00FFFFFF);
    pub const ALICEBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFFF0F8FF);
    pub const ANTIQUEWHITE: super::RGBA = super::RGBA::from_argb_u32(0xFFFAEBD7);
    pub const AQUA: super::RGBA = super::RGBA::from_argb_u32(0xFF00FFFF);
    pub const AQUAMARINE: super::RGBA = super::RGBA::from_argb_u32(0xFF7FFFD4);
    pub const AZURE: super::RGBA = super::RGBA::from_argb_u32(0xFFF0FFFF);
    pub const BEIGE: super::RGBA = super::RGBA::from_argb_u32(0xFFF5F5DC);
    pub const BISQUE: super::RGBA = super::RGBA::from_argb_u32(0xFFFFE4C4);
    pub const BLACK: super::RGBA = super::RGBA::from_argb_u32(0xFF000000);
    pub const BLANCHEDALMOND: super::RGBA = super::RGBA::from_argb_u32(0xFFFFEBCD);
    pub const BLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF0000FF);
    pub const BLUEVIOLET: super::RGBA = super::RGBA::from_argb_u32(0xFF8A2BE2);
    pub const BROWN: super::RGBA = super::RGBA::from_argb_u32(0xFFA52A2A);
    pub const BURLYWOOD: super::RGBA = super::RGBA::from_argb_u32(0xFFDEB887);
    pub const CADETBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF5F9EA0);
    pub const CHARTREUSE: super::RGBA = super::RGBA::from_argb_u32(0xFF7FFF00);
    pub const CHOCOLATE: super::RGBA = super::RGBA::from_argb_u32(0xFFD2691E);
    pub const CORAL: super::RGBA = super::RGBA::from_argb_u32(0xFFFF7F50);
    pub const CORNFLOWERBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF6495ED);
    pub const CORNSILK: super::RGBA = super::RGBA::from_argb_u32(0xFFFFF8DC);
    pub const CRIMSON: super::RGBA = super::RGBA::from_argb_u32(0xFFDC143C);
    pub const CYAN: super::RGBA = super::RGBA::from_argb_u32(0xFF00FFFF);
    pub const DARKBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF00008B);
    pub const DARKCYAN: super::RGBA = super::RGBA::from_argb_u32(0xFF008B8B);
    pub const DARKGOLDENROD: super::RGBA = super::RGBA::from_argb_u32(0xFFB8860B);
    pub const DARKGRAY: super::RGBA = super::RGBA::from_argb_u32(0xFFA9A9A9);
    pub const DARKGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF006400);
    pub const DARKKHAKI: super::RGBA = super::RGBA::from_argb_u32(0xFFBDB76B);
    pub const DARKMAGENTA: super::RGBA = super::RGBA::from_argb_u32(0xFF8B008B);
    pub const DARKOLIVEGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF556B2F);
    pub const DARKORANGE: super::RGBA = super::RGBA::from_argb_u32(0xFFFF8C00);
    pub const DARKORCHID: super::RGBA = super::RGBA::from_argb_u32(0xFF9932CC);
    pub const DARKRED: super::RGBA = super::RGBA::from_argb_u32(0xFF8B0000);
    pub const DARKSALMON: super::RGBA = super::RGBA::from_argb_u32(0xFFE9967A);
    pub const DARKSEAGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF8FBC8F);
    pub const DARKSLATEBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF483D8B);
    pub const DARKSLATEGRAY: super::RGBA = super::RGBA::from_argb_u32(0xFF2F4F4F);
    pub const DARKTURQUOISE: super::RGBA = super::RGBA::from_argb_u32(0xFF00CED1);
    pub const DARKVIOLET: super::RGBA = super::RGBA::from_argb_u32(0xFF9400D3);
    pub const DEEPPINK: super::RGBA = super::RGBA::from_argb_u32(0xFFFF1493);
    pub const DEEPSKYBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF00BFFF);
    pub const DIMGRAY: super::RGBA = super::RGBA::from_argb_u32(0xFF696969);
    pub const DODGERBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF1E90FF);
    pub const FIREBRICK: super::RGBA = super::RGBA::from_argb_u32(0xFFB22222);
    pub const FLORALWHITE: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFAF0);
    pub const FORESTGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF228B22);
    pub const FUCHSIA: super::RGBA = super::RGBA::from_argb_u32(0xFFFF00FF);
    pub const GAINSBORO: super::RGBA = super::RGBA::from_argb_u32(0xFFDCDCDC);
    pub const GHOSTWHITE: super::RGBA = super::RGBA::from_argb_u32(0xFFF8F8FF);
    pub const GOLD: super::RGBA = super::RGBA::from_argb_u32(0xFFFFD700);
    pub const GOLDENROD: super::RGBA = super::RGBA::from_argb_u32(0xFFDAA520);
    pub const GRAY: super::RGBA = super::RGBA::from_argb_u32(0xFF808080);
    pub const GREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF008000);
    pub const GREENYELLOW: super::RGBA = super::RGBA::from_argb_u32(0xFFADFF2F);
    pub const HONEYDEW: super::RGBA = super::RGBA::from_argb_u32(0xFFF0FFF0);
    pub const HOTPINK: super::RGBA = super::RGBA::from_argb_u32(0xFFFF69B4);
    pub const INDIANRED: super::RGBA = super::RGBA::from_argb_u32(0xFFCD5C5C);
    pub const INDIGO: super::RGBA = super::RGBA::from_argb_u32(0xFF4B0082);
    pub const IVORY: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFFF0);
    pub const KHAKI: super::RGBA = super::RGBA::from_argb_u32(0xFFF0E68C);
    pub const LAVENDER: super::RGBA = super::RGBA::from_argb_u32(0xFFE6E6FA);
    pub const LAVENDERBLUSH: super::RGBA = super::RGBA::from_argb_u32(0xFFFFF0F5);
    pub const LAWNGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF7CFC00);
    pub const LEMONCHIFFON: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFACD);
    pub const LIGHTBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFFADD8E6);
    pub const LIGHTCORAL: super::RGBA = super::RGBA::from_argb_u32(0xFFF08080);
    pub const LIGHTCYAN: super::RGBA = super::RGBA::from_argb_u32(0xFFE0FFFF);
    pub const LIGHTGOLDENRODYELLOW: super::RGBA = super::RGBA::from_argb_u32(0xFFFAFAD2);
    pub const LIGHTGRAY: super::RGBA = super::RGBA::from_argb_u32(0xFFD3D3D3);
    pub const LIGHTGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF90EE90);
    pub const LIGHTPINK: super::RGBA = super::RGBA::from_argb_u32(0xFFFFB6C1);
    pub const LIGHTSALMON: super::RGBA = super::RGBA::from_argb_u32(0xFFFFA07A);
    pub const LIGHTSEAGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF20B2AA);
    pub const LIGHTSKYBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF87CEFA);
    pub const LIGHTSLATEGRAY: super::RGBA = super::RGBA::from_argb_u32(0xFF778899);
    pub const LIGHTSTEELBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFFB0C4DE);
    pub const LIGHTYELLOW: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFFE0);
    pub const LIME: super::RGBA = super::RGBA::from_argb_u32(0xFF00FF00);
    pub const LIMEGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF32CD32);
    pub const LINEN: super::RGBA = super::RGBA::from_argb_u32(0xFFFAF0E6);
    pub const MAGENTA: super::RGBA = super::RGBA::from_argb_u32(0xFFFF00FF);
    pub const MAROON: super::RGBA = super::RGBA::from_argb_u32(0xFF800000);
    pub const MEDIUMAQUAMARINE: super::RGBA = super::RGBA::from_argb_u32(0xFF66CDAA);
    pub const MEDIUMBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF0000CD);
    pub const MEDIUMORCHID: super::RGBA = super::RGBA::from_argb_u32(0xFFBA55D3);
    pub const MEDIUMPURPLE: super::RGBA = super::RGBA::from_argb_u32(0xFF9370DB);
    pub const MEDIUMSEAGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF3CB371);
    pub const MEDIUMSLATEBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF7B68EE);
    pub const MEDIUMSPRINGGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF00FA9A);
    pub const MEDIUMTURQUOISE: super::RGBA = super::RGBA::from_argb_u32(0xFF48D1CC);
    pub const MEDIUMVIOLETRED: super::RGBA = super::RGBA::from_argb_u32(0xFFC71585);
    pub const MIDNIGHTBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF191970);
    pub const MINTCREAM: super::RGBA = super::RGBA::from_argb_u32(0xFFF5FFFA);
    pub const MISTYROSE: super::RGBA = super::RGBA::from_argb_u32(0xFFFFE4E1);
    pub const MOCCASIN: super::RGBA = super::RGBA::from_argb_u32(0xFFFFE4B5);
    pub const NAVAJOWHITE: super::RGBA = super::RGBA::from_argb_u32(0xFFFFDEAD);
    pub const NAVY: super::RGBA = super::RGBA::from_argb_u32(0xFF000080);
    pub const OLDLACE: super::RGBA = super::RGBA::from_argb_u32(0xFFFDF5E6);
    pub const OLIVE: super::RGBA = super::RGBA::from_argb_u32(0xFF808000);
    pub const OLIVEDRAB: super::RGBA = super::RGBA::from_argb_u32(0xFF6B8E23);
    pub const ORANGE: super::RGBA = super::RGBA::from_argb_u32(0xFFFFA500);
    pub const ORANGERED: super::RGBA = super::RGBA::from_argb_u32(0xFFFF4500);
    pub const ORCHID: super::RGBA = super::RGBA::from_argb_u32(0xFFDA70D6);
    pub const PALEGOLDENROD: super::RGBA = super::RGBA::from_argb_u32(0xFFEEE8AA);
    pub const PALEGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF98FB98);
    pub const PALETURQUOISE: super::RGBA = super::RGBA::from_argb_u32(0xFFAFEEEE);
    pub const PALEVIOLETRED: super::RGBA = super::RGBA::from_argb_u32(0xFFDB7093);
    pub const PAPAYAWHIP: super::RGBA = super::RGBA::from_argb_u32(0xFFFFEFD5);
    pub const PEACHPUFF: super::RGBA = super::RGBA::from_argb_u32(0xFFFFDAB9);
    pub const PERU: super::RGBA = super::RGBA::from_argb_u32(0xFFCD853F);
    pub const PINK: super::RGBA = super::RGBA::from_argb_u32(0xFFFFC0CB);
    pub const PLUM: super::RGBA = super::RGBA::from_argb_u32(0xFFDDA0DD);
    pub const POWDERBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFFB0E0E6);
    pub const PURPLE: super::RGBA = super::RGBA::from_argb_u32(0xFF800080);
    pub const REBECCAPURPLE: super::RGBA = super::RGBA::from_argb_u32(0xFF663399);
    pub const RED: super::RGBA = super::RGBA::from_argb_u32(0xFFFF0000);
    pub const ROSYBROWN: super::RGBA = super::RGBA::from_argb_u32(0xFFBC8F8F);
    pub const ROYALBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF4169E1);
    pub const SADDLEBROWN: super::RGBA = super::RGBA::from_argb_u32(0xFF8B4513);
    pub const SALMON: super::RGBA = super::RGBA::from_argb_u32(0xFFFA8072);
    pub const SANDYBROWN: super::RGBA = super::RGBA::from_argb_u32(0xFFF4A460);
    pub const SEAGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF2E8B57);
    pub const SEASHELL: super::RGBA = super::RGBA::from_argb_u32(0xFFFFF5EE);
    pub const SIENNA: super::RGBA = super::RGBA::from_argb_u32(0xFFA0522D);
    pub const SILVER: super::RGBA = super::RGBA::from_argb_u32(0xFFC0C0C0);
    pub const SKYBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF87CEEB);
    pub const SLATEBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF6A5ACD);
    pub const SLATEGRAY: super::RGBA = super::RGBA::from_argb_u32(0xFF708090);
    pub const SNOW: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFAFA);
    pub const SPRINGGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF00FF7F);
    pub const STEELBLUE: super::RGBA = super::RGBA::from_argb_u32(0xFF4682B4);
    pub const TAN: super::RGBA = super::RGBA::from_argb_u32(0xFFD2B48C);
    pub const TEAL: super::RGBA = super::RGBA::from_argb_u32(0xFF008080);
    pub const THISTLE: super::RGBA = super::RGBA::from_argb_u32(0xFFD8BFD8);
    pub const TOMATO: super::RGBA = super::RGBA::from_argb_u32(0xFFFF6347);
    pub const TURQUOISE: super::RGBA = super::RGBA::from_argb_u32(0xFF40E0D0);
    pub const VIOLET: super::RGBA = super::RGBA::from_argb_u32(0xFFEE82EE);
    pub const WHEAT: super::RGBA = super::RGBA::from_argb_u32(0xFFF5DEB3);
    pub const WHITE: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFFFF);
    pub const WHITESMOKE: super::RGBA = super::RGBA::from_argb_u32(0xFFF5F5F5);
    pub const YELLOW: super::RGBA = super::RGBA::from_argb_u32(0xFFFFFF00);
    pub const YELLOWGREEN: super::RGBA = super::RGBA::from_argb_u32(0xFF9ACD32);
}

#[repr(C)]
#[derive(Debug)]
pub struct Fade {
    in_: f32,
    out_: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct Range {
    datums: f32,
    range: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct LinkedList<T> {
    pub(crate) head: *mut T,
    pub(crate) tail: *mut T
}

#[repr(C)]
#[derive(Debug)]
pub struct LinkedListNode<T> {
    pub(crate) prev: *mut T,
    pub(crate) next: *mut T
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Rect(glam::Vec4);
impl Rect {
    pub fn new(top_left: f32, top_right: f32, width: f32, height: f32) -> Self {
        Self(glam::Vec4::new(top_left, top_right, width, height))
    }
    pub fn get_top_left(&self) -> f32 { self.0.x }
    pub fn get_top_right(&self) -> f32 { self.0.y }
    pub fn get_width(&self) -> f32 { self.0.z }
    pub fn get_height(&self) -> f32 { self.0.w }
}

#![allow(non_snake_case, non_camel_case_types)]
use crate::{
    graphics::texture::Texture,
    object::mesh::Mesh,
    utility::{
        misc::{ RGBFloat, RGBAFloat },
        name::Name
    }
};
use glam::Vec3A;
use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 76usize)]
pub struct Graphics4D88 {
    #[field_offset(0usize)]
    pub hdrField04: f32,
    #[field_offset(4usize)]
    pub bloomScale: f32,
    #[field_offset(8usize)]
    pub adaptedLum: f32,
    #[field_offset(12usize)]
    pub toonBloomScale: f32,
    #[field_offset(16usize)]
    pub bloomAdjustToon: f32,
    #[field_offset(20usize)]
    pub hdrField03: u8,
    #[field_offset(24usize)]
    pub elapsedTime: f32,
    #[field_offset(28usize)]
    pub ToneMap: u8,
    #[field_offset(29usize)]
    pub StarFilter: u8,
    #[field_offset(30usize)]
    pub AdaptedLumAuto: u8,
    #[field_offset(32usize)]
    pub hdrField14: f32,
    #[field_offset(36usize)]
    pub starScale: f32,
    #[field_offset(40usize)]
    pub hdrField1C: f32,
    #[field_offset(44usize)]
    pub hdrField20: f32,
    #[field_offset(48usize)]
    pub hdrField24: f32,
    #[field_offset(52usize)]
    pub hdrField28: f32,
    #[field_offset(56usize)]
    pub hdrField2C: f32,
    #[field_offset(60usize)]
    pub hdrField30: f32,
    #[field_offset(64usize)]
    pub adaptedLumAdjust: f32,
    #[field_offset(68usize)]
    pub adaptedLumLimit: f32,
    #[field_offset(72usize)]
    pub pbrIntensity: f32,
}

#[ensure_layout(size = 32usize)]
pub struct Graphics4E28 {}
#[ensure_layout(size = 72usize)]
pub struct Graphics4E68 {}
#[ensure_layout(size = 12usize)]
pub struct Graphics51EC {}

#[ensure_layout(size = 72usize)]
pub struct Graphics51A4 {}
#[ensure_layout(size = 56usize)]
pub struct Graphics4560 {
    #[field_offset(0usize)]
    pub shadowType: u32,
    #[field_offset(4usize)]
    pub shadowFarClip: f32,
    #[field_offset(8usize)]
    pub shadowFarClip2: f32,
    #[field_offset(12usize)]
    pub field3_0xc: f32,
    #[field_offset(16usize)]
    pub field4_0x10: f32,
    #[field_offset(20usize)]
    pub field5_0x14: f32,
    #[field_offset(24usize)]
    pub field6_0x18: f32,
    #[field_offset(28usize)]
    pub field7_0x1c: f32,
    #[field_offset(32usize)]
    pub field8_0x20: f32,
    #[field_offset(36usize)]
    pub field9_0x24: f32,
    #[field_offset(40usize)]
    pub field10_0x28: f32,
    #[field_offset(44usize)]
    pub field11_0x2c: f32,
    #[field_offset(48usize)]
    pub field12_0x30: f32,
    #[field_offset(52usize)]
    pub field13_0x34: f32,
}
#[ensure_layout(size = 32usize)]
pub struct Graphics4DD4 {}
#[ensure_layout(size = 48usize)]
pub struct Graphics51F8 {
    #[field_offset(0usize)]
    pub field0_0x0: f32,
    #[field_offset(4usize)]
    pub field1_0x4: f32,
    #[field_offset(8usize)]
    pub field2_0x8: f32,
    #[field_offset(12usize)]
    pub field3_0xc: f32,
    #[field_offset(16usize)]
    pub field4_0x10: f32,
    #[field_offset(20usize)]
    pub field5_0x14: f32,
    #[field_offset(24usize)]
    pub field6_0x18: f32,
    #[field_offset(28usize)]
    pub field7_0x1c: f32,
    #[field_offset(32usize)]
    pub field8_0x20: f32,
    #[field_offset(36usize)]
    pub field9_0x24: f32,
    #[field_offset(40usize)]
    pub field10_0x28: f32,
    #[field_offset(44usize)]
    pub field11_0x2c: u8,
    #[field_offset(45usize)]
    pub field12_0x2d: u8,
}

#[ensure_layout(size = 32usize)]
pub struct ColorCorrect {
    #[field_offset(0usize)]
    pub field0_0x0: bool,
    #[field_offset(4usize)]
    pub field2_0x4: f32,
    #[field_offset(8usize)]
    pub field3_0x8: f32,
    #[field_offset(12usize)]
    pub field4_0xc: f32,
    #[field_offset(16usize)]
    pub field5_0x10: f32,
    #[field_offset(20usize)]
    pub field6_0x14: f32,
    #[field_offset(24usize)]
    pub field7_0x18: f32,
    #[field_offset(28usize)]
    pub field8_0x1c: f32,
}
#[ensure_layout(size = 32usize)]
pub struct DeviceColorCorrectionParams {
    #[field_offset(0usize)]
    pub field0_0x0: f32,
    #[field_offset(4usize)]
    pub field1_0x4: f32,
    #[field_offset(8usize)]
    pub field2_0x8: f32,
    #[field_offset(12usize)]
    pub field3_0xc: f32,
    #[field_offset(16usize)]
    pub field4_0x10: f32,
    #[field_offset(20usize)]
    pub field5_0x14: f32,
    #[field_offset(24usize)]
    pub field6_0x18: f32,
    #[field_offset(28usize)]
    pub field7_0x1c: f32,
}

#[ensure_layout(size = 8usize)]
pub struct Environment4DC {}
#[ensure_layout(size = 124usize)]
pub struct EnvironmentLight {
    #[field_offset(0usize)]
    pub active: bool,
    #[field_offset(4usize)]
    pub type_: u32,
    #[field_offset(8usize)]
    pub ambient: RGBAFloat,
    #[field_offset(24usize)]
    pub diffuse: RGBAFloat,
    #[field_offset(40usize)]
    pub specular: RGBAFloat,
    #[field_offset(56usize)]
    pub kC: f32,
    #[field_offset(60usize)]
    pub kL: f32,
    #[field_offset(64usize)]
    pub kQ: f32,
    #[field_offset(68usize)]
    pub dS: f32,
    #[field_offset(72usize)]
    pub dE: f32,
    #[field_offset(76usize)]
    pub theta: f32,
    #[field_offset(80usize)]
    pub phi: f32,
    #[field_offset(84usize)]
    pub position: Vec3A,
    #[field_offset(100usize)]
    pub direction: Vec3A,
    #[field_offset(116usize)]
    pub metaphor_1: f32,
    #[field_offset(120usize)]
    pub metaphor_2: f32,
}
#[ensure_layout(size = 76usize)]
pub struct EnvironmentClouds {}
#[ensure_layout(size = 76usize)]
pub struct Temperare {
    #[field_offset(0usize)]
    pub WobbFocalPlane: f32,
    #[field_offset(4usize)]
    pub WobbNearRange: f32,
    #[field_offset(8usize)]
    pub WobbFarRange: f32,
    #[field_offset(12usize)]
    pub WobbFarBlurLimit: f32,
    #[field_offset(16usize)]
    pub WobbPower: f32,
    #[field_offset(20usize)]
    pub EdgeFocalPlane: f32,
    #[field_offset(24usize)]
    pub EdgeNearRange: f32,
    #[field_offset(28usize)]
    pub EdgeFarRange: f32,
    #[field_offset(32usize)]
    pub EdgeFarBlurLimit: f32,
    #[field_offset(36usize)]
    pub EdgeSize: f32,
    #[field_offset(40usize)]
    pub EdgePower: f32,
    #[field_offset(44usize)]
    pub field11_0x2c: f32,
    #[field_offset(48usize)]
    pub field12_0x30: f32,
    #[field_offset(52usize)]
    pub field13_0x34: f32,
    #[field_offset(56usize)]
    pub field14_0x38: f32,
    #[field_offset(60usize)]
    pub field15_0x3c: f32,
    #[field_offset(64usize)]
    pub field16_0x40: f32,
    #[field_offset(68usize)]
    pub field17_0x44: f32,
    #[field_offset(72usize)]
    pub field18_0x48: f32,
}
#[ensure_layout(size = 80usize)]
pub struct EnvironmentTemperare {
    #[field_offset(4usize)]
    pub params: Temperare,
}
#[ensure_layout(size = 52usize)]
pub struct EnvironmentInfiniteOcean {
    #[field_offset(0usize)]
    pub field0_0x0: u8,
    #[field_offset(4usize)]
    pub field2_0x4: f32,
    #[field_offset(8usize)]
    pub field3_0x8: f32,
    #[field_offset(12usize)]
    pub field4_0xc: f32,
    #[field_offset(16usize)]
    pub field5_0x10: f32,
    #[field_offset(20usize)]
    pub field6_0x14: f32,
    #[field_offset(24usize)]
    pub field7_0x18: f32,
    #[field_offset(28usize)]
    pub field8_0x1c: f32,
    #[field_offset(32usize)]
    pub field9_0x20: f32,
    #[field_offset(36usize)]
    pub field10_0x24: f32,
    #[field_offset(40usize)]
    pub field11_0x28: f32,
    #[field_offset(44usize)]
    pub field12_0x2c: f32,
    #[field_offset(48usize)]
    pub field13_0x30: u8,
    #[field_offset(49usize)]
    pub field14_0x31: u8,
}
#[ensure_layout(size = 76usize)]
pub struct EnvironmentHDR {
    #[field_offset(0usize)]
    pub active: u8,
    #[field_offset(4usize)]
    pub params: DeviceHDRParams,
}
#[ensure_layout(size = 12usize)]
pub struct EnvironmentCamera {
    #[field_offset(0usize)]
    pub fovy: f32,
    #[field_offset(4usize)]
    pub nearClip: f32,
    #[field_offset(8usize)]
    pub farClip: f32,
}
#[ensure_layout(size = 76usize)]
pub struct EnvironmentFog1_Inner {
    #[field_offset(0usize)]
    pub field0_0x0: i32,
    #[field_offset(4usize)]
    pub field1_0x4: u8,
    #[field_offset(8usize)]
    pub field5_0x8: f32,
    #[field_offset(12usize)]
    pub field6_0xc: f32,
    #[field_offset(16usize)]
    pub field7_0x10: RGBAFloat,
    #[field_offset(32usize)]
    pub field8_0x20: f32,
    #[field_offset(36usize)]
    pub field9_0x24: f32,
    #[field_offset(40usize)]
    pub field10_0x28: RGBAFloat,
    #[field_offset(56usize)]
    pub field11_0x38: RGBAFloat,
    #[field_offset(72usize)]
    pub field12_0x48: u8,
}
#[ensure_layout(size = 80usize)]
pub struct EnvironmentFog1 {
    #[field_offset(0usize)]
    pub field0_0x0: i32,
    #[field_offset(4usize)]
    pub params: EnvironmentFog1_Inner,
}
#[ensure_layout(size = 72usize)]
pub struct EnvironmentFog2 {
    #[field_offset(4usize)]
    pub field4_0x4: f32,
    #[field_offset(8usize)]
    pub field5_0x8: f32,
    #[field_offset(12usize)]
    pub field6_0xc: RGBAFloat,
    #[field_offset(28usize)]
    pub field7_0x1c: f32,
    #[field_offset(32usize)]
    pub field8_0x20: f32,
    #[field_offset(36usize)]
    pub field9_0x24: RGBAFloat,
    #[field_offset(52usize)]
    pub field10_0x34: RGBAFloat,
    #[field_offset(68usize)]
    pub field11_0x44: u8,
}
#[ensure_layout(size = 36usize)]
pub struct EnvironmentColorCorrectNew {
    #[field_offset(0usize)]
    pub active: u8,
    #[field_offset(4usize)]
    pub params: DeviceColorCorrectionParams,
}
#[ensure_layout(size = 116usize)]
pub struct EnvironmentShadow {
    #[field_offset(0usize)]
    pub field0_0x0: bool,
    #[field_offset(1usize)]
    pub field1_0x1: bool,
    #[field_offset(24usize)]
    pub field24_0x18: f32,
    #[field_offset(28usize)]
    pub field25_0x1c: f32,
    #[field_offset(32usize)]
    pub field26_0x20: f32,
    #[field_offset(36usize)]
    pub field27_0x24: f32,
    #[field_offset(40usize)]
    pub field28_0x28: f32,
    #[field_offset(44usize)]
    pub field29_0x2c: f32,
    #[field_offset(48usize)]
    pub field30_0x30: f32,
    #[field_offset(52usize)]
    pub field31_0x34: f32,
    #[field_offset(60usize)]
    pub field36_0x3c: u32,
    #[field_offset(64usize)]
    pub field37_0x40: f32,
    #[field_offset(68usize)]
    pub field38_0x44: f32,
    #[field_offset(72usize)]
    pub field39_0x48: f32,
    #[field_offset(76usize)]
    pub field40_0x4c: f32,
    #[field_offset(80usize)]
    pub field41_0x50: f32,
    #[field_offset(84usize)]
    pub field42_0x54: f32,
    #[field_offset(88usize)]
    pub field43_0x58: f32,
    #[field_offset(92usize)]
    pub field44_0x5c: f32,
    #[field_offset(96usize)]
    pub field45_0x60: f32,
    #[field_offset(100usize)]
    pub field46_0x64: f32,
    #[field_offset(104usize)]
    pub field47_0x68: f32,
    #[field_offset(108usize)]
    pub field48_0x6c: f32,
    #[field_offset(112usize)]
    pub field49_0x70: f32,
}
#[ensure_layout(size = 36usize)]
pub struct EnvironmentColorCorrect {}
#[ensure_layout(size = 40usize)]
pub struct EnvironmentTonemap {
    #[field_offset(0usize)]
    pub active: bool,
    #[field_offset(4usize)]
    pub params: DeviceTonemapParams,
}
#[ensure_layout(size = 80usize)]
pub struct Environment460 {}
#[ensure_layout(size = 1936usize)]
pub struct Environment {
    /*
    #[field_offset(0usize)]
    pub light: [EnvironmentLight; 3usize],
    #[field_offset(372usize)]
    pub independenceLight: EnvironmentLight,
    */
    #[field_offset(0)] lights: [u8; 496],
    #[field_offset(496usize)]
    pub camera: EnvironmentCamera,
    #[field_offset(508usize)]
    pub fog: EnvironmentFog,
    #[field_offset(664usize)]
    pub fog1: EnvironmentFog1,
    #[field_offset(744usize)]
    pub fog2: EnvironmentFog2,
    #[field_offset(816usize)]
    pub hdr: EnvironmentHDR,
    #[field_offset(892usize)]
    pub tonemap: EnvironmentTonemap,
    #[field_offset(932usize)]
    pub dof: EnvironmentColorCorrectNew,
    #[field_offset(968usize)]
    pub shadow: EnvironmentShadow,
    #[field_offset(1084usize)]
    pub correct_old: EnvironmentColorCorrect,
    #[field_offset(1120usize)]
    pub Field460: Environment460,
    #[field_offset(1200usize)]
    pub Field4B0: Environment4B0,
    #[field_offset(1208usize)]
    pub Field4B8: Environment4B8,
    #[field_offset(1244usize)]
    pub Field4DC: Environment4DC,
    #[field_offset(1252usize)]
    pub temperare: EnvironmentTemperare,
    #[field_offset(1332usize)]
    pub clouds: EnvironmentClouds,
    #[field_offset(1408usize)]
    pub Field580: Environment580,
    #[field_offset(1424usize)]
    pub infiniteOcean: EnvironmentInfiniteOcean,
    #[field_offset(1480usize)]
    pub infiniteOceanMesh: *mut Mesh,
    #[field_offset(1520usize)]
    pub SceneAmbientSky: RGBAFloat,
    #[field_offset(1536usize)]
    pub SceneSkyColor: RGBAFloat,
    #[field_offset(1552usize)]
    pub SceneENVColor: RGBAFloat,
    #[field_offset(1568usize)]
    pub SceneENVColorToon: RGBAFloat,
    #[field_offset(1584usize)]
    pub field36_0x630: f32,
    #[field_offset(1588usize)]
    pub field37_0x634: f32,
    #[field_offset(1592usize)]
    pub WaterDeepColorNoSkyboxInfluence: RGBAFloat,
    #[field_offset(1608usize)]
    pub WaterScatterColorNoSkyboxInfluence: RGBAFloat,
    #[field_offset(1624usize)]
    pub WaterReflectionColorNoSkyboxInfluence: RGBAFloat,
    #[field_offset(1640usize)]
    pub WaterFoamColorNoSkyboxInfluence: RGBAFloat,
    #[field_offset(1656usize)]
    pub WaterDeepColorSkyboxInfluenced: RGBAFloat,
    #[field_offset(1672usize)]
    pub WaterScatterColorSkyboxInfluenced: RGBAFloat,
    #[field_offset(1688usize)]
    pub WaterReflectionColorSkyboxInfluenced: RGBAFloat,
    #[field_offset(1704usize)]
    pub WaterFoamColorSkyboxInfluenced: RGBAFloat,
    #[field_offset(1720usize)]
    pub SceneSkyFogColor: RGBAFloat,
    #[field_offset(1744usize)]
    pub flags: u32,
    #[field_offset(1752usize)]
    pub hdrFilename: Name,
    #[field_offset(1776usize)]
    pub iblFilename: Name,
    #[field_offset(1800usize)]
    pub lutFilename: Name,
    #[field_offset(1824usize)]
    pub envToonFilename: Name,
    #[field_offset(1848usize)]
    pub hdrTex: *mut Texture,
    #[field_offset(1856usize)]
    pub iblTex: *mut Texture,
    #[field_offset(1864usize)]
    pub lutTex: *mut Texture,
    #[field_offset(1872usize)]
    pub envToonTex: *mut Texture,
    #[field_offset(1888usize)]
    pub skyboxFilename: Name,
    #[field_offset(1912usize)]
    pub skyboxMesh: *mut Mesh,
}

#[ensure_layout(size = 8usize)]
pub struct Environment4B0 {}
#[ensure_layout(size = 156usize)]
pub struct EnvironmentFog {
    #[field_offset(0usize)]
    pub field0_0x0: bool,
    #[field_offset(4usize)]
    pub params: DeviceFogParams,
}

#[ensure_layout(size = 152usize)]
pub struct DeviceFogParams {
    #[field_offset(0usize)]
    pub fogParametersR: f32,
    #[field_offset(4usize)]
    pub fogParametersG: f32,
    #[field_offset(8usize)]
    pub fogColorOpacity: f32,
    #[field_offset(12usize)]
    pub fogParametersA: f32,
    #[field_offset(16usize)]
    pub Field214: f32,
    #[field_offset(20usize)]
    pub fogColor: RGBFloat,
    #[field_offset(32usize)]
    pub fogColorMultiplyFactor: f32,
    #[field_offset(36usize)]
    pub field7_0x24: f32,
    #[field_offset(40usize)]
    pub dirInscatColorA: f32,
    #[field_offset(44usize)]
    pub dirInscatStartDistance: f32,
    #[field_offset(48usize)]
    pub dirInscatColor: RGBFloat,
    #[field_offset(60usize)]
    pub field11_0x3c: f32,
    #[field_offset(64usize)]
    pub field12_0x40: f32,
    #[field_offset(68usize)]
    pub field13_0x44: f32,
    #[field_offset(72usize)]
    pub field14_0x48: f32,
    #[field_offset(76usize)]
    pub field15_0x4c: f32,
    #[field_offset(80usize)]
    pub field16_0x50: f32,
    #[field_offset(84usize)]
    pub field17_0x54: f32,
    #[field_offset(88usize)]
    pub field18_0x58: f32,
    #[field_offset(92usize)]
    pub field19_0x5c: f32,
    #[field_offset(96usize)]
    pub field20_0x60: f32,
    #[field_offset(100usize)]
    pub field21_0x64: f32,
    #[field_offset(104usize)]
    pub field22_0x68: f32,
    #[field_offset(108usize)]
    pub field23_0x6c: f32,
    #[field_offset(112usize)]
    pub field24_0x70: f32,
    #[field_offset(116usize)]
    pub field25_0x74: f32,
    #[field_offset(120usize)]
    pub field26_0x78: f32,
    #[field_offset(124usize)]
    pub field27_0x7c: f32,
    #[field_offset(128usize)]
    pub field28_0x80: f32,
    #[field_offset(132usize)]
    pub field29_0x84: f32,
    #[field_offset(136usize)]
    pub field30_0x88: f32,
    #[field_offset(140usize)]
    pub field31_0x8c: f32,
    #[field_offset(144usize)]
    pub field32_0x90: f32,
    #[field_offset(148usize)]
    pub field33_0x94: f32,
}

#[ensure_layout(size = 16usize)]
pub struct Environment580 {}
#[ensure_layout(size = 36usize)]
pub struct Environment4B8 {}

#[ensure_layout(size = 72usize)]
pub struct DeviceHDRParams {
    #[field_offset(0usize)]
    pub field0_0x0: u8,
    #[field_offset(1usize)]
    pub field1_0x1: u8,
    #[field_offset(2usize)]
    pub field2_0x2: u8,
    #[field_offset(3usize)]
    pub field3_0x3: u8,
    #[field_offset(4usize)]
    pub field4_0x4: f32,
    #[field_offset(8usize)]
    pub field5_0x8: f32,
    #[field_offset(12usize)]
    pub field6_0xc: f32,
    #[field_offset(16usize)]
    pub field7_0x10: f32,
    #[field_offset(20usize)]
    pub field8_0x14: f32,
    #[field_offset(24usize)]
    pub field9_0x18: f32,
    #[field_offset(28usize)]
    pub field10_0x1c: f32,
    #[field_offset(32usize)]
    pub field11_0x20: f32,
    #[field_offset(36usize)]
    pub field12_0x24: f32,
    #[field_offset(40usize)]
    pub field13_0x28: f32,
    #[field_offset(44usize)]
    pub field14_0x2c: f32,
    #[field_offset(48usize)]
    pub field15_0x30: f32,
    #[field_offset(52usize)]
    pub field16_0x34: f32,
    #[field_offset(56usize)]
    pub field17_0x38: f32,
    #[field_offset(60usize)]
    pub field18_0x3c: f32,
    #[field_offset(64usize)]
    pub field19_0x40: f32,
    #[field_offset(68usize)]
    pub field20_0x44: f32,
}
#[ensure_layout(size = 36usize)]
pub struct DeviceTonemapParams {
    #[field_offset(0usize)]
    pub FilmSlope: f32,
    #[field_offset(4usize)]
    pub FilmToe: f32,
    #[field_offset(8usize)]
    pub FilmShoulder: f32,
    #[field_offset(12usize)]
    pub FilmBlackClip: f32,
    #[field_offset(16usize)]
    pub FilmWhiteClip: f32,
    #[field_offset(20usize)]
    pub FlimAlpha: f32,
    #[field_offset(24usize)]
    pub field6_0x18: f32,
    #[field_offset(28usize)]
    pub field7_0x1c: f32,
    #[field_offset(32usize)]
    pub field8_0x20: f32,
}

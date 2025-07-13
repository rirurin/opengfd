use glam::{ Vec2, Vec3, Vec3A, Vec4, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum BufferFieldType {
    Float = 0,
    Float2,
    Float3,
    Float4,
    Matrix3,
    Matrix4
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferFieldHint {
    field0: u64,
    type_:  BufferFieldType,
}

impl BufferFieldHint {
    pub const fn new(field0: u64, type_: BufferFieldType) -> Self {
        Self { field0, type_ }
    }
}

pub trait BufferFieldRustType {
    fn get_type() -> BufferFieldType;
}
impl BufferFieldRustType for f32 {
    fn get_type() -> BufferFieldType { BufferFieldType::Float }
}
impl BufferFieldRustType for Vec2 {
    fn get_type() -> BufferFieldType { BufferFieldType::Float2 }
}
impl BufferFieldRustType for Vec3 {
    fn get_type() -> BufferFieldType { BufferFieldType::Float3 }
}
impl BufferFieldRustType for Vec3A {
    fn get_type() -> BufferFieldType { BufferFieldType::Float3 }
}
impl BufferFieldRustType for Vec4 {
    fn get_type() -> BufferFieldType { BufferFieldType::Float4 }
}
impl BufferFieldRustType for [Vec4; 3] { // matrix3x4
    fn get_type() -> BufferFieldType { BufferFieldType::Matrix3 }
}
impl BufferFieldRustType for Mat4 { // matrix4x4
    fn get_type() -> BufferFieldType { BufferFieldType::Matrix4 }
}

// In InitializeNGRBasicBuffers (see Prologue Demo disasm)

pub static GFD_VSCONST_SYSTEM: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // zeroVec
    BufferFieldHint::new(0, BufferFieldType::Float4), // constants
    BufferFieldHint::new(0, BufferFieldType::Float2), // resolutionRev
    BufferFieldHint::new(0, BufferFieldType::Float2),
];

pub static GFD_VSCONST_TRANSFORM: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxLocalToWorld
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxLocalToWorldViewProj
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxLocalToWorldViewProjPrev
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxModelToLocal
];

pub static GFD_VSCONST_VIEWPROJ: [BufferFieldHint; 5] = [
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxViewProj
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxView
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxInvView
    BufferFieldHint::new(0, BufferFieldType::Float3), // eyePosition
    BufferFieldHint::new(0, BufferFieldType::Float), // fovy
];

pub static GFD_VSCONST_COLORS: [BufferFieldHint; 1] = [
    BufferFieldHint::new(0, BufferFieldType::Float4) // constantColor
];

pub static GFD_VSCONST_UVX_TRANSFORM: [BufferFieldHint; 1] = [
    BufferFieldHint::new(0, BufferFieldType::Matrix4) // mtxUVXTransform
];

pub static GFD_VSCONST_LIGHT_VEC: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float3), // lightDirection
    BufferFieldHint::new(0, BufferFieldType::Float), // light_reserved_0
    BufferFieldHint::new(0, BufferFieldType::Float3), // lightInvDirection
    BufferFieldHint::new(0, BufferFieldType::Float), // light_reserved_1
];

pub static GFD_PSCONST_LIGHT_PS: [BufferFieldHint; 7] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // lightColor
    BufferFieldHint::new(0, BufferFieldType::Float3), // lightDirection
    BufferFieldHint::new(0, BufferFieldType::Float), // lightSpecularIntensity
    BufferFieldHint::new(0, BufferFieldType::Float3), // lightInvDirection
    BufferFieldHint::new(0, BufferFieldType::Float), // light_reserved_1
    BufferFieldHint::new(0, BufferFieldType::Float3), // lightAmbient
    BufferFieldHint::new(0, BufferFieldType::Float) // lightShadowAlpha
];

pub static GFD_PSCONST_SYSTEM: [BufferFieldHint; 7] = [
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // resolution
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // resolutionRev
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxView
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxInvView
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxProj
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxInvProj
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // invProjParams
];

pub static GFD_PSCONST_ENV_COLORS: [BufferFieldHint; 12] = [
    BufferFieldHint::new(0, BufferFieldType::Float3), // sceneAmbientPBR
    BufferFieldHint::new(0, BufferFieldType::Float), // envMapMipLevels
    BufferFieldHint::new(0, BufferFieldType::Float3), // sceneAmbientToon
    BufferFieldHint::new(0, BufferFieldType::Float), // bloomAdjustToon
    BufferFieldHint::new(0, BufferFieldType::Float4), // sceneSkyColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // sceneEnvColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // sceneEnvColorToon
    BufferFieldHint::new(0, BufferFieldType::Float4), // clearColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // ENVandIBLMapRotateY
    BufferFieldHint::new(0, BufferFieldType::Float3), // sceneAmbientSky
    BufferFieldHint::new(0, BufferFieldType::Float), // sceneToonShadowAlpha
    BufferFieldHint::new(0, BufferFieldType::Float4), // sceneSkyFogColor
];

pub static GFD_VSCONST_VAT: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // boundingBoxMax
    BufferFieldHint::new(0, BufferFieldType::Float), // boundingBoxMin
    BufferFieldHint::new(0, BufferFieldType::Float), // time
    BufferFieldHint::new(0, BufferFieldType::Float), // totalFrame
];

pub static GFD_PSCONST_ALPHATEST: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // atest_func
    BufferFieldHint::new(0, BufferFieldType::Float), // atest_ref
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

// In FUN_1411217a0 (see Prologue Demo disasm)

pub static GFD_VSCONST_SHADOW: [BufferFieldHint; 3] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // mtxLightViewProj[0]
    BufferFieldHint::new(0, BufferFieldType::Float), // mtxLightViewProj[1]
    BufferFieldHint::new(0, BufferFieldType::Float), // mtxLightViewProj[2]
];

pub static GFD_PSCONST_FOG: [BufferFieldHint; 22] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogParameters
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogColorParameter
    BufferFieldHint::new(0, BufferFieldType::Float4), // dirInscatColor
    BufferFieldHint::new(0, BufferFieldType::Float), // dirInscatStartDistance
    BufferFieldHint::new(0, BufferFieldType::Float), // fogHeightParameterX
    BufferFieldHint::new(0, BufferFieldType::Float), // fogHeightParameterY
    BufferFieldHint::new(0, BufferFieldType::Float), // fogExponentialHeightYRate
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogHeightColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogColorParameter_sky
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogHeightColor_sky
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogDistanceParameter
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogDistanceColor
    BufferFieldHint::new(0, BufferFieldType::Float), // fogHeightParameterX_sky
    BufferFieldHint::new(0, BufferFieldType::Float), // fogHeightParameterY_sky
    BufferFieldHint::new(0, BufferFieldType::Float), // fog_reserved_1
    BufferFieldHint::new(0, BufferFieldType::Float), // fog_reserved_2
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogColorParameter_toon
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogDistanceColor_toon
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogHeightColor_toon
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogParameters_sky
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogDistanceParameter_sky
    BufferFieldHint::new(0, BufferFieldType::Float4), // fogDistanceColor_sky
];

pub static GFD_PSCONST_SHADOW: [BufferFieldHint; 7] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowDimmer
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowBiasPBR
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowBiasOther
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowTexShift
    BufferFieldHint::new(0, BufferFieldType::Float4), // shadowSplit
    BufferFieldHint::new(0, BufferFieldType::Float3), // shadowColor
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowSlopeScaledBias
];

// In gfdMaterialCreateConstantBuffer (see Prologue Demo disasm)

pub static GFD_PSCONST_MATERIAL_TYPE0: [BufferFieldHint; 9] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matEmissive
    BufferFieldHint::new(0, BufferFieldType::Float), // matRoughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matMetallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matMultiAlpha
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE1: [BufferFieldHint; 8] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matAmbient
    BufferFieldHint::new(0, BufferFieldType::Float4), // matDiffuse
    BufferFieldHint::new(0, BufferFieldType::Float4), // matSpecular
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEmissive
    BufferFieldHint::new(0, BufferFieldType::Float), // matReflectivity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE2: [BufferFieldHint; 22] = [
    BufferFieldHint::new(0, BufferFieldType::Float3), // matSpecularColor
    BufferFieldHint::new(0, BufferFieldType::Float), // outlineID
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matShadowColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEdgeColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEmissiveColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matSpecularThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // matSpecularPower
    BufferFieldHint::new(0, BufferFieldType::Float), // matMetallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeRemoveYAxisFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // ssssID
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float), // matBrightness
    BufferFieldHint::new(0, BufferFieldType::Float), // matRoughness
    BufferFieldHint::new(0, BufferFieldType::Float), // fittingTile
    BufferFieldHint::new(0, BufferFieldType::Float), // multiFittingTile
];

pub static GFD_PSCONST_MATERIAL_TOON_OUTLINE: [BufferFieldHint; 7] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // outlineBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float), // _reserve
    BufferFieldHint::new(0, BufferFieldType::Float), // outlineID
    BufferFieldHint::new(0, BufferFieldType::Float4), // outlineColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matShadowColor
];

pub static GFD_VSCONST_TOON_OUTLINE: [BufferFieldHint; 3] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // outlineThickness
    BufferFieldHint::new(0, BufferFieldType::Float), // outlineThinMax
    BufferFieldHint::new(0, BufferFieldType::Float2), // outlineThinFade
];

pub static GFD_PSCONST_MATERIAL_TYPE4: [BufferFieldHint; 14] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEmissiveColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // distortionPower
    BufferFieldHint::new(0, BufferFieldType::Float), // dissolveThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // rimTransPower
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float), // fittingTile
    BufferFieldHint::new(0, BufferFieldType::Float), // multiFittingTile
    BufferFieldHint::new(0, BufferFieldType::Float), // matBrightness
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCOSNT_MATERIAL_OCEAN: [BufferFieldHint; 15] = [
    BufferFieldHint::new(0, BufferFieldType::Float2), // TexShift
    BufferFieldHint::new(0, BufferFieldType::Float), // TCScale
    BufferFieldHint::new(0, BufferFieldType::Float), // OceanDepthScale
    BufferFieldHint::new(0, BufferFieldType::Float), // DisturbanceCameraScale
    BufferFieldHint::new(0, BufferFieldType::Float), // DisturbanceDepthScale
    BufferFieldHint::new(0, BufferFieldType::Float), // ScatteringCameraScale
    BufferFieldHint::new(0, BufferFieldType::Float), // DisturbanceTolerance
    BufferFieldHint::new(0, BufferFieldType::Float), // FoamDistance
    BufferFieldHint::new(0, BufferFieldType::Float), // CausticsTolerance
    BufferFieldHint::new(0, BufferFieldType::Float), // triPlanarScale
    BufferFieldHint::new(0, BufferFieldType::Float), // texAnimationSpeed
    BufferFieldHint::new(0, BufferFieldType::Float4), // waterDeepColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // waterScatterColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // waterReflectionColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // waterFoamColor
];

pub static GFD_VSCONST_OCEAN: [BufferFieldHint; 8] = [
    BufferFieldHint::new(0, BufferFieldType::Matrix4), // mtxInvLocalToWorld
    BufferFieldHint::new(0, BufferFieldType::Float2), // TexShift
    BufferFieldHint::new(0, BufferFieldType::Float), // TCScale
    BufferFieldHint::new(0, BufferFieldType::Float), // WaterWidthScale
    BufferFieldHint::new(0, BufferFieldType::Float), // WaterHeightScale
    BufferFieldHint::new(0, BufferFieldType::Float), // VertexPitch
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE6: [BufferFieldHint; 18] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matLayer0BaseColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer0Emissive
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer0Roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer0Metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer0BloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float4), // matLayer1BaseColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer1Emissive
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer1Roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer1Metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer1BloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // triPlanarScaleLayer0
    BufferFieldHint::new(0, BufferFieldType::Float), // triPlanarScaleLayer1
    BufferFieldHint::new(0, BufferFieldType::Float), // triPlanarScaleBlend
    BufferFieldHint::new(0, BufferFieldType::Float), // blendTextureIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // constantAlpha
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE7: [BufferFieldHint; 12] = [
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer0Roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer0Metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer1Roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer1Metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer2Roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer2Metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer3Roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matLayer3Metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // triPlanarScale
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_VSCONST_MATERIAL_TYPE7: [BufferFieldHint; 8] = [
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer0TileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer0TileOffset
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer1TileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer1TileOffset
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer2TileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer2TileOffset
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer3TileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // matLayer3TileOffset
];

pub static GFD_PSCONST_MATERIAL_TYPE8: [BufferFieldHint; 9] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matEmissive
    BufferFieldHint::new(0, BufferFieldType::Float), // matRoughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matMetallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE9: [BufferFieldHint; 18] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matShadowColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEdgeColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEmissiveColor
    BufferFieldHint::new(0, BufferFieldType::Float3), // matSpecularColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matSpecularThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // matSpecularPower
    BufferFieldHint::new(0, BufferFieldType::Float), // matMetallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE10: [BufferFieldHint; 5] = [
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matMultiAlpha
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE12: [BufferFieldHint; 22] = [
    BufferFieldHint::new(0, BufferFieldType::Float3), // matSpecularColor
    BufferFieldHint::new(0, BufferFieldType::Float), // outlineID
    BufferFieldHint::new(0, BufferFieldType::Float4), // matBaseColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matShadowColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEdgeColor
    BufferFieldHint::new(0, BufferFieldType::Float4), // matEmissiveColor
    BufferFieldHint::new(0, BufferFieldType::Float), // matSpecularThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // matSpecularPower
    BufferFieldHint::new(0, BufferFieldType::Float), // matRampAlpha
    BufferFieldHint::new(0, BufferFieldType::Float), // matMetallic
    BufferFieldHint::new(0, BufferFieldType::Float), // matRoughness
    BufferFieldHint::new(0, BufferFieldType::Float), // matBloomIntensity
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // edgeRemoveYAxisFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowThreshold
    BufferFieldHint::new(0, BufferFieldType::Float), // shadowFactor
    BufferFieldHint::new(0, BufferFieldType::Float), // ssssID
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
    BufferFieldHint::new(0, BufferFieldType::Float), // matBrightness
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_MATERIAL_TYPE14: [BufferFieldHint; 1] = [
    BufferFieldHint::new(0, BufferFieldType::Float4) // matBaseColor
];

pub static GFD_PSCONST_MATERIAL_TYPE15: [BufferFieldHint; 116] = [
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[0].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[0].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[0].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[0].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[0]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[0]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[0].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[1].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[1].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[1].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[1].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[1]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[1]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[1].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[2].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[2].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[2].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[2].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[2]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[2]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[2].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[3].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[3].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[3].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[3].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[3]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[3]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[3].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[4].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[4].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[4].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[4].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[4]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[4]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[4].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[5].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[5].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[5].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[5].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[5]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[5]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[5].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[6].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[6].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[6].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[6].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[6]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[6]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[6].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[7].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[7].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[7].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[7].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[7]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[7]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[7].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[8].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[8].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[8].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[8].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[8]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[8]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[8].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[9].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[9].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[9].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[9].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[9]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[9]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[9].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[10].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[10].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[10].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[10].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[10]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[10]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[10].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[11].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[11].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[11].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[11].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[11]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[11]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[11].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[12].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[12].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[12].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[12].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[12]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[12]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[12].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[13].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[13].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[13].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[13].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[13]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[13]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[13].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[14].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[14].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[14].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[14].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[14]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[14]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[14].color
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[15].tileSize
    BufferFieldHint::new(0, BufferFieldType::Float2), // layers[15].tileOffset
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[15].roughness
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[15].metallic
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[15]._reserve_0
    BufferFieldHint::new(0, BufferFieldType::Float), // layers[15]._reserve_1
    BufferFieldHint::new(0, BufferFieldType::Float4), // layers[15].color
    BufferFieldHint::new(0, BufferFieldType::Float), // layerCount
    BufferFieldHint::new(0, BufferFieldType::Float), // triPlanarScale
    BufferFieldHint::new(0, BufferFieldType::Float), // atestRef
    BufferFieldHint::new(0, BufferFieldType::Float), // lerpBlendRate
];

// In gfdCreateStaticConstantBuffers (see Prologue Demo disasm)

pub static REG_12_BUF_598: [BufferFieldHint; 12] = [
    BufferFieldHint::new(0, BufferFieldType::Float3),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float3),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float3),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float3),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_638: [BufferFieldHint; 5] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_640: [BufferFieldHint; 2] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
];

pub static REG_11_BUF_648: [BufferFieldHint; 2] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
];

pub static REG_11_BUF_650: [BufferFieldHint; 3] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
];

pub static REG_11_BUF_658: [BufferFieldHint; 1] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
];

pub static REG_11_BUF_660: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_668: [BufferFieldHint; 47] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
    BufferFieldHint::new(0, BufferFieldType::Matrix4),
];

pub static REG_11_BUF_680: [BufferFieldHint; 3] = [
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_688: [BufferFieldHint; 6] = [
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_4C0: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static GFD_PSCONST_METABALL: [BufferFieldHint; 37] = [
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float4),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_690: [BufferFieldHint; 3] = [
    BufferFieldHint::new(0, BufferFieldType::Float2),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

pub static REG_11_BUF_6A8: [BufferFieldHint; 4] = [
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
    BufferFieldHint::new(0, BufferFieldType::Float),
];

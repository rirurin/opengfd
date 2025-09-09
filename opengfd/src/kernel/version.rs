#[allow(nonstandard_style)]
pub enum GfdVersion {
    // GFD v1
    NodeContainsProperties = 0x1060001,
    NameContainsHash = 0x1080001,
    MaterialBlendAddMultiplyMode = 0x1080004,
    MaterialHasHighlightMapMode = 0x108011c,
    EnvFogMode = 0x1102000,
    GeometryAddGeomType = 0x1103021,
    MaterialBlendAsU8 = 0x1103041, // also adds mat constant field
    MaterialFlagsAllowConstantColor = 0x1104000,
    MaterialAllowUVTransform = 0x1104001,
    EnvAddHeightFog = 0x1104021,
    CameraAddRoll = 0x1104061,
    MaterialExtensionToonV2 = 0x1104221,
    MaterialExtensionToonV2_Flag3 = 0x1104261,
    MaterialExtensionToonV3 = 0x1104501,
    AnimationHasFlags = 0x1104111,
    EnvColorCorrectionScreenBurn = 0x1104141,
    MaterialExtensionEdgeFlag1 = 0x1104181,
    LightAddFlags = 0x1104191,
    MaterialExtensionEdgeFlag2 = 0x1104211,
    EnvAddIndependenceLight = 0x1104241,
    EnvLightingStarParameters = 0x1104301,
    MaterialExtensionEdgeFlag3 = 0x1104401,
    MaterialAddSecondFlags = 0x1104801,
    AnmationPackHasFlags = 0x1104951,
    PERSONA4DANCING = 0x1105030, // xrd757_p4d
    EplAddP5RField80 = 0x1105061,
    PERSONA5 = 0x1105070, // xrd664
    P5DP3D = 0x1105090, // xrd757
    PERSONA5ROYAL = 0x1105100, // xrd744 and CFB
    // GFD v2
    MaterialUseParameterSet = 0x2000000,
    MaterialParameter0AddMultiAlpha = 0x2000004,
    EnvColorCorrectionScreenHSL = 0x2000005,
    MaterialParameterToonSetP12 = 0x2010000,
    MaterialDiffusivitySSAONotRequired = 0x2010001,
    EnvAddTemperareSection = 0x2020001,
    MaterialParameterAddBloomIntensity = 0x2030001,
    EnvAddCloudsSection = 0x2060000,
    MaterialParameterToonAddSpecularThreshold = 0x2090001,
    EnvColorCorrectionFieldModelSpecific = 0x2092001,
    MaterialParameterToonAddEdgeRemoveYAxisFactor = 0x2094001,
    EnvLightMetaphorField0 = 0x2098001,
    EnvAddSSAOSection = 0x2099001,
    EnvAddToneMapSection = 0x2102001,
    EnvToneMapAddFilmAlpha = 0x2103001,
    EnvFieldLightMetaphorField1 = 0x2104001,
    MaterialParameterType11AddField = 0x2108001,
    MaterialParameterToonAddP17 = 0x2109501,
    MaterialParameterToonAddP20 = 0x2109601,
    MaterialParameterMetalAddSpecular = 0x2109701,
    EnvFogColorMultiplyFactor = 0x2101001,
    EnvCloudsAddCloudColor = 0x2110011,
    MaterialParameterLayerExtraFields = 0x2110021,
    MaterialParameter0AddFlags = 0x2110041,
    CameraAddUnkMetaphor = 0x2110051,
    MaterialParameterMetalAddShadow = 0x2110071,
    MaterialParameterSkyAddP2 = 0x2110091,
    MaterialParameterSkyAddFlags = 0x2110101,
    EnvFogColorParameterSky = 0x2110111,
    EnvFogHeightColorSky = 0x2110120,
    MaterialFieldAddExtraFloat = 0x2110140,
    MaterialFieldAddFlags = 0x2110141,
    MaterialAddField6C = 0x2110161,
    EnvAddLUTRecolorWeighting = 0x2110173,
    EnvHeightFogSkyStartEnd = 0x2110174,
    EnvFogColorParameterToon = 0x2110175, // env height color toon, fog dist color toon
    MaterialParameterWaterAddTextureSpeed = 0x2110182,
    MaterialParameter4AddBloomIntensity = 0x2110184,
    EnvAddEnvironmentColorsSection = 0x2110185,
    MaterialParameterWaterAddFlags = 0x2110188, // env fog parameters sky
    LightAddAlpha = 0x2110191,
    EnvFogDistanceColorSky = 0x2110194,
    MaterialParameterToonAddMatRoughness = 0x2110198,
    EnvLightingAdaptedLumAdjust = 0x2110201,
    EnvLightingPbrIntensity = 0x2110202,
    LightAddToonInfluence = 0x2110203,
    MaterialParameterToonAddFittingTile = 0x2110204, // add P4_7
    EnvAddInfiniteOcean_LUTRecolorParams = 0x2110205,
    GeometryAddStrideType = 0x2110206,
    EplAddMetaphorField60 = 0x2110208,
    MaterialParameterToonAddMultiFittingTile = 0x2110210,
    GeometryAddMetaphorUnkVertexWeightRelated = 0x2110213,
    MaterialParameterDistortAddMultiFittingTile = 0x2110217,
    MaterialParameterDistortAddP8 = 0x2110218,
    EnvFogExponentialHeightYRate = 0x2110219,
    METAPHORREFANTAZIO = 0x02110221, // xrd759
    // OpenGFD Extensions for Metaphor
    MaterialDefineWaterColor = 0x2110222
}

impl GfdVersion {
    pub const fn current() -> Self {
        #[cfg(feature = "v1-core")]
        { GfdVersion::PERSONA5ROYAL }
        #[cfg(feature = "v2-core")]
        { GfdVersion::METAPHORREFANTAZIO }
    }
}
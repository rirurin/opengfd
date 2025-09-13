use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GraphicsFlags : u32 {
        const ShadowCaster = 1 << 0;
        const Lighting = 1 << 1;
        const Flag2 = 1 << 2;
        const Fog = 1 << 3;
        const DepthOfField = 1 << 4;
        const ColorGrading = 1 << 5;
        const CharacterOutline = 1 << 6;
        const HeightFog = 1 << 7;
        const Physics = 1 << 8;
        const Physics2 = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const HasInfiniteOcean = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const HasTemperare = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}
//! We have GFD at home
//! GFD at home:

#[cfg(all(feature = "v1-core", feature = "v2-core"))]
compile_error!("v1-core and v2-core are mutually exclusive!");

#[cfg(all(feature = "adapter-hedge", feature = "adapter-ngr"))]
compile_error!("adapter-hedge and adapter-ngr are mutually exclusive!");

pub mod ai {

}
pub mod anim {
    pub mod anim_controller;
    pub mod anim_effector;
    pub mod animation;
    pub mod biped_ik;
    pub mod key;
    pub mod timeline;
}
pub mod debug {

}
pub mod device {
    pub mod hedge {

    }
    pub mod ngr {

    }
}
pub mod effect {
    pub mod behavior;
    pub mod camera;
    pub mod directional_particle;
    pub mod helper;
    pub mod light;
    pub mod mesh;
    pub mod misc;
    pub mod object_particle;
    pub mod parts;
    pub mod particle;
    pub mod polygon_board;
    pub mod polygon_circle;
    pub mod polygon_flash;
    pub mod polygon_glitter;
    pub mod polygon_thunder;
    pub mod polygon_track;
    pub mod polygon_wind;
    pub mod post_effect;
    pub mod resources;
}
pub mod fw {

}
pub mod graphics {
    pub mod cull;
    pub mod curve;
    pub mod material;
    pub mod resources;
    pub mod shader {
        #[cfg(feature = "v1-core")]
        #[path = "flag_xrd744.rs"]
        pub mod flag;
        #[cfg(feature = "v2-core")]
        #[path = "flag_xrd759.rs"]
        pub mod flag;
        pub mod shader;
    }
    pub mod quake;
    pub mod skin;
    pub mod texture;
}
pub mod kernel {
    pub mod asset;
}
pub mod object {
    #[path = "object_base.rs"]
    pub mod object;
    pub mod mesh;
    pub mod node;
    pub mod geometry;
    pub mod camera;
    pub mod light;
    pub mod epl;
    pub mod morph;
}
pub mod utility {
    pub mod item_array;
    pub mod math;
    pub mod misc;
    #[cfg(target_os = "windows")]
    #[path = "mutex_win32.rs"]
    pub mod mutex;
    #[cfg(not(target_os = "windows"))]
    #[path = "mutex_generic.rs"]
    pub mod mutex;
    pub mod name;
    pub mod property;
    pub mod reference;
    pub mod stream;
}

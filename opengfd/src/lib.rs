//! We have GFD at home
//! GFD at home:

#[cfg(all(feature = "v1-core", feature = "v2-core"))]
compile_error!("v1-core and v2-core are mutually exclusive!");

#[cfg(all(feature = "adapter-hedge", feature = "adapter-ngr"))]
compile_error!("adapter-hedge and adapter-ngr are mutually exclusive!");

// LIBRARY STRUCTURE

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
    pub mod perf_meter;
}
pub mod device {
    #[cfg(feature = "adapter-hedge")]
    pub mod hedge {

    }
    #[cfg(feature = "adapter-ngr")]
    pub mod ngr {
        pub mod allocator;
        pub mod hint;
        #[cfg(target_os = "windows")]
        #[path = "win32"]
        pub mod platform {
            pub mod allocator;
        }
        pub mod renderer { 
            pub mod blend;
            pub mod bytecode;
            pub mod cbuffer;
            #[cfg(target_os = "windows")]
            #[path = "d3d"]
            pub mod platform {
                pub mod d3d;
                pub mod state;
            }
            pub mod hint;
            pub mod ps;
            pub mod render;
            pub mod pkt;
            pub mod shader;
            pub mod set_texture;
            pub mod state;
            pub mod vs;
        } 
        pub mod structures;
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
pub mod globals;
pub mod graphics {
    pub mod cull;
    pub mod curve;
    pub mod draw2d;
    pub mod draw3d;
    pub mod environment;
    pub mod material;
    pub mod render {
        pub mod cmd_buffer;
        pub mod render;
    }
    pub mod resources;
    pub mod render_ot;
    pub mod shader {
        #[cfg(feature = "v1-core")]
        pub mod attribute {
            pub mod edge_v1;
            pub mod outline_v1;
            pub mod shadow_edge_v1;
            pub mod toon_v1;
            pub mod water_v1;
        }
        #[cfg(feature = "v2-core")]
        pub mod attribute {
            pub mod lambert_v2;
            pub mod layered_v2;
            pub mod metal_v2;
            pub mod pbr_v2;
            pub mod shadow_v2;
            pub mod toon_v2;
            pub mod water_v2;
        }
        #[cfg(feature = "v1-core")]
        #[path = "flag_xrd744.rs"]
        pub mod flag;
        #[cfg(feature = "v2-core")]
        #[path = "flag_xrd759.rs"]
        pub mod flag;
        pub mod shader;
    }
    pub mod quake;
    pub mod scene;
    pub mod skin;
    pub mod texture;
}
pub mod kernel {
    pub mod allocator;
    pub mod asset;
    #[cfg(feature = "v1-core")]
    #[path = "global_xrd744.rs"]
    pub mod global;
    #[cfg(feature = "v2-core")]
    #[path = "global_xrd759.rs"]
    pub mod global;
    pub mod global_common;
    pub mod task;
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
pub mod tests {
    #![allow(dead_code)]
    use std::{
        error::Error,
        fmt::{ Debug, Display }
    };
    pub(crate) type TestReturn = Result<(), Box<dyn std::error::Error>>;
    pub(crate) struct OpengfdError(String);
    impl Error for OpengfdError { }
    impl Debug for OpengfdError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OpenGFD error: {}", self.0)
        }
    }
    impl Display for OpengfdError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OpenGFD error: {}", self.0)
        }
    }
    impl OpengfdError {
        pub fn new(t: String) -> Self { OpengfdError(t) }
        pub fn new_str<T: AsRef<str>>(t: T) -> Self { OpengfdError(t.as_ref().to_owned()) }
    }
}
pub mod tpl {
    pub mod file_manager;
    pub mod resource;
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
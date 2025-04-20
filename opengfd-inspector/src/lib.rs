pub mod globals;
pub mod imgui_hook;
pub mod panels {
    pub mod about;
    pub mod attachment {
        pub mod camera;
        pub mod epl;
        pub mod geometry;
        pub mod mesh;
        pub mod node;
        pub mod property;
    }
    pub mod graphics;
    pub mod scene_graph;
    pub mod scheduler;
}
pub mod state;
pub mod window;
//! We have GFD at home
//! GFD at home:
pub mod ai {

}
pub mod anim {

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
    
}
pub mod fw {

}
pub mod graphics {
    pub mod curve;
    pub mod attribute {
    }
}
pub mod utility {
    pub mod item_array;
    pub mod math;
    #[cfg(target_os = "windows")]
    #[path = "mutex_win32.rs"]
    pub mod mutex;
    #[cfg(not(target_os = "windows"))]
    #[path = "mutex_generic.rs"]
    pub mod mutex;
    pub mod name;
    pub mod stream;
}

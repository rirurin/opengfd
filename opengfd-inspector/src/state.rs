use riri_inspector_components::panel::InspectorPanel;
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{ 
        InitTask,
        Task,
        TaskFunctionReturn,
        UpdateTask 
    }
};

#[repr(C)]
#[derive(Debug, Default)]
pub struct Inspector {
    // Panels
    pub(crate) initialized: bool,
    pub(crate) panels: Vec<Box<dyn InspectorPanel>>
}
impl Inspector {
    fn new() -> Self {
        Self {
            initialized: false,
            panels: vec![
                Box::new(crate::panels::scheduler::SchedulerPanel::new()),
                Box::new(crate::panels::graphics::GraphicsPanel::new()),
                Box::new(crate::panels::scene_graph::SceneGraphPanel::new()),
                Box::new(crate::panels::about::AboutPanel::new()),
            ]
        }
    }
}

impl UpdateTask for Inspector {
    const NAME: &'static str = "Rirurin GFD Inspector";
    fn update(task: &mut Task<GfdAllocator, Self>, _delta: f32) 
            -> TaskFunctionReturn where Self: Sized {
        let ctx = task.get_work_data_mut().unwrap();
        if !ctx.initialized {
            crate::window::init();
            ctx.initialized = true;
        }
        TaskFunctionReturn::Continue
    }
    fn shutdown(_task: &mut Task<GfdAllocator, Self>) -> ()
    where Self: Sized {}
}
impl InitTask for Inspector {
    fn new() -> Self where Self: Sized {
        Self::new()
    }
}
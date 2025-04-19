use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{ 
        InitTask, 
        Task as GfdTask,
        TaskFunctionReturn,
        UpdateTask 
    }
};

// #[derive(Debug)]
#[repr(C)]
pub struct SampleFont {
    state: i32,
    font: usize,
    string: usize
}
impl InitTask for SampleFont {
    fn new() -> Self where Self: Sized {
        Self {
            state: 0,
            font: 0,
            string: 0
        }
    }
}
impl UpdateTask for SampleFont {
    const NAME: &'static str = "sample_font";
    fn update(task: &mut GfdTask<GfdAllocator, Self>, delta: f32)
        -> TaskFunctionReturn where Self: Sized {
        let fn_ptr = unsafe { std::mem::transmute::<usize, extern "C" fn(&mut GfdTask<GfdAllocator, Self>, f32) -> TaskFunctionReturn>(0x1408ac180) };
        // let fn_ptr = 0x1408ac180 as *const fn(&mut GfdTask<GfdAllocator, Self>, f32) -> TaskFunctionReturn;
        (fn_ptr)(task, delta)
        
    }
    fn shutdown(_task: &mut GfdTask<GfdAllocator, Self>) -> () where Self: Sized {
        
    }
}
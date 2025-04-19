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
pub struct HelloWorld {
    data: [u8; 0xb8]
}
impl InitTask for HelloWorld {
    fn new() -> Self where Self: Sized {
        Self {
            data: [0; 0xb8]
        }
    }
}
impl UpdateTask for HelloWorld {
    const NAME: &'static str = "hello_world_01";
    fn update(_task: &mut GfdTask<GfdAllocator, Self>, _delta: f32)
    -> TaskFunctionReturn where Self: Sized {
        TaskFunctionReturn::Continue
        // let fn_ptr = unsafe { std::mem::transmute::<usize, extern "C" fn(&mut GfdTask<GfdAllocator, Self>, f32) -> TaskFunctionReturn>(0x140935af0) };
        // (fn_ptr)(task, delta)
        
    }
    fn shutdown(_task: &mut GfdTask<GfdAllocator, Self>) -> () where Self: Sized {
        
    }
}
use allocator_api2::alloc::Allocator;
use crate::{
    kernel::{
        allocator::GfdAllocator,
        global::{ Global, GlobalFlags }
    },
    utility::name::Name
};
use std::{
    alloc::Layout,
    fmt::Display,
    ptr::NonNull
};
use riri_mod_tools_rt::logln;
// use bitflags::bitflags;
// use std::marker::PhantomPinned;

#[repr(C)]
#[derive(Debug)]
pub struct TaskLink<A = GfdAllocator>
where A: Allocator + Clone
{
    pub parent: *mut Task<A>,
    pub child: *mut Task<A>,
    pub sibling: *mut Task<A>,
}

#[repr(C)]
#[derive(Debug)]
pub struct TaskListNode<A = GfdAllocator>
where A: Allocator + Clone
{
    prev: *mut Task<A>,
    next: *mut Task<A>
}

#[repr(C)]
#[derive(Debug)]
pub struct TaskList<A = GfdAllocator>
where A: Allocator + Clone 
{
    head: *mut Task<A>,
    tail: *mut Task<A>
}

impl<A> TaskList<A>
where A: Allocator + Clone 
{
    pub fn get_head_ptr(&self) -> *mut Task<A> { self.head }
    pub fn get_tail_ptr(&self) -> *mut Task<A> { self.tail }
    pub fn set_head(&mut self, val: *mut Task<A>) {
        self.head = val;
    }
    pub fn set_tail(&mut self, val: *mut Task<A>) {
        self.tail = val;
    }
}

type UpdateFn<D> = fn(&'static mut Task<GfdAllocator, D>, f32) -> TaskFunctionReturn;
type RenderFn<D> = fn(&'static mut Task<GfdAllocator, D>) -> TaskFunctionReturn;
type EndFn<D> = fn(&'static mut Task<GfdAllocator, D>) -> ();

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskStatus {
    Begin = 1,
    Update = 2,
    Shutdown = 3
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskFunctionReturn {
    Continue = 0,
    Shutdown = u64::MAX
}

#[repr(C)]
#[derive(Debug)]
pub struct Task<A = GfdAllocator, D = u8>
where A: Allocator + Clone,
      D: 'static
{
    // pub status: i32,
    status: TaskStatus,
    flags: i32,
    update_prio: TaskPriority,
    render_prio: TaskPriority,
    name: Name<A>,
    start_timer: i32,
    shutdown_timer: i32,
    update_fn: Option<UpdateFn<D>>,
    render_fn: Option<RenderFn<D>>,
    end_fn: Option<EndFn<D>>,
    // pub update_fn: Option<fn(*mut Self, f32) -> u64>,
    // pub render_fn: Option<fn(*mut Self) -> u64>,
    // pub end_fn: Option<fn(*mut Self) -> ()>,
    main_work: Option<NonNull<D>>,
    // NOTE: This field isn't in P5R PS4
    #[cfg(feature = "adapter-hedge")]
    mouse_work: *mut u8,
    link: TaskLink<A>,
    idle: TaskListNode<A>,
    update: TaskListNode<A>,
    render: TaskListNode<A>,
    pad: TaskListNode<A>,
    detach: *mut Task<A>,
    uid: u64,
    // _pinned: PhantomPinned,
    _allocator: A,
}

impl<A, D> PartialEq<str> for Task<A, D>
where A: Allocator + Clone
{
    fn eq(&self, other: &str) -> bool {
        &self.name == other
    }
}

impl<A, D> PartialEq<u64> for Task<A, D>
where A: Allocator + Clone
{
    fn eq(&self, other: &u64) -> bool {
        self.uid == *other
    }
}

impl<A, D> Display for Task<A, D>
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_inner(true))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskPriority(u32);

impl Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = if self.0 == u32::MAX { "tool".to_owned() } 
        else if self.0 >= 0x10000000 { "system".to_owned() } 
        else { format!("{}", self.0) };
        write!(f, "{}", fmt)
    }
}

impl PartialEq<u32> for TaskPriority {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<usize> for TaskPriority {
    fn eq(&self, other: &usize) -> bool {
        self.0 as usize == *other
    }
}

impl<A, D> Task<A, D>
where A: Allocator + Clone
{

    fn get_layout() -> Layout {
        Layout::new::<D>()
    }

    fn get_update_function_address(&self) -> *const u8 {
        match self.update_fn {
            Some(v) => v as *const u8,
            None => std::ptr::null()
        }
    }

    fn get_render_function_address(&self) -> *const u8 {
        match self.render_fn {
            Some(v) => v as *const u8,
            None => std::ptr::null()
        }
    }

    fn get_end_function_address(&self) -> *const u8 {
        match self.end_fn {
            Some(v) => v as *const u8,
            None => std::ptr::null()
        }
    }

    fn get_main_work_address(&self) -> *const u8 {
        match self.main_work {
            Some(v) => v.as_ptr() as *const u8,
            None => std::ptr::null()
        }
    }

    pub fn get_main_work_ref(&self) -> Option<&D> {
        self.main_work.map(|v| unsafe { v.as_ref() })
    }

    pub fn get_main_work_mut(&mut self) -> Option<&mut D> {
        self.main_work.map(|mut v| unsafe { v.as_mut() })
    }

    fn fmt_inner(&self, update: bool) -> String {
        let prio = if update { self.update_prio } else { self.render_prio };
        format!(
            "- {} @ 0x{:x}: <prio: {}> <func: 0x{:x}, 0x{:x}, 0x{:x}> <data: 0x{:x}> <uid: {}>",
        self.get_name_native(), &raw const *self as usize, prio, self.get_update_function_address() as usize, 
        self.get_render_function_address() as usize, self.get_end_function_address() as usize, 
        self.get_main_work_address() as usize, self.uid)
    }

    fn fmt_recursive(&self, indent: usize, update: bool) -> String {
        let this_printf = self.fmt_inner(update);
        let indent_str = "\t".repeat(indent);
        let mut out = format!("{}{}", indent_str, this_printf);
        for ch in self.iter_link() {
            out.push_str("\n");
            out.push_str(&ch.fmt_recursive(indent + 1, update));
        }
        out
    }

    fn get_task_allocation() -> &'static mut Self {
        let glb = Global::get_gfd_global_mut();
        let task_allocator = glb.get_task_free_list_unchecked_mut();
        let ptr = task_allocator.add() as *mut Self;
        unsafe { std::ptr::write_bytes(ptr, 0, size_of::<Self>()) }
        unsafe { &mut *ptr }
    }
    fn get_uid() -> u64 {
        let glb = Global::get_gfd_global_mut();
        glb.get_uid()
    }
    fn set_parameters(&mut self,
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        name: &str,
        update: Option<UpdateFn<D>>,
        render: Option<RenderFn<D>>,
        end: Option<EndFn<D>>,
        allocator: A
    ) {
        // have to ptr::write since this is uninitialized
        unsafe { std::ptr::write(&raw mut self.name, 
            Name::new_in(name, allocator.clone())); }
        self.update_prio = TaskPriority(update_prio);
        self.render_prio = TaskPriority(render_prio);
        self.start_timer = start_timer;
        self.shutdown_timer = shutdown_timer.max(2);
        self.update_fn = update;
        self.render_fn = render;
        self.end_fn = end;
        self._allocator = allocator;
        self.uid = Self::get_uid();
        self.status = if start_timer == 0 { TaskStatus::Update } 
        else { TaskStatus::Begin };
    }

    /// Original function: gfdTaskGetState
    pub fn get_state(&self) -> TaskStatus {
        self.status
    }
    /// Original function: gfdTaskGetWorkData
    pub fn get_work_data(&self) -> Option<&'static D> {
        self.main_work.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_work_data_mut(&mut self) -> Option<&'static mut D> {
        self.main_work.map(|mut v| unsafe { v.as_mut() })
    }

    pub fn get_task_uid(&self) -> u64 { self.uid }

    pub fn get_main_work_ptr(&self) -> *mut D {
        match self.main_work {
            Some(v) => v.as_ptr(),
            None => std::ptr::null_mut()
        }
    }
    pub fn get_update_ptr(&self) -> *mut UpdateFn<D> {
        match self.update_fn {
            Some(v) => v as *mut UpdateFn<D>,
            None => std::ptr::null_mut()
        }
    }
    pub fn get_render_ptr(&self) -> *mut RenderFn<D> {
        match self.render_fn {
            Some(v) => v as *mut RenderFn<D>,
            None => std::ptr::null_mut()
        }
    }
    pub fn get_shutdown_ptr(&self) -> *mut EndFn<D> {
        match self.end_fn {
            Some(v) => v as *mut EndFn<D>,
            None => std::ptr::null_mut()
        }
    }
}

#[allow(dead_code)]
impl<A, D> Task<A, D>
where A: Allocator + Clone
{
    fn get_next_idle_task(&self) -> Option<&'static Self> {
        if self.idle.next != std::ptr::null_mut() {
            Some( unsafe { &*(self.idle.next as *mut Self) })
        } else { None }
    }
    fn get_next_update_task(&self) -> Option<&'static Self> {
        if self.update.next != std::ptr::null_mut() {
            Some( unsafe { &*(self.update.next as *mut Self) })
        } else { None }
    }
    fn get_prev_idle_task(&self) -> Option<&'static Self> {
        if self.idle.prev != std::ptr::null_mut() {
            Some( unsafe { &*(self.idle.prev as *mut Self) })
        } else { None }
    }
    fn get_prev_update_task(&self) -> Option<&'static Self> {
        if self.update.prev != std::ptr::null_mut() {
            Some( unsafe { &*(self.update.prev as *mut Self) })
        } else { None }
    }

    fn get_next_idle_task_ptr(&self) -> *mut Self {
        self.idle.next as *mut Self
    }
    fn get_next_update_task_ptr(&self) -> *mut Self {
        self.update.next as *mut Self
    }
    fn get_prev_idle_task_ptr(&self) -> *mut Self {
        self.idle.prev as *mut Self
    }
    fn get_prev_update_task_ptr(&self) -> *mut Self {
        self.update.prev as *mut Self
    }

    fn get_next_idle_task_mut(&mut self) -> Option<&'static mut Self> {
        if self.idle.next != std::ptr::null_mut() {
            Some( unsafe { &mut *(self.idle.next as *mut Self) })
        } else { None }
    }
    fn get_next_update_task_mut(&mut self) -> Option<&'static mut Self> {
        if self.update.next != std::ptr::null_mut() {
            Some( unsafe { &mut *(self.update.next as *mut Self) })
        } else { None }
    }
    fn get_next_render_task_mut(&mut self) -> Option<&'static mut Self> {
        if self.render.next != std::ptr::null_mut() {
            Some( unsafe { &mut *(self.render.next as *mut Self) })
        } else { None }
    }
    fn get_prev_idle_task_mut(&mut self) -> Option<&'static mut Self> {
        if self.idle.prev != std::ptr::null_mut() {
            Some( unsafe { &mut *(self.idle.prev as *mut Self) })
        } else { None }
    }
    fn get_prev_update_task_mut(&mut self) -> Option<&'static mut Self> {
        if self.update.prev != std::ptr::null_mut() {
            Some( unsafe { &mut *(self.update.prev as *mut Self) })
        } else { None }
    }
    fn get_prev_render_task_mut(&mut self) -> Option<&'static mut Self> {
        if self.render.prev != std::ptr::null_mut() {
            Some( unsafe { &mut *(self.render.prev as *mut Self) })
        } else { None }
    }
}

#[allow(dead_code)]
impl<A, D> Task<A, D>
where A: Allocator + Clone
{
    /// Original function: gfdTaskAttachUpdateList
    pub fn attach_to_update_list(&mut self) {
        let glb = Global::get_gfd_global_mut();
        let glb2 = unsafe { &mut *(&raw mut *glb) };
        let mut mutex = glb.get_tasks_mut().mutex.lock(glb2);
        let pself = &raw mut *self as *mut Task<GfdAllocator>;
        if (&*mutex).get_tasks().get_last_update_task() != std::ptr::null() {
            // insert according to update priority
            let mut curr = Some(unsafe { &mut *((&*mutex).get_tasks().get_last_update_task_mut()) });
            while let Some(task) = curr {
                if self.update_prio < task.update_prio {
                    match task.get_next_update_task_mut() {
                        Some(v) => {
                            // put between [task] and [v]
                            v.update.prev = pself;
                            self.update.next = &raw mut *v as *mut Task<A>;
                            task.update.next = pself;
                            self.update.prev = &raw mut *task as *mut Task<A>;
                        },
                        None => {
                            // put at end
                            (&mut *mutex).get_tasks_mut().set_last_update_task(pself);
                            task.update.next = pself;
                            self.update.prev = &raw mut *task as *mut Task<A>;
                        }
                    };
                    return;
                }
                curr = task.get_prev_update_task_mut();
            }
        }
        // insert at the "end" (at head)
        let last = if (&*mutex).get_tasks().get_first_update_task() != std::ptr::null_mut() {
            let first = (&*mutex).get_tasks().get_first_update_task_mut();
            self.update.next = first as *mut Task<A>;
            unsafe { first.as_mut().unwrap() }.update.prev = pself;
            // (&mut *mutex).tasks.get_last_update_task_mut()
            (&*mutex).get_tasks().get_last_update_task_mut()
        } else {
            pself
        };
        (&mut *mutex).get_tasks_mut().set_last_update_task(last);
        (&mut *mutex).get_tasks_mut().set_first_update_task(pself);
    }

    /// Original function: gfdTaskAttachRenderList
    pub fn attach_to_render_list(&mut self) {
        let glb = Global::get_gfd_global_mut();
        let glb2 = unsafe { &mut *(&raw mut *glb) };
        let mut mutex = glb.get_tasks_mut().mutex.lock(glb2);
        let pself = &raw mut *self as *mut Task<GfdAllocator>;
        if (&*mutex).get_tasks().get_last_render_task() != std::ptr::null() {
            // insert according to render priority
            let mut curr = Some(unsafe { &mut *((&*mutex).get_tasks().get_last_render_task_mut()) });
            while let Some(task) = curr {
                if self.render_prio < task.update_prio {
                    match task.get_next_render_task_mut() {
                        Some(v) => {
                            // put between [task] and [v]
                            v.render.prev = pself;
                            self.render.next = &raw mut *v as *mut Task<A>;
                            task.render.next = pself;
                            self.render.prev = &raw mut *task as *mut Task<A>;
                        },
                        None => {
                            // put at end
                            (&mut *mutex).get_tasks_mut().set_last_render_task(pself);
                            task.render.next = pself;
                            self.render.prev = &raw mut *task as *mut Task<A>;
                        }
                    };
                    return;
                }
                curr = task.get_prev_render_task_mut();
            }
        }
        // insert at the "end" (at head)
        let last = if (&*mutex).get_tasks().get_first_render_task() != std::ptr::null_mut() {
            let first = (&*mutex).get_tasks().get_first_render_task_mut();
            self.render.next = first as *mut Task<A>;
            unsafe { first.as_mut().unwrap() }.render.prev = pself;
            (&mut *mutex).get_tasks().get_last_render_task_mut()
        } else {
            pself
        };
        (&mut *mutex).get_tasks_mut().set_last_render_task(last);
        (&mut *mutex).get_tasks_mut().set_first_render_task(pself);
    }

    fn attach_to_begin_list(&mut self) {
        let glb = Global::get_gfd_global_mut();
        let glb2 = unsafe { &mut *(&raw mut *glb) };
        let mut mutex = glb.get_tasks_mut().mutex.lock(glb2);
        if !(&*mutex).get_flags().contains(GlobalFlags::NO_INCREASE_TASK_START_TIMER) {
            self.start_timer += 1;
        }
        let first_idle = if (&*mutex).get_tasks().get_first_begin_task() != std::ptr::null() {
            Some(unsafe { &mut *(&*mutex).get_tasks().get_first_begin_task_mut() })
        } else { None };
        let pself = &raw mut *self as *mut Task<GfdAllocator>;
        match first_idle {
            Some(v) => {
                self.idle.next = &raw mut *v as *mut Task<A>;
                v.idle.prev = pself;
            },
            None => (&mut *mutex).get_tasks_mut().set_last_update_task(pself),
        }
        (&mut* mutex).get_tasks_mut().set_first_update_task(pself);
    }

    /// Original function: gfdTaskExist
    pub fn exists(&self) -> bool {
        let glb = Global::get_gfd_global_mut();
        let mut curr_task = match self.status {
            TaskStatus::Begin => glb.get_tasks().get_last_begin_task_mut(),
            TaskStatus::Update => glb.get_tasks().get_last_update_task_mut(),
            TaskStatus::Shutdown => glb.get_tasks().get_last_ending_task_mut()
        };
        while curr_task != std::ptr::null_mut() {
            let task_ref = unsafe { &*curr_task };
            if task_ref == &self.uid { return true }
            curr_task = match self.status {
                TaskStatus::Begin |
                TaskStatus::Shutdown => task_ref.get_prev_idle_task_ptr(),
                TaskStatus::Update => task_ref.get_prev_update_task_ptr(),
            };
        }
        false
    }

    /// Original function: gfdTaskFindByName
    pub fn find_by_str(name: &str) -> Option<&'static Self> {
        let mut found = Self::iter_update().find(|p| *p == name);
        if found.is_none() {
            found = Self::iter_begin().find(|p| *p == name);
        }
        found
    }
    pub fn find_by_str_mut(name: &str) -> Option<&'static mut Self> {
        let mut found = Self::iter_update().find(|p| *p == name);
        if found.is_none() {
            found = Self::iter_begin().find(|p| *p == name);
        }
        found.map(|t| unsafe { &mut *(&raw const *t as *mut Self) })
    }
    /// Original function: gfdTaskFindByUID
    pub fn find_by_uid(uid: u64) -> Option<&'static Self> {
        let mut found = Self::iter_update().find(|p| *p == &uid);
        if found.is_none() {
            found = Self::iter_begin().find(|p| *p == &uid);
        }
        found
    }
    /// Original function: gfdTaskGetCurrentID
    pub fn current_id() -> Option<&'static Self> {
        let glb = Global::get_gfd_global_mut();
        if glb.get_tasks().current != std::ptr::null_mut() {
            Some(unsafe { &*(glb.get_tasks().current as *mut Self) })
        } else {
            None
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.get_string()
    }

    pub fn get_name_native(&self) -> &Name<A> {
        &self.name
    }
}

impl<A, D> Task<A, D>
where D: 'static,
      A: Allocator + Clone + 'static
{
    // 0x886c18 (P5 2014 eboot)
    /// Original function: gfdTaskPrintf
    pub fn print_running_tasks() {
        logln!(Debug, "<<< update task >>>");
        for task in Self::iter_update() {
            logln!(Debug, "{}", task.fmt_recursive(0, true));
        }
        logln!(Debug, "<<< render task >>>");
        for task in Self::iter_render() {
            logln!(Debug, "{}", task.fmt_recursive(0, true));
        }
        logln!(Debug, "<<< start task >>>");
        for task in Self::iter_begin() {
            logln!(Debug, "{}", task.fmt_recursive(0, true));
        }
        logln!(Debug, "<<< end task >>>");
        for task in Self::iter_end() {
            logln!(Debug, "{}", task.fmt_recursive(0, true));
        }
    }
}

impl<A, D> Task<A, D>
where D: UpdateTask + 'static,
      A: Allocator + Clone
{

    pub fn new_core_update_from_ref(
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        work: &mut D,
        allocator: A
    ) -> &'static mut Self {
        Self::new_core_update(update_prio, render_prio, 
        start_timer, shutdown_timer, &raw mut *work, allocator)
    }

    /// Original function: gfdTaskStartCore
    pub fn new_core_update(
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        work: *mut D,
        allocator: A
    ) -> &'static mut Self
    {
        let new_task = Self::get_task_allocation();
        new_task.set_parameters(
            update_prio, render_prio, start_timer, shutdown_timer,
            D::NAME, Some(D::update as UpdateFn<D>),
            None, Some(D::shutdown as EndFn<D>), allocator
        );
        new_task.main_work = if work != std::ptr::null_mut() {
            Some(unsafe { NonNull::new_unchecked(work) })
        } else { None };
        if start_timer == 0 {
            new_task.attach_to_update_list();
        } else {
            new_task.attach_to_begin_list();
        }
        new_task
    }

    pub fn new_core_update_child<E>(
        parent: &mut Task<A, E>,
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        work: *mut D,
        allocator: A
    ) -> &'static mut Self
    where E: UpdateTask {
        let task = Self::new_core_update(
            update_prio, render_prio, start_timer, 
            shutdown_timer, work, allocator
        );
        task.link.parent = &raw mut *parent as *mut Task<A>;
        task
    }
}

impl<A, D> Task<A, D>
where D: UpdateTask + InitTask + 'static,
      A: Allocator + Clone
{

    pub fn new_update(
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        allocator: A
    ) -> &'static mut Self {
        let work = allocator.allocate_zeroed(Self::get_layout())
            .unwrap().cast().as_ptr();
        unsafe { std::ptr::write(work, D::new()); }
        Self::new_core_update(
            update_prio, render_prio,
            start_timer, shutdown_timer,
            work,
            allocator
        )
    }
}

impl<A, D> Task<A, D>
where D: RenderTask + 'static,
      A: Allocator + Clone
{
    pub fn new_core_render_from_ref(
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        work: &mut D,
        allocator: A
    ) -> &'static mut Self {
        Self::new_core_render(update_prio, render_prio, 
        start_timer, shutdown_timer, &raw mut *work, allocator)
    }

    pub fn new_core_render(
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        work: *mut D,
        allocator: A
    ) -> &'static mut Self
    {
        let new_task = Self::get_task_allocation();
        new_task.set_parameters(
            update_prio, render_prio, start_timer, shutdown_timer,
            D::NAME, 
            Some(D::update as UpdateFn<D>),
            Some(D::render as RenderFn<D>),
            Some(D::shutdown as EndFn<D>), 
            allocator
        );
        new_task.main_work = if work != std::ptr::null_mut() {
            Some(unsafe { NonNull::new_unchecked(work) })
        } else { None };
        if start_timer == 0 {
            new_task.attach_to_update_list();
            new_task.attach_to_render_list();
        } else {
            new_task.attach_to_begin_list();
        }
        new_task
    }
}

impl<A, D> Task<A, D>
where D: RenderTask + InitTask + 'static,
      A: Allocator + Clone
{

    pub fn new_render(
        update_prio: u32,
        render_prio: u32,
        start_timer: i32,
        shutdown_timer: i32,
        allocator: A
    ) -> &'static mut Self {
        let work = allocator.allocate_zeroed(Self::get_layout())
            .unwrap().cast().as_ptr();
        unsafe { std::ptr::write(work, D::new()); }
        Self::new_core_render(
            update_prio, render_prio,
            start_timer, shutdown_timer,
            work,
            allocator
        )
    }
}

pub struct TaskLinkIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{ curr: Option<&'a Task<A, D>> }

impl<'a, A, D> Iterator for TaskLinkIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{
    type Item = &'a Task<A, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = if v.link.sibling != std::ptr::null_mut() {
                Some(unsafe { &*(v.link.sibling as *mut Task<A, D>) })
            } else { None };
            v
        })
    }
}

pub struct TaskBeginIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{ curr: Option<&'a Task<A, D>> }

impl<'a, A, D> Iterator for TaskBeginIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{
    type Item = &'a Task<A, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = Some(v);
            while let Some(next_t) = self.curr {
                self.curr = if next_t.idle.prev != std::ptr::null_mut() {
                    Some(unsafe { &*(next_t.idle.prev as *mut Task<A, D>) })
                } else { None };
                if next_t.get_state() == TaskStatus::Begin { break; }
            }
            v
        })
    }
}

pub struct TaskEndIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{ curr: Option<&'a Task<A, D>> }

impl<'a, A, D> Iterator for TaskEndIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{
    type Item = &'a Task<A, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = Some(v);
            while let Some(next_t) = self.curr {
                self.curr = if next_t.idle.prev != std::ptr::null_mut() {
                    Some(unsafe { &*(next_t.idle.prev as *mut Task<A, D>) })
                } else { None };
                if next_t.get_state() == TaskStatus::Shutdown { break; }
            }
            v
        })
    }
}

pub struct TaskUpdateIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{ curr: Option<&'a Task<A, D>> }

impl<'a, A, D> Iterator for TaskUpdateIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{
    type Item = &'a Task<A, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = if v.update.prev != std::ptr::null_mut() {
                Some(unsafe { &*(v.update.prev as *mut Task<A, D>) })
            } else { None };
            v
        })
    }
}

pub struct TaskRenderIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{ curr: Option<&'a Task<A, D>> }

impl<'a, A, D> Iterator for TaskRenderIterator<'a, A, D>
where D: 'static,
      A: Allocator + Clone
{
    type Item = &'a Task<A, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = if v.render.prev != std::ptr::null_mut() {
                Some(unsafe { &*(v.render.prev as *mut Task<A, D>) })
            } else { None };
            v
        })
    }
}

impl<A, D> Task<A, D>
where A: Allocator + Clone
{
    fn iter_link(&self) -> TaskLinkIterator<'_, A, D> {
        let curr = if self.link.child != std::ptr::null_mut() {
            Some(unsafe { &*(self.link.child as *mut Task<A, D>) })
        } else { None };
        TaskLinkIterator { curr }
    }
}

#[allow(dead_code)]
impl<A, D> Task<A, D>
where D: 'static,
      A: Allocator + Clone
{
    pub fn iter_begin() -> TaskBeginIterator<'static, A, D> {
        let glb = Global::get_gfd_global_mut();
        let mut curr = if glb.get_tasks().get_last_begin_task() != std::ptr::null_mut() {
            Some(unsafe { &*(glb.get_tasks().get_last_begin_task() as *mut Task<A, D>) })
        } else { None };
        while let Some(next_t) = curr {
            curr = if next_t.idle.prev != std::ptr::null_mut() {
                Some(unsafe { &*(next_t.idle.prev as *mut Task<A, D>) })
            } else { None };
            if next_t.get_state() == TaskStatus::Begin { break; }
        }
        TaskBeginIterator { curr }
    }
    pub fn iter_end() -> TaskEndIterator<'static, A, D> {
        let glb = Global::get_gfd_global_mut();
        let mut curr = if glb.get_tasks().get_last_ending_task() != std::ptr::null_mut() {
            Some(unsafe { &*(glb.get_tasks().get_last_ending_task() as *mut Task<A, D>) })
        } else { None };
        while let Some(next_t) = curr {
            curr = if next_t.idle.prev != std::ptr::null_mut() {
                Some(unsafe { &*(next_t.idle.prev as *mut Task<A, D>) })
            } else { None };
            if next_t.get_state() == TaskStatus::Shutdown { break; }
        }
        TaskEndIterator { curr }
    }
    pub fn iter_update() -> TaskUpdateIterator<'static, A, D> {
        let glb = Global::get_gfd_global_mut();
        let update_task = glb.get_tasks().get_last_update_task();
        let curr = if update_task != std::ptr::null_mut() {
            Some(unsafe { &*(update_task as *mut Task<A, D>) })
        } else { None };
        TaskUpdateIterator { curr }
    }
    pub fn iter_render() -> TaskRenderIterator<'static, A, D> {
        let glb = Global::get_gfd_global_mut();
        let update_task = glb.get_tasks().get_last_render_task();
        let curr = if update_task != std::ptr::null_mut() {
            Some(unsafe { &*(update_task as *mut Task<A, D>) })
        } else { None };
        TaskRenderIterator { curr }
    }
}

pub trait UpdateTask {
    const NAME: &'static str;
    fn update(task: &mut Task<GfdAllocator, Self>, delta: f32) 
        -> TaskFunctionReturn where Self: Sized;
    fn shutdown(task: &mut Task<GfdAllocator, Self>) -> ()
        where Self: Sized;
}

pub trait RenderTask : UpdateTask {
    fn render(task: &mut Task<GfdAllocator, Self>) 
        -> TaskFunctionReturn where Self: Sized;
}

pub trait InitTask : UpdateTask {
    fn new() -> Self where Self: Sized;
}

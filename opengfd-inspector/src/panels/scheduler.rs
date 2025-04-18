use crate::{
    components::{
        // searchbar::Searchbar,
        table::{
            InspectorTable,
            TableDraw
        }
    },
    panels::common::InspectorPanel
};
use imgui::Ui;
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::Task as GfdTask
};
use std::ops::Deref;

type GfdDefaultTask = GfdTask<GfdAllocator, u8>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TaskTableEntry(&'static GfdDefaultTask);
impl Deref for TaskTableEntry {
    type Target = GfdDefaultTask;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl TableDraw<SchedulerPanel> for TaskTableEntry {
    fn draw_contents(&self, ui: &mut Ui, ctx: &mut SchedulerPanel, index: usize) {
        match index {
            0 => {
                if ui.selectable_config(&format!("{}", self.get_name_native()))
                    .span_all_columns(true).build()
                {
                    let sel_task = unsafe { &*(&raw const **self) };
                    ctx.selected_task = Some(sel_task);
                }
                if ctx.selected_task.is_some()
                && std::ptr::addr_eq(self, ctx.selected_task.as_ref().unwrap()) {
                    ui.set_item_default_focus();
                }
            },
            1 => ui.text(&format!("{}", self.get_task_uid())),
            2 => ui.text(&format!("0x{:x}", self.get_main_work_ptr() as usize)),
            3 => ui.text(&format!("0x{:x}", self.get_update_ptr() as usize)),
            4 => ui.text(&format!("0x{:x}", self.get_render_ptr() as usize)),
            5 => ui.text(&format!("0x{:x}", self.get_shutdown_ptr() as usize)),
            _ => ()
        }
    }
}

#[derive(Debug)]
pub struct SchedulerPanel {
    table: InspectorTable<'static, TaskTableEntry, SchedulerPanel, 6>,
    selected_task: Option<&'static GfdDefaultTask>,
}
impl InspectorPanel for SchedulerPanel {
    fn get_panel_name(&self) -> &'static str { "Scheduler" }
    fn draw_contents(&mut self, ui: &mut Ui) {
        let self_ptr = unsafe { &mut *(&raw mut *self) };
        let entries: Vec<TaskTableEntry> = GfdDefaultTask::iter_update().map(|v| TaskTableEntry(v)).collect();
        // check if selected task still exists
        if self.selected_task.is_some() 
        && entries.iter().find(|v| std::ptr::addr_eq(&***v, *self.selected_task.as_ref().unwrap())).is_none() {
            self.selected_task = None;
        }
        // update entries
        self.table.set_entries(entries);
        self.table.draw_table(ui, self_ptr);
        ui.text(&format!("Showing {} tasks", self.table.contents.len()));
        ui.separator();
        // selected task details
        if let Some(t) = self.selected_task {
            ui.text(&format!("Selected task: {}", t.get_name_native()));
        } else {
            ui.text("No task is selected");
        }
    }
}
impl SchedulerPanel {
    pub(crate) fn new() -> Self {
        Self {
            table: InspectorTable::new(
                "Task Scheduler List",
                Some([
                    "Task Name",
                    "Task ID",
                    "State",
                    "Update",
                    "Render",
                    "Shutdown"
                ]),
                crate::components::table::default_flags(),
                crate::components::table::default_height()
            ),
            selected_task: None
        }
    }
}
use imgui::{ ListClipper, TableColumnSetup, TableFlags, Ui };
use std::{
    marker::PhantomData,
    mem::MaybeUninit
};

pub trait TableDraw<TContext> {
    fn draw_contents(&self, ui: &mut Ui, ctx: &mut TContext);
}

pub(crate) fn default_flags() -> TableFlags {
    TableFlags::BORDERS | TableFlags::ROW_BG | TableFlags::RESIZABLE | TableFlags::SCROLL_Y
}
pub(crate) fn default_height() -> f32 { 300. }

#[allow(dead_code)]
#[derive(Debug)]
pub struct InspectorTable<'a, TContents, TContext, const C: usize>
where TContents : TableDraw<TContext>
{
    pub(crate) contents: Vec<TContents>,
    table_name: &'a str,
    columns: Option<[&'a str; C]>,
    flags: TableFlags,
    height: f32,
    _context: PhantomData<TContext>
}

#[allow(dead_code)]
impl<'a, TContents, TContext, const C: usize> InspectorTable<'a, TContents, TContext, C>
where TContents : TableDraw<TContext>
{
    fn create_header_column(&self) -> Option<[TableColumnSetup<&'a str>; C]> {
        self.columns.map(|v| {
            let mut result: MaybeUninit<[TableColumnSetup<&'a str>; C]> = MaybeUninit::uninit();
            for i in 0..C {
                unsafe { result.assume_init_mut()[i] = TableColumnSetup::new(v[i]) }
            }
            unsafe { result.assume_init() }
        })
    }

    pub(crate) fn draw_table(&mut self, ui: &mut Ui, ctx: &mut TContext) {
        let ui_copy0 = unsafe { &mut *(&raw mut *ui) };
        let ui_copy1 = unsafe { &mut *(&raw mut *ui) };
        if let Some(_) = match self.create_header_column() {
            Some(h) => ui.begin_table_header_with_sizing(
                self.table_name, 
                h, 
                self.flags, 
                [0. , self.height], 
                0.),
            None => ui.begin_table_with_sizing(
                self.table_name, 
                C,
                self.flags, 
                [0. , self.height], 
                0.),
        } {
            let clipper = ListClipper::new(self.contents.len() as i32);
            let clip = clipper.begin(ui_copy0);
            for i in clip.iter() {
                self.contents[i as usize].draw_contents(ui_copy1, ctx);
            }
        }
    }

    pub(crate) fn set_entries(&mut self, contents: Vec<TContents>) {
        self.contents = contents;
    }

    pub(crate) fn new(
        table_name: &'a str, 
        columns: Option<[&'a str; C]>,
        flags: TableFlags,
        height: f32
    ) -> Self {
        Self {
            contents: vec![],
            table_name,
            columns,
            height,
            flags,
            _context: PhantomData::<TContext>
        }
    }
}
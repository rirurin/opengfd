use imgui::{
    ComboBoxFlags,
    Ui
};
use regex::Regex;
use std::{
    fmt::Debug,
    ptr::NonNull
};

pub trait SearchType : Debug {
    fn get_name(&self) -> &str;
    fn search_matches(&self, src: &str, tgt: &str) -> bool;
    fn update(&mut self, new: &str);
}
#[derive(Debug)]
pub struct SearchContaining;
impl SearchType for SearchContaining {
    fn get_name(&self) -> &str { "Contains" }
    fn update(&mut self, _new: &str) {}
    fn search_matches(&self, src: &str, tgt: &str) -> bool {
        src.contains(tgt)
    }
}
impl SearchContaining {
    pub fn new() -> Self { Self }
}
#[derive(Debug)]
pub struct SearchWholeWord;
impl SearchType for SearchWholeWord {
    fn get_name(&self) -> &str { "Whole Word" }
    fn update(&mut self, _new: &str) {}
    fn search_matches(&self, src: &str, tgt: &str) -> bool {
        src == tgt
    }
}
impl SearchWholeWord {
    pub fn new() -> Self { Self }
}

#[derive(Debug)]
pub struct SearchRegex {
    regex: Regex,
    error: Option<regex::Error>
}
impl SearchType for SearchRegex {
    fn get_name(&self) -> &str { "Whole Word" }
    fn update(&mut self, new: &str) {
        match Regex::new(new) {
            Ok(v) => self.regex = v,
            Err(e) => self.error = Some(e)
        }
    }
    fn search_matches(&self, src: &str, _tgt: &str) -> bool {
        self.regex.is_match(src)
    }
}

#[derive(Debug)]
pub struct Searchbar {
    buf: String,
    label: String,
    show_label: bool,
    search_types: Vec<Box<dyn SearchType>>,
    type_selected: NonNull<Box<dyn SearchType>>
}
impl Searchbar {
    pub(crate) fn new<T>(label: T, show_label: bool) -> Self
    where T: Into<String>
    { 
        let mut out = Self {
            buf: String::new(),
            label: label.into(),
            show_label,
            search_types: vec![
                Box::new(SearchContaining::new()),
                Box::new(SearchWholeWord::new())
            ],
            type_selected: NonNull::dangling()
        };
        out.type_selected = unsafe { NonNull::new_unchecked(&raw mut out.search_types[0]) };
        out
    }
    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        let region = ui.content_region_avail();
        ui.set_next_item_width(region[0] * 0.6);
        let label_fmt = match self.show_label {
            true => self.label.clone(),
            false => format!("##{}", self.label)
        };
        ui.input_text(&label_fmt, &mut self.buf).build();
        ui.same_line_with_spacing(0., 10.);
        ui.set_next_item_width(region[0] * 0.25);
        if let Some(c) = ui.begin_combo_with_flags(
            "Search type", 
            unsafe { self.type_selected.as_ref().get_name() },
            ComboBoxFlags::empty()
        ) {
            let sel_idx = self.search_types.iter().position(
                |p| std::ptr::addr_eq(p, unsafe { self.type_selected.as_ref() })).unwrap();
            for (i, sel) in self.search_types.iter_mut().enumerate() {
                if ui.selectable_config(sel.get_name()).selected(i == sel_idx).build() {
                    self.type_selected = unsafe { NonNull::new_unchecked(&raw mut *sel) };
                }
                if i == sel_idx { ui.set_item_default_focus(); }
            }
        }
    }
}
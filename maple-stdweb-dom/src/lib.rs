#![feature(never_type, proc_macro_hygiene)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate maple_core;
extern crate maple_stdweb;

mod components;
mod canvas;
mod panel;
mod tabs;

use std::cell::RefCell;
pub use self::components::*;
pub use self::canvas::*;
pub use self::panel::*;
pub use self::tabs::*;
use maple_core::prelude::*;
use std::collections::HashMap;

pub struct CanvasContextEngine {
    data: RefCell<String>
}
impl Engine for CanvasContextEngine {}

impl CanvasContextEngine {
    pub fn new() -> Self {
        return Self {
            data: RefCell::new(String::new())
        }
    }

    pub fn to_string(&self) -> String {
        self.data.borrow().clone()
    }

    pub fn add(&self, code: &str) {
        self.data.borrow_mut().push_str(code);
    }
}

pub struct HtmlEngine {
    data: RefCell<String>
}
impl Engine for HtmlEngine {}

impl HtmlEngine {
    pub fn new() -> Self {
        return Self {
            data: RefCell::new(String::new())
        }
    }

    pub fn to_string(&self) -> String {
        self.data.borrow().clone()
    }

    pub fn open(&self, name: &str, attrs: Option<HashMap<String, String>>) {
        let attrs_str = if let Some(attrs_map) = attrs {
            attrs_map.iter().fold(" ".to_string(), |acc, (ref l, ref r)| acc + l + "=\"" + r + "\" ")

        } else {
            "".to_string()
        };

        self.data.borrow_mut()
            .push_str(&format!("<{}{}>\n", name, attrs_str));
    }

    pub fn close(&self, name: &str) {
        self.data.borrow_mut()
            .push_str(&format!("</{}>\n", name));
    }

    pub fn text(&self, dat: &str) {
         self.data.borrow_mut()
            .push_str(dat);
    }

    pub fn script(&self, data: &str) {
        self.data.borrow_mut()
            .push_str(&("<script>".to_string() + data + "</script>"));
    }
}

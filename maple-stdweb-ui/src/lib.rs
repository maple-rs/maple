#![feature(proc_macro_non_items, existential_type)]

pub mod panel;
pub mod tabs;

pub use self::panel::*;
pub use self::tabs::*;

pub use maple_stdweb_ui::HtmlEngine;
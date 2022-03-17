#![feature(associated_type_defaults, never_type, unsize, specialization)]
#![feature(generic_associated_types)]

pub mod callback;
pub mod component;
pub mod context;
pub mod convert;
pub mod engine;

pub mod children;
pub mod node;
pub mod renderable;
pub mod view;

#[macro_export]
macro_rules! children {
    ($w:expr) => ($w);
    ($w1:expr, $($rest:tt)*) => (Children2::new($w1, children!($($rest)*)));
}

pub mod prelude {
    pub use crate::callback::{Callback, Callback1};
    pub use crate::children::Children2;
    pub use crate::component::{Component, ComponentMsg};
    pub use crate::context::{Context, DefaultContext};
    pub use crate::convert::{MyFrom, MyInto};
    pub use crate::engine::Engine;
    pub use crate::node::Node;
    pub use crate::renderable::{RenderImplementation, Renderable, Stub};
    pub use crate::view::View;
}

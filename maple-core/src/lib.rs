#![feature(associated_type_defaults, never_type, unsize, specialization)]

pub mod convert;
pub mod engine;
pub mod context;
pub mod callback;
pub mod component;

#[macro_use]
pub mod children;
pub mod renderable;
pub mod node;
pub mod view;

pub mod prelude {
    pub use crate::convert::{MyFrom, MyInto};
    pub use crate::engine::Engine;
    pub use crate::context::{Context, DefaultContext};
    pub use crate::callback::{Callback, Callback1};
    pub use crate::component::{Component, ComponentMsg};
    pub use crate::children::{Children2};
    pub use crate::renderable::{Renderable, RenderImplementation, Stub};
    pub use crate::node::Node;
    pub use crate::view::View;
}
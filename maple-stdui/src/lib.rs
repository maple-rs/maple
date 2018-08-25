pub mod panel;
pub mod tabs;
pub mod text;

pub mod prelude {
    pub use super::panel::*;
    pub use super::tabs;
    pub use super::text::*;
}
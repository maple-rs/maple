mod div;
mod span;
mod button;
mod canvas;
mod canvas_comp;

pub use self::canvas_comp::*;
pub use self::div::{Div, DivEvents, DivProps};
pub use self::span::{Span, SpanProps};
pub use self::button::{Button, ButtonEvents, ButtonProps};
pub use self::canvas::{Canvas, CanvasEvents, CanvasProps};

use std::borrow::Cow;
use maple_core::prelude::Component;

pub struct Text {
    pub props: TextProps
}

#[derive(Default)]
pub struct TextProps {
    pub text: Option<Cow<'static, str>>
}

impl TextProps {
    pub fn new<T: Into<Cow<'static, str>>>(txt: T) -> Self {
        TextProps {
            text: Some(txt.into())
        }
    }
}

impl Component for Text {
    type Props = TextProps;

    fn create(props: TextProps) -> Self {
        Self { props }
    }
}
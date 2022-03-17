mod button;
mod canvas;
mod canvas_comp;
mod div;
mod span;

pub use self::button::{Button, ButtonEvents, ButtonProps};
pub use self::canvas::{Canvas, CanvasEvents, CanvasProps};
pub use self::canvas_comp::*;
pub use self::div::{Div, DivEvents, DivProps};
pub use self::span::{Span, SpanProps};

use maple_core::prelude::Component;
use std::borrow::Cow;

pub struct Text {
    pub props: TextProps,
}

#[derive(Default)]
pub struct TextProps {
    pub text: Option<Cow<'static, str>>,
}

impl TextProps {
    pub fn new<T: Into<Cow<'static, str>>>(txt: T) -> Self {
        TextProps {
            text: Some(txt.into()),
        }
    }
}

impl Component for Text {
    type Props = TextProps;

    fn create(props: TextProps) -> Self {
        Self { props }
    }
}

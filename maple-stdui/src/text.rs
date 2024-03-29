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

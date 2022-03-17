use maple_core::prelude::*;

pub enum SpanEvents {
    Click,
}

pub struct Span {
    props: SpanProps,
}

#[derive(Default)]
pub struct SpanProps {
    pub id: Option<&'static str>,
    pub name: Option<&'static str>,
    pub class: Option<&'static str>,
    pub click: Option<Box<dyn Callback<Arg = ()>>>,
}

impl Component for Span {
    type Props = SpanProps;
    type Msg = SpanEvents;

    fn create(props: SpanProps) -> Self {
        Span { props }
    }

    fn update(&mut self, msg: Self::Msg) -> bool {
        match msg {
            SpanEvents::Click => {
                if let Some(cb) = &self.props.click {
                    cb.call(());
                }
            }
        };

        false
    }
}

use maple_core::prelude::*;

pub enum DivEvents {
    Click,
}

pub struct Div {
    pub props: DivProps,
}

#[derive(Default)]
pub struct DivProps {
    pub id: Option<&'static str>,
    pub name: Option<&'static str>,
    pub class: Option<&'static str>,
    pub click: Option<Box<dyn Fn() -> DivEvents>>,
}

impl Component for Div {
    type Props = DivProps;
    type Msg = DivEvents;

    fn create(props: DivProps) -> Self {
        Div { props }
    }

    fn update(&mut self, _msg: Self::Msg) -> bool {
        false
    }
}

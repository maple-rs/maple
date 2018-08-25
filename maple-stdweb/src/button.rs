use maple_core::prelude::Component;

pub enum ButtonEvents {
    Click
}

pub struct Button {
    props: ButtonProps
}

#[derive(Default)]
pub struct ButtonProps {
    pub id: Option<&'static str>,
    pub name: Option<&'static str>,
    pub class: Option<&'static str>,
    pub click: Option<Box<Fn() -> ButtonEvents>>
}

impl Component for Button {
    type Props = ButtonProps;
    type Msg = ButtonEvents;

    fn create(props: ButtonProps) -> Self {
        Button { props }
    }

    fn update(&mut self, msg: Self::Msg) -> bool {
        false
    }
}
use maple_core::prelude::Component;

pub enum CanvasEvents {
    Click,
}

pub struct Canvas {
    pub props: CanvasProps,
}

#[derive(Default)]
pub struct CanvasProps {
    pub id: Option<&'static str>,
    pub name: Option<&'static str>,
    pub class: Option<&'static str>,
    pub click: Option<Box<dyn Fn() -> CanvasEvents>>,
}

impl Component for Canvas {
    type Props = CanvasProps;
    type Msg = CanvasEvents;

    fn create(props: CanvasProps) -> Self {
        Canvas { props }
    }

    fn update(&mut self, _msg: Self::Msg) -> bool {
        true
    }
}

use maple_core::prelude::Component;

pub struct Circle {
    pub props: CircleProps
}

#[derive(Default)]
pub struct CircleProps {
    pub id: Option<&'static str>,
    pub fill: Option<&'static str>,
    pub stroke: Option<&'static str>,
    pub cx: Option<u32>,
    pub cy: Option<u32>,
    pub r: Option<u32>
}

impl Component for Circle {
    type Props = CircleProps;

    fn create(props: CircleProps) -> Self {
        Circle { props }
    }
}
use maple_core::prelude::Component;

pub struct Rect {
    pub props: RectProps
}

#[derive(Default)]
pub struct RectProps {
    pub id: Option<&'static str>,
    pub name: Option<&'static str>,
    pub fill: Option<&'static str>,
    pub stroke: Option<&'static str>,
    pub x1: Option<u32>,
    pub y1: Option<u32>,
    pub x2: Option<u32>,
    pub y2: Option<u32>
}

impl Component for Rect {
    type Props = RectProps;
    
    fn create(props: RectProps) -> Self {
        Rect { props }
    }
}
use maple_core::prelude::*;

pub struct Panel {
    props: PanelProps,
}

#[derive(Default)]
pub struct PanelProps {}

impl Component for Panel {
    type Props = PanelProps;

    fn create(props: PanelProps) -> Self {
        Panel { props }
    }
}

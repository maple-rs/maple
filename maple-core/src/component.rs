use std::default::Default;

pub trait Component {
    type Props: Default;
    type Msg = !;

    fn create(props: Self::Props) -> Self;

    fn update(&mut self, _msg: Self::Msg) -> bool {
        return true;
    }
}

pub trait ComponentMsg {
    type Component: Component;
}

impl Component for ! {
    type Props = ();

    fn create(_props: Self::Props) -> ! {
        unreachable!()
    }
}

impl ComponentMsg for () {
    type Component = !;
}

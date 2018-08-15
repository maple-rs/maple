
pub trait Component {
    type Props: Default;
    type Msg = !;

    fn create(props: Self::Props) -> Self;

    fn update(&mut self, _msg: Self::Msg) -> bool {
        return true;
    }
}

impl Component for ! {
    type Props = ();

    fn create(_props: Self::Props) -> ! {
        unreachable!()
    }
}

pub trait ComponentMsg {
    type Component: Component;
}

impl ComponentMsg for () {
    type Component = !;
}

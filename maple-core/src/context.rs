///
/// 
/// 
/// 
/// 
/// 
pub trait Context: Clone {
    type Context: Context;

    fn unwrap(self) -> Self::Context;
    fn wrap(ctx: Self::Context) -> Self;
}


#[derive(Clone)]
pub struct DefaultContext;

impl Context for DefaultContext {
    type Context = Self;

    fn unwrap(self) -> Self::Context {
        self
    }

    fn wrap(_ctx: Self::Context) -> Self {
        unimplemented!()
    }
}

impl Context for ! {
    type Context = !;

    fn unwrap(self) -> Self::Context {
        unreachable!()
    }

    fn wrap(_ctx: Self::Context) -> Self {
        unreachable!()
    }
}

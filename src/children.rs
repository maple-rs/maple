#[macro_export]
macro_rules! children {
    ($w:expr) => ($w);
    ($w1:expr, $($rest:tt)*) => (Children2::new($w1, children!($($rest)*)));
}

pub struct Children2<E, CH0, CH1> 
    where E: Engine,
          CH0: Renderable<Engine = E>,
          CH1: Renderable<Engine = E>
{
    _e: PhantomData<E>,

    child0: CH0,
    child1: CH1
}

impl<E, CH0, CH1> Children2<E, CH0, CH1> 
    where E: Engine,
          CH0: Renderable<Engine = E>,
          CH1: Renderable<Engine = E>
{
    pub fn new(child0: CH0, child1: CH1) -> Self {
        Self {
            _e: PhantomData,
            child0,
            child1
        }
    }
}

impl<E, CH0, CH1> Renderable for Children2<E, CH0, CH1>
    where E: Engine,
          CH0: Renderable<Engine = E>,
          CH1: Renderable<Engine = E>
{
    type Engine = E;
    
    fn render(&self, eng: &Self::Engine) {
        self.child0.render(eng);
        self.child1.render(eng);
    }
}
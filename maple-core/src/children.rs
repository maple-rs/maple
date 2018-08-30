use crate::engine::Engine;
use crate::renderable::Renderable;
use std::marker::PhantomData;

pub struct Children2<E, CH0, CH1> 
    where E: Engine,
          CH0: Renderable<E>,
          CH1: Renderable<E>
{
    _e: PhantomData<E>,

    child0: CH0,
    child1: CH1
}

impl<E, CH0, CH1> Children2<E, CH0, CH1> 
    where E: Engine,
          CH0: Renderable<E>,
          CH1: Renderable<E>
{
    pub fn new(child0: CH0, child1: CH1) -> Self {
        Self {
            _e: PhantomData,
            child0,
            child1
        }
    }
}

impl<E, CH0, CH1> Renderable<E> for Children2<E, CH0, CH1>
    where E: Engine,
          CH0: Renderable<E>,
          CH1: Renderable<E>
{
    fn render(&self, eng: &E) {
        self.child0.render(eng);
        self.child1.render(eng);
    }
}
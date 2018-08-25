use crate::engine::Engine;
use crate::renderable::{Renderable, RenderImplementation};
use std::marker::PhantomData;
///
/// 
/// 
/// 
pub struct Node <E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<E, CHE>,
        CH: Renderable<CHE>
{
    item: I,
    children: CH,
    _m: PhantomData<(E, CHE)>
}

impl<E, I, CHE, CH> Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<E, CHE>,
        CH: Renderable<CHE>
{
    pub fn new(item: I, children: CH) -> Self {
        Self {
            item,
            children,
            _m: Default::default()
        }
    }
}

impl<E, I, CHE, CH> Renderable<E> for Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<E, CHE>,
        CH: Renderable<CHE>
{
    fn render(&self, eng: &E) {
        self.item.render_impl(eng, &self.children)
    }
}

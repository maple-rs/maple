use std::marker::PhantomData;

use crate::engine::Engine;

/// defines implementation of how the children elements binds to the component
/// in dom backend it actually attaches/detaches elements to each other
/// 
/// 
pub trait RenderImplementation<E, CE> 
    where E: Engine,
          CE: Engine,
{
    fn render_impl<C: Renderable<CE>>(&self, eng: &E, children: &C);
}

/// Trait Renderable 
/// defines implementation of how the component should be rendered itself
/// 
/// 
/// 
pub trait Renderable<E> 
    where E: Engine,
{
    fn render(&self, eng: &E);
}

///
/// for nodes without children
/// 
/// 
/// 
#[derive(Default)]
pub struct Stub<E: Engine> {
    _e: PhantomData<E>
}

impl<E: Engine> Renderable<E> for Stub<E>  {
    fn render(&self, _eng: &E) {}
}

// default impl<E, T> Renderable<E> for T 
//     where E: Engine,
//           T: RenderImplementation<E, E>,
// {
//     fn render(&self, eng: &E) {
//         self.render_impl(eng, &Stub::default())
//     }
// }

impl<E: Engine> Renderable<E> for Box<dyn Renderable<E>> {
    fn render(&self, eng: &E) {
        (**self).render(eng);
    }
}

impl <E, T> Renderable<E> for Vec<T> 
    where E: Engine,
          T: Renderable<E>
{
    fn render(&self, eng: &E) {
        for child in self.iter() {
            child.render(eng);
        }
    }
}

impl <E, T> Renderable<E> for Option<T> 
    where E: Engine,
          T: Renderable<E>
{
    fn render(&self, eng: &E) {
        if let Some(inner) = self {
            inner.render(eng);
        }
    }
}
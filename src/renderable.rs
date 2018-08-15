/// defines implementation of how the children elements binds to the component
/// in dom backend it actually attaches/detaches elements to each other
/// 
/// 
pub trait RenderImplementation {
    type Engine: Engine;
    type ChildrenEngine: Engine = Self::Engine;
    
    fn render_impl<C>(&self, eng: &Self::Engine, children: &C)
        where C: Renderable<Engine = Self::ChildrenEngine>;
}

/// Trait Renderable 
/// defines implementation of how the component should be rendered itself
/// 
/// 
/// 
pub trait Renderable {
    type Engine: Engine;

    fn render(&self, eng: &Self::Engine);
}

///
/// for nodes without children
/// 
/// 
pub struct Stub<E: Engine> {
    _e: PhantomData<E>
}

impl<E: Engine> Renderable for Stub<E>  {
    type Engine = E;
    
    fn render(&self, _eng: &Self::Engine) {}
}

impl<E: Engine> Stub<E> {
    pub fn new() -> Self {
        Self {
            _e: PhantomData
        }
    }
}

impl<E, T> Renderable for T 
    where E: Engine,
          T: RenderImplementation<Engine = E, ChildrenEngine = E>
{
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        self.render_impl(eng, &Stub::new())
    }
}

impl<E: Engine> Renderable for Box<dyn Renderable<Engine = E>> {
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        (**self).render(eng);
    }
}

impl <'a, E> Renderable for Vec<&'a (dyn Renderable<Engine = E> + 'a)>
    where E: Engine + 'a
{
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        for child in self.iter() {
            child.render(eng);
        }
    }
}

impl <E, T> Renderable for Vec<T> 
    where E: Engine,
          T: Renderable<Engine = E>
{
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        for child in self.iter() {
            child.render(eng);
        }
    }
}

impl <E, T> Renderable for Option<T> 
    where E: Engine,
          T: Renderable<Engine = E>
{
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        if let Some(inner) = self {
            inner.render(eng);
        }
    }
}
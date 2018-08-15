#![feature(associated_type_defaults, never_type, unsize)]

mod convert;
use std::marker::PhantomData;
pub use convert::*;

pub trait Callback {
    type Arg;

    fn call(&self, arg: Self::Arg);
}

pub struct Callback1<A, M, CB> 
    where M: ComponentMsg,
          CB: Fn(A) -> M
{
    cb: CB,
    _m1: PhantomData<A>,
    _m2: PhantomData<M>
}

impl<A, M, CB> Callback for Callback1<A, M, CB>
    where M: ComponentMsg,
          CB: Fn(A) -> M
{
    type Arg = A;

    fn call(&self, arg: Self::Arg) {
        
    }
}

impl<A, M, CB> Callback1<A, M, CB>
    where M: ComponentMsg,
          CB: Fn(A) -> M
{
    pub fn new(cb: CB) -> Self {
        Self {
            cb,
            _m1: PhantomData,
            _m2: PhantomData
        }
    }
}

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

pub trait Engine {}
impl Engine for ! {}

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

pub trait RenderImplementation {
    type Engine: Engine;
    type ChildrenEngine: Engine = Self::Engine;
    
    fn render_impl<C>(&self, eng: &Self::Engine, children: &C)
        where C: Renderable<Engine = Self::ChildrenEngine>;
}

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

pub trait Renderable {
    type Engine: Engine;

    fn render(&self, eng: &Self::Engine);
}

pub trait View {
    type InputContext: Context;
    type OutputContext: Context;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext;
}

pub trait ViewX {
    type Engine: Engine;
    type ChildrenEngine: Engine = Self::Engine;
    type Renderable: Renderable<Engine = Self::Engine>;

    fn build<C: Renderable<Engine = Self::ChildrenEngine> + 'static>(self, children: Option<C>) -> Self::Renderable;
}

// It will have sense when `specialization` will be ready to use
//
// default impl<T: Component> View for T {
//     type InputContext = DefaultContext;
//     type OutputContext = DefaultContext;
//
//     fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
//         ctx
//     }
// }

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

pub struct Node <E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<Engine = E, ChildrenEngine = CHE>,
        CH: Renderable<Engine = CHE>
{
    item: I,
    children: CH
}

impl<E, I, CHE, CH> Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<Engine = E, ChildrenEngine = CHE>,
        CH: Renderable<Engine = CHE>
{
    pub fn new(item: I, children: CH) -> Self {
        Self {
            item,
            children
        }
    }
}

impl<E, I, CHE, CH> Renderable for Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<Engine = E, ChildrenEngine = CHE>,
        CH: Renderable<Engine = CHE>
{
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        self.item.render_impl(eng, &self.children)
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

impl<E: Engine> Renderable for Box<Renderable<Engine = E>> {
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

// impl <IC, OC, E, CE, T> RenderImplementation for Option<T> 
//     where E:  Engine,
//           CE: Engine,
//           IC: Context,
//           OC: Context,
//           T: RenderImplementation<Engine = E, ChildrenEngine = CE, InputContext = IC, OutputContext = OC>
// {
//     type Engine = E;
//     type ChildrenEngine = CE;
//     type InputContext = IC;
//     type OutputContext = OC;
// }
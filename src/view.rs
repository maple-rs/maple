///
/// 
/// 
/// 
/// 
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

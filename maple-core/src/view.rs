use crate::engine::Engine;
use crate::renderable::Renderable;
use crate::context::Context;
///
/// 
/// 
/// 
/// 
pub trait View<E, CE>
    where E: Engine,
          CE: Engine,
{
    type InputContext: Context;
    type OutputContext: Context;
    type Renderable<T: Renderable<CE> + 'static>: Renderable<E>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext;
    fn build<C: Renderable<CE> + 'static>(self, children: Option<C>) -> Self::Renderable<C>;
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

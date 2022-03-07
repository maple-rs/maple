use super::CanvasContextEngine;
use maple_core::prelude::*;
use maple_stdweb::*;
use maple_macro::view;

impl RenderImplementation<CanvasContextEngine, !> for Circle {
    fn render_impl<C: Renderable<!>>(&self, eng: &CanvasContextEngine, _children: &C) {
        eng.add(&format!("
            ctx.beginPath();
            ctx.arc({},{},{},0,2*Math.PI);
            ctx.stroke();
        ", self.props.cx.unwrap(), self.props.cy.unwrap(), self.props.r.unwrap()));
    }
}

impl View<CanvasContextEngine, !> for Circle {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<!> + 'static> = impl Renderable<CanvasContextEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C: Renderable<!> + 'static>(self, children: Option<C>) -> Self::Renderable<C> {
        Node::new(self, children)
    }
}


impl RenderImplementation<CanvasContextEngine, !> for Rect {
    fn render_impl<C>(&self, eng: &CanvasContextEngine, _children: &C)
        where C: Renderable<!> 
    {
        eng.add(&format!("
            ctx.rect({},{},{},{});
            ctx.stroke();
        ", self.props.x1.unwrap(), self.props.y1.unwrap(), self.props.x2.unwrap(), self.props.y2.unwrap()));
    }
}

impl View<CanvasContextEngine, !> for Rect {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext; 
    type Renderable<C: Renderable<!> + 'static> = impl Renderable<CanvasContextEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<!> + 'static
    {
        Node::new(self, children)
    }
}

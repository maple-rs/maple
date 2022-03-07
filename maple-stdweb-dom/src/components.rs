use super::{HtmlEngine, CanvasContextEngine};
use std::collections::HashMap;

use maple_core::prelude::*;
use maple_stdweb::*;

/** Text **/

impl Renderable<HtmlEngine> for Text {   
    fn render(&self, eng: &HtmlEngine) {
        if let Some(text_ref) = &self.props.text {
            HtmlEngine::text(eng, text_ref);
        }
    }
}

/** Div **/

impl RenderImplementation<HtmlEngine, HtmlEngine> for Div {   
    fn render_impl<C>(&self, eng: &HtmlEngine, children: &C)
        where C: Renderable<HtmlEngine> 
    {
        let mut attrs = HashMap::new();

        if let Some(class) = self.props.class {
            attrs.insert("class".into(), class.into());
        }

        HtmlEngine::open(eng, "div", Some(attrs));
        children.render(eng);
        HtmlEngine::close(eng, "div");
    }
}

impl View<HtmlEngine, HtmlEngine> for Div {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        Node::new(self, children)
    }
}

/** Span **/

impl RenderImplementation<HtmlEngine, HtmlEngine> for Span {   
    fn render_impl<C>(&self, eng: &HtmlEngine, children: &C)
        where C: Renderable<HtmlEngine> 
    {
        HtmlEngine::open(eng, "span", None);
        children.render(eng);
        HtmlEngine::close(eng, "span");
    }
}

impl View<HtmlEngine, HtmlEngine> for Span {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        Node::new(self, children)
    }
}


/** Button **/

impl RenderImplementation<HtmlEngine, HtmlEngine> for Button {    
    fn render_impl<C>(&self, eng: &HtmlEngine, children: &C)
        where C: Renderable<HtmlEngine> 
    {
        HtmlEngine::open(eng, "button", None);
        children.render(eng);
        HtmlEngine::close(eng, "button");
    }
}

impl View<HtmlEngine, HtmlEngine> for Button {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        Node::new(self, children)
    }
}

impl RenderImplementation<HtmlEngine, CanvasContextEngine> for Canvas {   
    fn render_impl<C>(&self, eng: &HtmlEngine, children: &C)
        where C: Renderable<CanvasContextEngine> 
    {
        let can_eng = CanvasContextEngine::new();
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "myCanvas".to_string());

        eng.open("canvas", Some(attrs));
        children.render(&can_eng);
        eng.close("canvas");
        eng.script(&format!("
        var c=document.getElementById(\"myCanvas\");
        var ctx=c.getContext(\"2d\"); {};", can_eng.to_string()));
    }
}

impl View<HtmlEngine, CanvasContextEngine> for Canvas {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<CanvasContextEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<CanvasContextEngine> + 'static
    {
        Node::new(self, children)
    }
}

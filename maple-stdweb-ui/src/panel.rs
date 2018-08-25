use maple_core::prelude::*;
use super::HtmlEngine;
use maple_macro::view;
use maple_stdweb::*;
use maple_stdui::prelude::Panel;

impl View for Panel {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }
}

impl ViewX for Panel {
    type Engine = HtmlEngine;
    type ChildrenEngine = HtmlEngine;
    existential type Renderable: Renderable<Engine = HtmlEngine>;

    fn build<C>(self, children: Option<C>) -> Self::Renderable
        where C: Renderable<Engine = Self::ChildrenEngine> + 'static
    {
        view! {
            <Div class="panel">
                { ...children }
            </Div>
        }
    }
}
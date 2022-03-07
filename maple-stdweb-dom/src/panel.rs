use maple_core::prelude::*;
use maple_macro::view;
use maple_stdweb::*;
use maple_stdui::prelude::Panel;
use super::HtmlEngine;

impl View<HtmlEngine, HtmlEngine> for Panel {
    type InputContext = DefaultContext;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        view! {
            <Div class="panel">
                { ...children }
            </Div>
        }
    }
}

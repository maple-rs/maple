use maple_core::prelude::*;
use maple_macro::view;
use maple_stdweb::*;
use maple_stdui::prelude::tabs::*;
use super::HtmlEngine;


impl View<HtmlEngine, HtmlEngine> for Tabs {
    type InputContext = DefaultContext;
    type OutputContext = TabsContext<DefaultContext>;
    type Renderable<C: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        TabsContext::wrap(ctx)
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        view! {
            <Div class="tabs">
                { ... children }
            </Div>
        }
    }
}

impl View<HtmlEngine, HtmlEngine> for Header {
    type InputContext = TabsContext<DefaultContext>;
    type OutputContext = DefaultContext;
    type Renderable<C: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        self.tabs_ctx = Some(ctx.clone());

        ctx.unwrap()
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        let tabs = self.tabs_ctx.map(|ctx|
            ctx.get_tabs()
                .iter()
                .enumerate()
                .map(|(i, tab)| view! {
                    <Span class="tab-title" click={move |_| HeaderEvent::Click(i)}>
                        {match tab {
                            Some(val) => val,
                            None => "<No title>"
                        }}
                    </Span>
                })
                .collect::<Vec<_>>());

        view! {
            <Div class="tabs-header">
                { ...tabs }
            </Div>
        }
    }
}

impl View<HtmlEngine, HtmlEngine> for Body {
    type InputContext = TabsContext<DefaultContext>;
    type OutputContext = TabsBodyContext<DefaultContext>;
    type Renderable<T: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        TabsBodyContext::wrap(ctx)
    }
 
    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        view! {
            <Div class="tab">
                { ... children }
            </Div>
        }
    }
}

impl View<HtmlEngine, HtmlEngine> for Tab {
    type InputContext = TabsBodyContext<DefaultContext>;
    type OutputContext = DefaultContext;
    type Renderable<T: Renderable<HtmlEngine> + 'static> = impl Renderable<HtmlEngine>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx.add_tab(self.props.title);
        ctx.unwrap().unwrap()
    }

    fn build<C>(self, children: Option<C>) -> Self::Renderable<C>
        where C: Renderable<HtmlEngine> + 'static
    {
        view! {
            <Div class="tab">
                { ... children }
            </Div>
        }
    }
}

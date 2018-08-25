use maple_core::prelude::*;
use maple_macro::view;
use maple_stdweb::*;
use maple_stdui::prelude::tabs::*;
use super::HtmlEngine;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;



impl View for Tabs {
    type InputContext = DefaultContext;
    type OutputContext = TabsContext<DefaultContext>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        TabsContext::wrap(ctx)
    }
}

impl Tabs {
    pub fn build<C>(self, children: Option<C>) -> impl Renderable<Engine = HtmlEngine>
        where C: Renderable<Engine = HtmlEngine> + 'static
    {
        view! {
            <Div class="tabs">
                { ... children }
            </Div>
        }
    }
}

impl View for Header {
    type InputContext = TabsContext<DefaultContext>;
    type OutputContext = DefaultContext;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        self.tabs_ctx = Some(ctx.clone());

        ctx.unwrap()
    }
}

impl Header {
    pub fn build<C>(self, _children: Option<C>) -> impl Renderable<Engine = HtmlEngine>
        where C: Renderable<Engine = HtmlEngine> + 'static
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

impl View for Body {
    type InputContext = TabsContext<DefaultContext>;
    type OutputContext = TabsBodyContext<DefaultContext>;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        TabsBodyContext::wrap(ctx)
    }
}

impl Body {
    pub fn build<C>(self, children: Option<C>) -> impl Renderable<Engine = HtmlEngine>
        where C: Renderable<Engine = HtmlEngine> + 'static
    {
        view! {
            <Div class="tab">
                { ... children }
            </Div>
        }
    }
}

impl View for Tab {
    type InputContext = TabsBodyContext<DefaultContext>;
    type OutputContext = DefaultContext;

    fn receive_context(&mut self, ctx: Self::InputContext) -> Self::OutputContext {
        ctx.add_tab(self.props.title);
        ctx.unwrap().unwrap()
    }
}

impl Tab {
    pub fn build<C>(self, children: Option<C>) -> impl Renderable<Engine = HtmlEngine>
        where C: Renderable<Engine = HtmlEngine> + 'static
    {
        view! {
            <Div class="tab">
                { ... children }
            </Div>
        }
    }
}
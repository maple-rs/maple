use std::ops::Deref;
use maple_core::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct TabsContext<T: Context> { 
    inner: T,
    tabs: Rc<RefCell<Vec<Option<&'static str>>>>
}

impl<T: Context> TabsContext<T> {
    pub fn add_tab(&self, tab_title: Option<&'static str>) {
        self.tabs.borrow_mut().push(tab_title);
    }

    pub fn get_tabs(&self) -> Vec<Option<&'static str>> {
        return self.tabs.borrow().clone()
    }
}

impl<T: Context> Context for TabsContext<T> {
    type Context = T;

    fn unwrap(self) -> Self::Context {
        self.inner
    }

    fn wrap(ctx: Self::Context) -> Self {
        Self {
            inner: ctx,
            tabs: Default::default()
        }
    }
}

#[derive(Clone)]
pub struct TabsBodyContext<T: Context> { inner: TabsContext<T> }

impl<T: Context> Context for TabsBodyContext<T> {
    type Context = TabsContext<T>;

    fn unwrap(self) -> Self::Context {
        self.inner
    }

    fn wrap(ctx: Self::Context) -> Self {
        Self {
            inner: ctx
        }
    }
}

impl<T: Context> Deref for TabsBodyContext<T> {
    type Target = TabsContext<T>;

    fn deref(&self) -> &Self::Target {
        return &self.inner;
    }
}

pub struct Tabs {
    props: TabsProps
}

pub enum TabsEvent {
    TabChange(usize)
}

#[derive(Default)]
pub struct TabsProps {
    pub on_tab_change: Option<Box<dyn Callback<Arg=usize>>>
}

impl Component for Tabs {
    type Props = TabsProps;
    type Msg = TabsEvent;

    fn create(props: TabsProps) -> Self {
        Tabs { props }
    }

    fn update(&mut self, msg: Self::Msg) -> bool {
        match msg {
            TabsEvent::TabChange(idx) => {
                if let Some(cb) = &self.props.on_tab_change {
                    cb.call(idx);
                }
            }
        };
        
        false
    }
}

pub struct Header {
    pub props: HeaderProps,
    pub tabs_ctx: Option<TabsContext<DefaultContext>>
}

pub enum HeaderEvent {
    Click(usize)
}

impl ComponentMsg for HeaderEvent {
    type Component = Header;
}

#[derive(Default)]
pub struct HeaderProps {
    pub on_click: Option<Box<dyn Callback<Arg=usize>>>
}

impl Component for Header {
    type Props = HeaderProps;
    type Msg = HeaderEvent;

    fn create(props: HeaderProps) -> Self {
        Header { 
            props,
            tabs_ctx: Default::default()
        }
    }
    
    fn update(&mut self, msg: Self::Msg) -> bool {
        match msg {
            HeaderEvent::Click(idx) => {
                if let Some(cb) = &self.props.on_click {
                    cb.call(idx);
                }
            }
        };
        
        false
    }
}

pub struct Body {
    pub props: BodyProps
}

#[derive(Default)]
pub struct BodyProps {}

impl Component for Body {
    type Props = BodyProps;

    fn create(props: BodyProps) -> Self {
        Body { props }
    }
}

pub struct Tab {
   pub props: TabProps
}

#[derive(Default)]
pub struct TabProps {
    pub title: Option<&'static str>
}

impl Component for Tab {
    type Props = TabProps;

    fn create(props: TabProps) -> Self {
        Tab { props }
    }
}
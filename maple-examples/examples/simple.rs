#![feature(proc_macro_non_items)]
#![allow(dead_code)]

#[macro_use]
extern crate maple;
extern crate maple_stdweb_dom;

macro_rules! children {
    ($w:expr) => ($w);
    ($w1:expr, $($rest:tt)*) => (Children2::new($w1, children!($($rest)*)));
}

use maple::prelude::*;

use std::default::Default;
use maple::prelude::tabs::*;
use maple::prelude::prelude::Panel;

use maple_stdweb_dom::*;

fn get_rest(count: usize) -> impl Renderable<HtmlEngine> {
    let mut vec = Vec::new();

    for i in 0..count {
        vec.push(view! { <Span> "Number " {format!("{}", i + 1)} </Span> });
    }

    return vec;
}

fn main() {
    let x = view! {
        <Div>
            <Div>
                <Canvas>
                    <Circle cx={95} cy={70} r={20} />
                    <Rect x1={20} y1={20} x2={150} y2={100} />
                </Canvas>
            </Div>
            <Panel>
                <Tabs on_tab_change={|idx| println!("Tab has changed! New tab idx {}", idx)}>
                    <Header />
                    <Body>
                        <Tab title="Tab 1">
                            <Span>"Tab 1 Content"</Span>
                        </Tab>
   
                        <Tab title="Tab 2">
                            <Button>"But in Tab " {1 + 1} </Button>
                        </Tab>

                        <Tab title="Tab 3">
                            { ... get_rest(5) }
                        </Tab>
                    </Body>
                </Tabs>
            </Panel>
        </Div>
    };

    let eng = HtmlEngine::new();

    x.render(&eng);

    println!("{}", eng.to_string());
}

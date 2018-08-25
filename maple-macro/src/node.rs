use syn::synom::Synom;
use syn::{LitStr, Expr};
use crate::tag::Tag;


#[derive(Debug)]
crate enum Node {
    Tag(Tag),
    Text(LitStr),
    Expr(Expr),
    Embed(Expr)
}

impl Synom for Node {
    named!(parse -> Self, do_parse!(
        value: alt!(
            syn!(Tag) => { |tag| Node::Tag(tag) }
            |
            braces!(
                do_parse!(
                        punct!(.) >> 
                        punct!(.) >> 
                        punct!(.) >> 
                    tt: syn!(Expr) >> (tt)
                )) => { |d| Node::Embed(d.1) }
            |
            braces!(syn!(Expr)) => { |d| Node::Expr(d.1) }
            |
            syn!(LitStr) => { |_str| Node::Text(_str) }
        ) >>

        (value)
    ));
}
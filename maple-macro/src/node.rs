use crate::tag::Tag;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
crate enum Node {
    Tag(Tag),
    Text(syn::LitStr),
    Expr(syn::Expr),
    Embed(syn::Expr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Node::Text(input.parse()?))
        } else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);

            Ok(if content.peek(Token![...]) {
                content.parse::<Token![...]>()?;
                let expr = content.parse::<syn::Expr>()?;

                Node::Embed(expr)
            } else {
                let expr = content.parse::<syn::Expr>()?;

                Node::Expr(expr)
            })
        } else {
            Ok(Node::Tag(input.parse()?))
        }
    }
}

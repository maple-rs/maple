use crate::attribute::Attribute;
use crate::node::Node;
use syn::TypePath;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
crate struct Tag {
    pub path: TypePath,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Node>
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs: Vec<Attribute> = Vec::new();
        let mut children: Vec<Node> = Vec::new();
        let mut open_path: TypePath;
        let mut close_path: TypePath;

        // parsing opening "<"
        input.parse::<Token![<]>()?;

        // parsing tag name
        open_path = input.parse()?;

        // parsing attributes
        while input.peek(syn::Ident) {
            attrs.push(input.parse()?);
        }

        if input.peek(Token![/]) && input.peek2(Token![>]) {
            // parsing closing "/>"
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;

            close_path = open_path.clone();
        } else {
            // parsing closing ">"
            input.parse::<Token![>]>()?;

            // parsing children
            while !(input.peek(Token![<]) && input.peek2(Token![/])) {
                children.push(input.parse()?);
            }

            // parsing closing "</{tag path}>"
            input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;
            close_path = input.parse()?;
            input.parse::<Token![>]>()?;
        }

        if close_path == open_path {
            Ok(Tag {
                path: open_path,
                attributes: attrs,
                body: children,
            })
        } else {
            Err(input.error("open and close paths shoud match"))
        }
    }
}
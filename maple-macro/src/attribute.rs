use syn::parse::{Parse, ParseStream, Result};
use quote::ToTokens;

#[derive(Debug)]
crate struct Attribute {
    pub name: syn::Ident,
        eq: Token![=],
    pub value: AttributeValue
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name:  input.parse()?,
            eq:    input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum AttributeValue {
    Expr(syn::Expr),
    Literal(syn::Lit)
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Lit) {
            Ok(AttributeValue::Literal(input.parse()?))
        } else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            
            Ok(AttributeValue::Expr(content.parse()?))
        } else {
            Err(input.error("Expected a literal or { expression }"))
        }
    }
}

impl ToTokens for AttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttributeValue::Literal(s) => s.to_tokens(tokens),
            AttributeValue::Expr(e) => e.to_tokens(tokens)
        };
    }
}
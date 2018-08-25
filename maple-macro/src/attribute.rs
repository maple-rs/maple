use syn::{Expr, ExprLit, Ident};
use syn::synom::Synom;
use quote::ToTokens;

#[derive(Debug)]
crate struct Attribute {
    pub name: Ident,
    pub value: AttributeValue
}

impl Synom for Attribute {
    named!(parse -> Self, do_parse!(
        name:  syn!(Ident)          >>
               punct!(=)            >>
        value: syn!(AttributeValue) >>

        (Attribute { name, value })
    ));
}

#[derive(Debug)]
crate enum AttributeValue {
    Expr(Expr),
    Literal(ExprLit)
}

impl Synom for AttributeValue {
    named!(parse -> Self, do_parse!(
        value: alt!(
            syn!(ExprLit) => { |d| AttributeValue::Literal(d) }
            |
            braces!(syn!(Expr)) => { |d| AttributeValue::Expr(d.1) }
        ) >>

        (value)
    ));
}

impl ToTokens for AttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttributeValue::Literal(s) => s.to_tokens(tokens),
            AttributeValue::Expr(e) => e.to_tokens(tokens)
        };
    }
}
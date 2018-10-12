 #![feature(crate_visibility_modifier)]

mod attribute;
mod node;
mod tag; 

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;
extern crate proc_macro;

use self::proc_macro::{TokenStream};
use syn::{TypePath, Ident, Expr};
use quote::*;

use self::node::Node;
use self::attribute::{Attribute, AttributeValue};
use self::tag::Tag;

fn visit_attribute(ident: &Ident, attr: &Attribute, _ctx: &TypePath) -> impl ToTokens {
    let attr_name = &attr.name;
    let attr_value = &attr.value;

    match attr_value {
        AttributeValue::Expr(Expr::Closure(closure)) => quote! {
            #ident.#attr_name = Callback1::new({#closure}).my_into();
        },
        _ => quote! { #ident.#attr_name = {#attr_value}.my_into(); }
    }
}

fn visit_node(parent: &str, idx: &mut u32, node: &Node, stream: &mut proc_macro2::TokenStream, thread: &mut proc_macro2::TokenStream) {
    let ii_name = &("p".to_string() + &idx.to_string());
    let ii = Ident::new(ii_name, proc_macro2::Span::call_site());
    let ctx_ii = Ident::new(&("ctx_".to_string() + ii_name), proc_macro2::Span::call_site());
    let comp_ii = Ident::new(&("comp_".to_string() + ii_name), proc_macro2::Span::call_site());
    let pp = Ident::new(&("ctx_".to_string() + parent), proc_macro2::Span::call_site());

    match node {
        Node::Text(txt) => {
            let toks = quote!{
                let #ii = Text::create(<Text as Component>::Props::new(#txt));
            };
            toks.to_tokens(stream);
        },

        Node::Expr(exr) => {
            let toks = quote!{
                let #ii = Text::create(<Text as Component>::Props::new(format!("{:?}", #exr)));
            };
            toks.to_tokens(stream);
        },

        Node::Embed(exr) => {
            let toks = quote!{
                let #ii = #exr;
            };
            toks.to_tokens(stream);
        },

        Node::Tag(tag) => {
            let iip = Ident::new(&(ii_name.to_string() + "_props"), proc_macro2::Span::call_site());
            let path = &tag.path;
            
            #[allow(unused_mut)]
            let mut children = Vec::new();

            let attributes = tag.attributes
                .iter()
                .map(|attr| visit_attribute(&iip, &attr, &path));

            let toks = quote!{
                #[allow(unused_mut)]
                let mut #iip = <#path as Component>::Props::default();
                #(#attributes;)*
                let mut #comp_ii = #path::create(#iip);
                #[allow(unused_variables)]
                let #ctx_ii = #comp_ii.receive_context(#pp.clone());
            };

            toks.to_tokens(stream);

            for child in tag.body.iter() {
                *idx += 1;

                let ch_name = "p".to_string() + &idx.to_string();
                let ch_ii = Ident::new(&ch_name, proc_macro2::Span::call_site());
                
                children.push(ch_ii);

                visit_node(ii_name, idx, child, stream, thread);
            }

            let toks = if children.len() > 0 {
                quote!{
                    let #ii = #comp_ii.build(Some(children![#(#children),*]));
                }
            } else {
                quote!{
                    let #ii = #comp_ii.build::<Stub<_>>(None);
                }
            };

            toks.to_tokens(thread);
        }
    };
}

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let toks = match syn::parse(input) {
        Ok(root) => {
            let mut output = proc_macro2::TokenStream::new();
            let mut thread = proc_macro2::TokenStream::new();

            let mut idx = 0;

            visit_node("default", &mut idx, &root, &mut output, &mut thread);

            quote! {{
                let ctx_default = DefaultContext;
                #output
                #thread
                p0
            }}
        }

        Err(err) => err.to_compile_error()
    };

    toks.into()
}
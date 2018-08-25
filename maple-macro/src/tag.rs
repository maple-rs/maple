use crate::attribute::Attribute;
use crate::node::Node;
use syn::TypePath;
use syn::synom::Synom;

#[derive(Debug)]
crate struct Tag {
    pub path: TypePath,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Node>
}

impl Tag {
    pub fn new(open: TypePath, close: Option<TypePath>, attrs: Vec<Attribute>, body: Vec<Node>) -> Result<Self, String> {
        if let Some(close_ref) = &close {
            if &open != close_ref {
                return Err(format!("Open({}) and closing({}) tags are not matching!", quote!{#open}, quote!{#close_ref}))
            }
        }

        Ok(Tag {
            path: open,
            attributes: attrs,
            body
        })
    }
}

impl Synom for Tag {
    named!(parse -> Self, do_parse!(
        t: alt!(
            do_parse!(
                        punct!(<)                  >>
                open:   syn!(TypePath)             >>
                attrs:  many0!(syn!(Attribute))    >>
                        punct!(>)                  >>
                body:   many0!(syn!(Node))         >>
                        punct!(<) >> punct!(/)     >>
                close:  syn!(TypePath)             >>
                        punct!(>)                  >>
                        
                        (match Tag::new(open, Some(close), attrs, body) {
                            Ok(v) => v,
                            Err(e) => panic!("{:?}", e)
                        })
            ) => {|x|x}
            |
            do_parse!(
                    punct!(<)                  >>
            open:   syn!(TypePath)             >>
            attrs:  many0!(syn!(Attribute))    >>
                    punct!(/)                  >> 
                    punct!(>)                  >>
                    (Tag::new(open, None, attrs, vec![]).unwrap())
            ) => {|x|x}
        ) >> (t)
    ));
}
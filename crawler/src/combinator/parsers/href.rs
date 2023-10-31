use std::collections::hash_map::Entry;

use crate::combinator::{DomParser, DomParserContext, NestedAttr};

pub struct HrefParser {}

impl HrefParser {
    pub fn new() -> Self {
        Self {}
    }
    // FIX: Proper URL building
    fn append_domain(href: &str, ctx: &DomParserContext) -> String {
        if href.contains("https://")
            || href.contains("mailto:")
            || href.contains("www.")
            || href.contains("http://")
        {
            return String::from(href);
        }
        String::from(&ctx.current_domain) + "/" + href
    }
}

impl DomParser for HrefParser {
    fn parse<'a>(
        &self,
        dom: &'a markup5ever_rcdom::Handle,
        context: &'a mut crate::combinator::DomParserContext,
    ) -> anyhow::Result<(
        &'a markup5ever_rcdom::Handle,
        std::vec::Vec<crate::combinator::Suggestion>,
        &'a mut crate::combinator::DomParserContext,
    )> {
        if let markup5ever_rcdom::NodeData::Element { ref attrs, .. } = dom.data {
            for attr in attrs.borrow().iter() {
                let attr_name = &attr.name.local;
                if attr_name == "href" {
                    let href = HrefParser::append_domain(&attr.value, context);
                    match context.attrs.entry("HrefParser".to_string()) {
                        Entry::Vacant(e) => {
                            e.insert(NestedAttr::List(vec![href]));
                        }
                        Entry::Occupied(mut e) => {
                            match e.get_mut() {
                                NestedAttr::List(v) => {
                                    v.push(href);
                                }
                                _ => anyhow::bail!("HrefParser: Unexpected NestedAttr"),
                            };
                        }
                    };
                }
            }
            Ok((dom, vec![], context))
        } else {
            anyhow::bail!("HrefParser: Unexpected Element")
        }
    }
}

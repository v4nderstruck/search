pub mod core;
mod parsers;

#[derive(Debug)]
pub enum NestedAttr {
    Attr(String),
    List(Vec<String>),
    Nested(std::collections::HashMap<String, NestedAttr>),
}

pub enum Suggestion<'a> {
    Parse(Box<dyn DomParser>, &'a markup5ever_rcdom::Handle),
    Crawl(String),
}

#[derive(Debug)]
pub struct DomParserContext {
    pub attrs: std::collections::HashMap<String, NestedAttr>,
    pub current_domain: String,
}

impl DomParserContext {
    pub fn new(domain: String) -> Self {
        return Self {
            attrs: std::collections::HashMap::new(),
            current_domain: domain,
        };
    }
}

pub trait DomParser {
    fn parse<'a>(
        &self,
        dom: &'a markup5ever_rcdom::Handle,
        context: &'a mut DomParserContext,
    ) -> anyhow::Result<(
        &'a markup5ever_rcdom::Handle,
        std::vec::Vec<Suggestion>,
        &'a mut DomParserContext,
    )>;
}

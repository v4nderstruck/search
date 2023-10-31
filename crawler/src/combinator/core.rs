use markup5ever::QualName;
use markup5ever_rcdom::NodeData;

use super::{parsers::href::HrefParser, DomParser};

#[derive(Debug, Eq, PartialEq, Hash)]
enum DomWalkEventsElementSelector {
    Any,
    Local(String),
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum DomWalkEvents {
    Element(DomWalkEventsElementSelector),
    Text,
}

pub struct CoreParser {
    subscriptions: std::collections::HashMap<DomWalkEvents, Vec<Box<dyn DomParser>>>,
}

impl Default for CoreParser {
    fn default() -> Self {
        let mut core = Self::new();
        core.register(
            DomWalkEvents::Element(DomWalkEventsElementSelector::Any),
            Box::new(HrefParser::new()),
        );
        core
    }
}

impl CoreParser {
    pub fn new() -> Self {
        Self {
            subscriptions: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, event: DomWalkEvents, parser: Box<dyn DomParser>) {
        match self.subscriptions.entry(event) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(vec![parser]);
            }
            std::collections::hash_map::Entry::Occupied(mut e) => {
                e.get_mut().push(parser);
            }
        }
    }
}

impl DomParser for CoreParser {
    fn parse<'a>(
        &self,
        dom: &'a markup5ever_rcdom::Handle,
        context: &'a mut super::DomParserContext,
    ) -> anyhow::Result<(
        &'a markup5ever_rcdom::Handle,
        std::vec::Vec<super::Suggestion>,
        &'a mut super::DomParserContext,
    )> {
        match dom.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if let Some(subs) = self
                    .subscriptions
                    .get(&DomWalkEvents::Element(DomWalkEventsElementSelector::Any))
                {
                    for sub in subs {
                        let _ = sub.parse(dom, context)?;
                    }
                }
            }
            NodeData::Text { ref contents } => {
                println!("text {}", contents.borrow().escape_default());
            }
            _ => {}
        }
        for child in dom.children.borrow().iter() {
            self.parse(child, context)?;
        }
        Ok((dom, vec![], context))
    }
}

mod combinator;
use combinator::DomParser;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

async fn request_body(url: &str) -> anyhow::Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn parse_body(html_str: &str, url: String) -> anyhow::Result<()> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html_str.as_bytes())?;

    let core = combinator::core::CoreParser::default();
    let mut context = combinator::DomParserContext::new(url);
    let _ = core.parse(&dom.document, &mut context)?;
    println!("{:?}", context.attrs);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Some(arg1) = std::env::args().nth(1) {
        println!("Crawl {} ", arg1);
        let body = request_body(&arg1).await?;
        parse_body(&body, arg1).unwrap();
    }
    Ok(())
}

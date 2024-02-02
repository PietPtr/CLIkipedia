mod parser;
mod wikipedia;

use parser::HtmlParser;
use std::error::Error;
use wikipedia::Wikipedia;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wikipedia = Wikipedia::new();
    let html = wikipedia.random_page().await?;
    dbg!(HtmlParser::parse_page(html));

    Ok(())
}

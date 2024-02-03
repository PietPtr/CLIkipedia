mod parser;
mod wikipedia;

use parser::HtmlParser;
use std::{error::Error, fs};
use wikipedia::Wikipedia;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wikipedia = Wikipedia::new();
    let html = wikipedia.random_page().await?;
    println!("{}", &html);
    return Ok(());

    let filename = "test.html";
    match fs::read_to_string(filename) {
        Ok(contents) => {
            dbg!(HtmlParser::parse_page(contents));
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }

    Ok(())
}

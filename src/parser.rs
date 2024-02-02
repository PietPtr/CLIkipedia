use scraper::{Html, Selector};

pub struct HtmlParser {}

#[derive(Debug)]
pub enum ParagraphElement {
    Text(String),
    Link(String),
}

type Paragraph = Vec<ParagraphElement>;

#[derive(Debug)]
pub struct Page {
    title: String,
    paragraphs: Vec<Paragraph>,
}

impl HtmlParser {
    pub fn parse_page(html: String) -> Page {
        let document = Html::parse_document(&html);
        let selector = Selector::parse("p").unwrap();

        let mut paragraphs = Vec::new();

        for element in document.select(&selector) {
            // for node in element.children() {
            //     match node.value() {
            //         scraper::node::Node::Element(element_ref) => {
            //             println!("HTML Element: <{}>", element_ref.name());
            //         }
            //         scraper::node::Node::Text(text) => {
            //             println!("Text node: {}", text.text);
            //         }
            //         scraper::node::Node::Comment(comment) => {
            //             println!("Comment: {:?}", comment);
            //         }
            //         _ => (),
            //     };
            // }
            // println!("{}",);
            paragraphs.push(vec![ParagraphElement::Text(
                element.text().collect::<Vec<_>>().join(""),
            )]);
        }

        let title_selector = Selector::parse("title").unwrap();

        let title = if let Some(title_element) = document.select(&title_selector).next() {
            title_element.text().collect::<Vec<_>>().join("")
        } else {
            "-".to_string()
        };

        Page { title, paragraphs }
    }
}

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
};
use scraper::{Html, Node, Selector};
use std::fmt;

pub struct HtmlParser {}

#[derive(Debug, Clone)]
pub struct Link {
    pub link: String,
    pub text: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]({})", self.text, self.link)
    }
}

#[derive(Debug, Clone)]
pub enum ParagraphElement {
    Text(String),
    Link(Link),
}

impl fmt::Display for ParagraphElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParagraphElement::Text(text) => write!(f, "{}", text),
            ParagraphElement::Link(link) => write!(f, "{}", link),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub elems: Vec<ParagraphElement>,
}

impl Paragraph {
    fn new() -> Self {
        Self { elems: vec![] }
    }

    fn push(&mut self, elem: ParagraphElement) {
        self.elems.push(elem)
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.elems {
            match elem {
                ParagraphElement::Text(text) => write!(f, "{}", text)?,
                ParagraphElement::Link(link) => write!(f, "{}", link)?,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Page {
    pub title: String,
    pub paragraphs: Vec<Paragraph>,
}

impl HtmlParser {
    pub fn parse_page(html: &String) -> Page {
        let document = Html::parse_document(html);
        let selector = Selector::parse("p").unwrap();

        let mut paragraphs = Vec::new();

        for element in document.select(&selector) {
            let mut paragraph = Paragraph::new();

            for node in element.children() {
                match node.value() {
                    scraper::node::Node::Element(element_ref) => {
                        let mut push_text_node = || {
                            if let Some(first_child) = node.first_child() {
                                paragraph.push(ParagraphElement::Text(
                                    match first_child.value().as_text() {
                                        None => "".to_string(),
                                        Some(text) => text.to_string(),
                                    },
                                ));
                            }
                        };

                        match element_ref.name().to_string().as_str() {
                            "span" => push_text_node(),
                            "b" => push_text_node(),
                            "i" => push_text_node(),
                            "a" => {
                                paragraph.push(ParagraphElement::Link(Link {
                                    link: element_ref.attr("href").unwrap().to_string(),
                                    text: node
                                        .first_child()
                                        .unwrap()
                                        .value()
                                        .as_text()
                                        .unwrap()
                                        .to_string(),
                                }));
                            }
                            _ => {}
                        }
                    }
                    scraper::node::Node::Text(text) => {
                        paragraph.push(ParagraphElement::Text(text.text.to_string()));
                    }
                    _ => (),
                };
            }
            paragraphs.push(paragraph);
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

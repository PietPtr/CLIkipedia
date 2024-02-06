use std::{
    collections::HashMap,
    error::{self, Error},
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ScrollbarState,
};

use crate::{
    flog,
    parser::{HtmlParser, Link, Paragraph, ParagraphElement},
    util::{base26_to_usize, usize_to_base26},
    wikipedia::Wikipedia,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    // TODO: make a struct for pages with title, paragraphs, and links.
    pub paragraphs: Vec<Paragraph>,
    pub page_title: String,
    links: HashMap<String, Link>,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub frame_size: Rect,
    page_content_length: usize,
    wikipedia: Wikipedia,
    pub selector: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            paragraphs: vec![],
            page_title: "".to_string(),
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            frame_size: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            page_content_length: 0,
            wikipedia: Wikipedia::new(),
            selector: String::new(),
            links: HashMap::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        if self.page_title.len() == 0 {
            // TODO: put page struct (see other todo) in an optional
            self.new_page().await
        } else {
            Ok(())
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // self.page_title = format!("{:?}", self.page_content_length);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.frame_size.width = width;
        self.frame_size.height = height;
    }

    pub fn link_select(&mut self, c: char) {
        if self.link_selector_exists() | self.selector.is_empty() {
            self.selector.push(c);
        }
    }

    pub fn delete_link_selector(&mut self) {
        self.selector.pop();
    }

    pub async fn go_to_selected_link(&mut self) {
        if let Some(link) = self.links.get(&self.selector) {
            self.selector = String::new();
            // TODO: this await blocks the whole app, should not be awaited but there should be some sort of callback and some state and a loading icon
            let html = self.wikipedia.get_page(&link.link).await;
            match html {
                Ok(html) => self.set_html(&html),
                Err(_) => todo!(),
            }
        }
    }

    pub fn link_selector_exists(&self) -> bool {
        self.links.get(&self.selector).is_some()
    }

    pub fn scroll(&mut self, key: KeyCode) {
        enum Direction {
            Up,
            Down,
        }

        let (amount, direction) = match key {
            KeyCode::Home => (usize::MAX, Direction::Up),
            KeyCode::End => (usize::MAX, Direction::Down),
            KeyCode::Up => (1, Direction::Up),
            KeyCode::Down => (1, Direction::Down),
            KeyCode::PageUp => (self.frame_size.height as usize - 2, Direction::Up),
            KeyCode::PageDown => (self.frame_size.height as usize - 2, Direction::Down),
            _ => panic!("Passed non-scroll key into scroll function."),
        };

        match direction {
            Direction::Down => {
                self.vertical_scroll = (self.vertical_scroll + amount).min(
                    self.page_content_length.saturating_sub(5),
                    // .saturating_sub(self.frame_size.height as usize  / 2),
                )
            }
            Direction::Up => self.vertical_scroll = self.vertical_scroll.saturating_sub(amount),
        }

        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn set_scroll_params(&mut self, length: usize) {
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(length);
        self.page_content_length = length;
    }

    pub fn set_html(&mut self, html: &str) {
        let page = HtmlParser::parse_page(html);

        self.page_title = page.title;
        self.paragraphs = page.paragraphs.clone();

        let mut num_links = 0;
        self.links.clear();
        for p in &self.paragraphs {
            for e in &p.elems {
                match e {
                    ParagraphElement::Link(link) => {
                        self.links.insert(usize_to_base26(num_links), link.clone());
                        num_links += 1;
                    }
                    _ => (),
                }
            }
        }
        flog!(self.links);

        self.vertical_scroll = 0;
        self.vertical_scroll_state = ScrollbarState::default();
    }

    pub async fn new_page(&mut self) -> Result<(), Box<dyn Error>> {
        let html = self.wikipedia.random_page().await?;
        self.set_html(&html);

        let path_str = &format!("htmls/{}.html", self.page_title);
        let path = Path::new(path_str.as_str());
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    pub fn get_text(&self) -> Vec<Line<'_>> {
        let mut link_counter = 0;
        let mut lines = vec![];
        for paragraph in &self.paragraphs {
            // let mut line_vec = vec![Span::raw(format!("{:?}: ", paragraph.elems))];
            let mut line_vec = vec![];
            for elem in &paragraph.elems {
                match elem {
                    ParagraphElement::Text(text, false) => line_vec.push(Span::raw(text)),
                    ParagraphElement::Text(text, true) => line_vec.push(Span::raw(text).italic()),
                    ParagraphElement::Link(Link { link: _, text }) => {
                        // TODO: if link counter = to_usize(selector) then this is selected
                        let selected = !self.selector.is_empty()
                            && link_counter == base26_to_usize(&self.selector);
                        let mut style = Style::default();
                        if selected {
                            style = style.bg(Color::Blue).fg(Color::White);
                        } else {
                            style = style.fg(Color::Blue).underlined();
                        }
                        let link = Span::styled(text, style);
                        line_vec.push(link);
                        line_vec.append(&mut self.format_link_ref(link_counter, style));
                        link_counter += 1;
                    }
                };
            }
            if !line_vec.is_empty() {
                lines.push(Line::from(line_vec));
                lines.push(Line::from(vec![]));
            }
        }
        lines.clone()
    }

    fn format_link_ref(&self, link_counter: usize, style: Style) -> Vec<Span> {
        vec![Span::styled(
            format!("[{}]", usize_to_base26(link_counter)),
            style,
        )]
    }
}

use std::{
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
    parser::{HtmlParser, Link, Paragraph, ParagraphElement},
    wikipedia::Wikipedia,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub paragraphs: Vec<Paragraph>,
    pub page_title: String,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub frame_size: Rect,
    page_content_length: usize,
    wikipedia: Wikipedia,
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
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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

    pub fn scroll(&mut self, key: KeyCode) {
        enum Direction {
            Up,
            Down,
        }

        let (amount, direction) = match key {
            KeyCode::Up => (1, Direction::Up),
            KeyCode::Down => (1, Direction::Down),
            KeyCode::PageUp => (self.frame_size.height as usize - 2, Direction::Up),
            KeyCode::PageDown => (self.frame_size.height as usize - 2, Direction::Down),
            _ => panic!("Passed non-scroll key into scroll function."),
        };

        match direction {
            Direction::Down => {
                self.vertical_scroll =
                    (self.vertical_scroll + amount).min(self.page_content_length - 1)
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
        let page = HtmlParser::parse_page(&html);

        self.page_title = page.title;
        self.paragraphs = page.paragraphs.clone();

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
        let mut lines = vec![];
        for paragraph in &self.paragraphs {
            let mut line_vec = vec![];
            for elem in &paragraph.elems {
                let span = match elem {
                    ParagraphElement::Text(text, false) => Span::raw(text),
                    ParagraphElement::Text(text, true) => Span::raw(text).italic(),
                    ParagraphElement::Link(Link { link: _, text }) => {
                        Span::styled(text, Style::default().fg(Color::Blue).underlined())
                    }
                };
                line_vec.push(span);
            }
            lines.push(Line::from(line_vec));
        }
        lines.clone()
    }
}

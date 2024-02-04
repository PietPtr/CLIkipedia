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

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const TEST_STRING: &str = "The [mass](./Mass) of a long gun is usually greater than that of a handgun, making the long gun more expensive to transport, and more difficult and tiring to carry. The increased [moment of inertia](./Moment_of_inertia) makes the long gun slower and more difficult to [traverse and elevate](./Gun_laying), and it is thus slower and more difficult to adjust the aim. However, this also results in greater [stability](./Directional_stability) in aiming. The greater amount of material in a long gun tends to make it more expensive to manufacture, other factors being equal. The greater size makes it more difficult to conceal, and more inconvenient to use in confined quarters, as well as requiring larger storage space.\nAs long guns include a stock that is braced against the shoulder, the [recoil](./Recoil) when firing is transferred directly into the body of the user. This allows better control of aim than handguns, which do not include stock, and thus all their recoil must be transferred to the arms of the user. It also makes it possible to manage larger amounts of recoil without damage or loss of control; in combination with the higher mass of long guns, this means more [propellant](./Propellant) (such as [gunpowder](./Gunpowder)) can be used and thus larger projectiles can be fired at higher [velocities](./Velocity). This is one of the main reasons for the use of long guns over handguns—faster or heavier projectiles help with penetration and accuracy over longer distances.\n[Shotguns](./Shotguns) are long guns that are designed to fire many small projectiles at once. This makes them very effective at close ranges, but with diminished usefulness at long ranges, even with [shotgun slugs](./Shotgun_slugs) they are mostly only effective to about 100 yd (91 m).\nIn historical [navy](./Navy) usage, a long gun was the standard type of [cannon](./Cannon) mounted by a sailing vessel, called such to distinguish it from the much shorter [carronades](./Carronade). In informal usage, the length was combined with the weight of the shot, yielding terms like long 9s, referring to full-length cannons firing a 9-pound round shot.\nThe [mass](./Mass) of a long gun is usually greater than that of a handgun, making the long gun more expensive to transport, and more difficult and tiring to carry. The increased [moment of inertia](./Moment_of_inertia) makes the long gun slower and more difficult to [traverse and elevate](./Gun_laying), and it is thus slower and more difficult to adjust the aim. However, this also results in greater [stability](./Directional_stability) in aiming. The greater amount of material in a long gun tends to make it more expensive to manufacture, other factors being equal. The greater size makes it more difficult to conceal, and more inconvenient to use in confined quarters, as well as requiring larger storage space.\nAs long guns include a stock that is braced against the shoulder, the [recoil](./Recoil) when firing is transferred directly into the body of the user. This allows better control of aim than handguns, which do not include stock, and thus all their recoil must be transferred to the arms of the user. It also makes it possible to manage larger amounts of recoil without damage or loss of control; in combination with the higher mass of long guns, this means more [propellant](./Propellant) (such as [gunpowder](./Gunpowder)) can be used and thus larger projectiles can be fired at higher [velocities](./Velocity). This is one of the main reasons for the use of long guns over handguns—faster or heavier projectiles help with penetration and accuracy over longer distances.\n[Shotguns](./Shotguns) are long guns that are designed to fire many small projectiles at once. This makes them very effective at close ranges, but with diminished usefulness at long ranges, even with [shotgun slugs](./Shotgun_slugs) they are mostly only effective to about 100 yd (91 m).\nIn historical [navy](./Navy) usage, a long gun was the standard type of [cannon](./Cannon) mounted by a sailing vessel, called such to distinguish it from the much shorter [carronades](./Carronade). In informal usage, the length was combined with the weight of the shot, yielding terms like long 9s, referring to full-length cannons firing a 9-pound round shot.";

/// Application.
pub struct App {
    pub running: bool,
    pub paragraphs: Vec<Paragraph>,
    pub page_title: String,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub frame_size: Rect,
    wikipedia: Wikipedia,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            paragraphs: vec![],
            page_title: "title".to_string(),
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            frame_size: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            wikipedia: Wikipedia::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    fn calculate_wrapped_lines(&self, max_width: usize) -> usize {
        0
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.vertical_scroll_state = self
            .vertical_scroll_state
            .content_length(self.calculate_wrapped_lines(self.frame_size.width as usize));
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
                self.vertical_scroll = (self.vertical_scroll + amount)
                    .min(self.calculate_wrapped_lines(self.frame_size.width as usize))
            }
            Direction::Up => self.vertical_scroll = self.vertical_scroll.saturating_sub(amount),
        }

        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub async fn new_page(&mut self) -> Result<(), Box<dyn Error>> {
        let html = self.wikipedia.random_page().await?;
        let page = HtmlParser::parse_page(&html);

        let path_str = &format!("htmls/{}.html", page.title);
        let path = Path::new(path_str.as_str());
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        file.write_all(&html.as_bytes())?;

        self.page_title = page.title;

        self.paragraphs = page.paragraphs.clone();

        self.vertical_scroll = 0;
        self.vertical_scroll_state = ScrollbarState::default();

        Ok(())
    }

    pub fn get_text<'a>(&'a self) -> Vec<Line<'a>> {
        let mut lines = vec![];
        for paragraph in &self.paragraphs {
            let mut line_vec = vec![];
            for elem in &paragraph.elems {
                let span = match elem {
                    ParagraphElement::Text(text) => Span::raw(text),
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

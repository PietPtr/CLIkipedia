use clap::{Arg, Command};
use clikipedia_tui::app::{App, AppResult};
use clikipedia_tui::event::{Event, EventHandler};
use clikipedia_tui::handler::{handle_key_events, handle_mouse_events};
use clikipedia_tui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{fs, io};

#[tokio::main]
async fn main() -> AppResult<()> {
    let matches = Command::new("clikipedia")
        .arg(
            Arg::new("html")
                .long("html")
                .value_name("FILE")
                .help("Views a given HTML file")
                .conflicts_with("page"),
        )
        .arg(
            Arg::new("page")
                .long("page")
                .value_name("STRING")
                .help("Title of a page to look up")
                .conflicts_with("html"),
        )
        .get_matches();

    let mut app = App::new();

    if let Some(html_path) = matches.get_one::<String>("html") {
        if let Ok(html) = fs::read_to_string(html_path) {
            app.set_html(&html);
        } else {
            println!("Cannot read file {}", html_path);
            return Ok(());
        }
    } else if let Some(page_str) = matches.get_one::<String>("page") {
        println!("Searching for page: {}", page_str);
        unimplemented!();
    } else {
        println!("No arguments provided.");
        app.new_page().await?;
    }

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(width, height) => app.resize(width, height),
        }
    }

    tui.exit()?;
    Ok(())
}

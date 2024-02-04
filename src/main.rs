use clikipedia_tui::app::{App, AppResult};
use clikipedia_tui::event::{Event, EventHandler};
use clikipedia_tui::handler::{handle_key_events, handle_mouse_events};
use clikipedia_tui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    // TODO: add viewer that can view by page name or by HTML file.

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

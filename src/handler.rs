use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        _ => (),
    }
    match key_event.code {
        KeyCode::Down | KeyCode::Up | KeyCode::PageDown | KeyCode::PageUp => {
            app.scroll(key_event.code);
        }
        KeyCode::Char(' ') => {
            app.new_page().await?;
        }
        KeyCode::Char(c @ 'a'..='z') => app.link_select(c),
        KeyCode::Backspace => app.delete_link_selector(),
        KeyCode::Enter => app.go_to_selected_link().await,
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        crossterm::event::MouseEventKind::Down(_) => {}
        crossterm::event::MouseEventKind::Up(_) => {}
        crossterm::event::MouseEventKind::Drag(_) => {}
        crossterm::event::MouseEventKind::Moved => {}
        crossterm::event::MouseEventKind::ScrollDown => app.scroll(KeyCode::Down),
        crossterm::event::MouseEventKind::ScrollUp => app.scroll(KeyCode::Up),
        crossterm::event::MouseEventKind::ScrollLeft => {}
        crossterm::event::MouseEventKind::ScrollRight => {}
    }
    Ok(())
}

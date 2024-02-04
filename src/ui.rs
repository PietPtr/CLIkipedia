use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.resize(frame.size().width, frame.size().height);

    // Title bar
    frame.render_widget(
        Paragraph::new(app.page_title.clone())
            .style(Style::default().fg(Color::White).bg(Color::Blue)),
        Rect {
            x: 0,
            y: 0,
            width: frame.size().width,
            height: 1,
        },
    );

    // Page content
    let p = Paragraph::new(app.get_text())
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .wrap(Wrap { trim: false })
        .scroll((app.vertical_scroll as u16, 0));
    let content_length = p.line_count(frame.size().width - 1);

    frame.render_widget(
        p,
        Rect {
            x: 0,
            y: 1,
            width: frame.size().width - 1,
            height: frame.size().height - 1,
        },
    );

    app.set_scroll_params(content_length);

    // Scrollbar
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .style(Style::default().fg(Color::Black).bg(Color::Blue)),
        Rect {
            x: frame.size().width - 1,
            y: 1,
            width: 1,
            height: frame.size().height - 1,
        },
        &mut app.vertical_scroll_state,
    );

    // Link selector box
    if !app.selector.is_empty() {
        let text = format!("[{}]", app.selector);
        let width = text.len() as u16;
        let mut style = Style::default().bg(Color::Gray);
        if app.link_selector_exists() {
            style = style.fg(Color::Blue).bg(Color::White);
        } else {
            style = style.fg(Color::White).bg(Color::Red);
        };
        frame.render_widget(
            Paragraph::new(text).style(style),
            Rect {
                x: 0,
                y: frame.size().height - 1,
                width,
                height: 1,
            },
        )
    }
}

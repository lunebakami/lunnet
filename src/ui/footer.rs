use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn footer_component(app: &App) -> (Paragraph<'static>, Paragraph<'static>) {
    let active_conn = app.get_active_connection_name();

    let mut conn_str = "Connected: ".to_string();
    let conn_name = match active_conn {
        Ok(conn) => conn,
        Err(_) => "".to_string(),
    };

    conn_str.push_str(&conn_name);

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Main Menu", Style::default().fg(Color::Green)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled(conn_str, Style::default().fg(Color::Green)),
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
            CurrentScreen::Exiting => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    (mode_footer, key_notes_footer)
}

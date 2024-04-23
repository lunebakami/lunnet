use app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
    Frame,
};

use crate::app::{self, CurrentScreen};

use super::{footer::footer_component, main_menu::main_menu_component, title::title_component};

// helper function to create a centered rect using up certain percentage of the available rect 'r'
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return middle chunk
}

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = title_component();
    f.render_widget(title, chunks[0]);

    // Main menu
    let text = main_menu_component();
    f.render_widget(text, chunks[1]);

    // footer
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let (mode_footer, key_notes_footer) = footer_component(app);
    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear,f.size());
    }
}

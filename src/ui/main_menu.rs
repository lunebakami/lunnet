use ratatui::{style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph}};

pub fn main_menu_component() -> Paragraph<'static> {
    let text_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let text = Paragraph::new(Text::styled(
        "Options here",
        Style::default().fg(Color::Green),
    ))
    .block(text_block);

    text
}

use ratatui::{layout::Alignment, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph}};

pub fn title_component() -> Paragraph<'static> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Lunnet",
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    title
}

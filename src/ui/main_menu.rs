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

    // Lista dos 5 Wi-fis mais proximos
    //    - Pode navegar entre eles
    //    - Apertar Enter deve conectar com a rede
    // Botão de ver todos

    text
}

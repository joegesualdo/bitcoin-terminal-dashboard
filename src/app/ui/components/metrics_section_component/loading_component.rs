use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn metrics_section_loading_component<'a>() -> Paragraph<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    let foreground_color = Color::LightCyan;
    let style = Style::default().fg(foreground_color);
    let alignment = Alignment::Left;

    let paragraphs = vec![Spans::from(Span::raw("Loading..."))];

    Paragraph::new(paragraphs)
        .block(block)
        .style(style)
        .alignment(alignment)
}

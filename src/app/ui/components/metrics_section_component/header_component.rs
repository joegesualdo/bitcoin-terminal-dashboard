use tui::style::{Modifier, Style};
use tui::text::Span;

use crate::app::ui::BITCOIN_ORANGE_COLOR;

pub fn metric_section_header_component<'a>(header_text: &'a str) -> Vec<Span<'a>> {
    let section_header_span = Span::styled(
        format!("{}: ", header_text),
        Style::default()
            .fg(BITCOIN_ORANGE_COLOR)
            .add_modifier(Modifier::UNDERLINED),
    );

    let spans_list = vec![section_header_span];
    spans_list
}

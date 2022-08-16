use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

use crate::app::ui::BITCOIN_ORANGE_COLOR;

fn metric_name_component<'a>(metric_name: &'a str) -> Spans<'a> {
    let metric_name_span = Span::styled(
        format!(" {}", metric_name),
        Style::default().fg(Color::White),
    );
    let dot_span = Span::styled(
        "...............................................................................................................................................................................................................................................................................................................................................",
        Style::default().fg(Color::DarkGray),
    );
    Spans(vec![metric_name_span, dot_span])
}
fn metric_value_component<'a>(metric_value: String) -> Spans<'a> {
    let span = Span::styled(
        format!("{} ", metric_value),
        Style::default().fg(BITCOIN_ORANGE_COLOR),
    );
    Spans(vec![span])
}

pub fn metric_line_component<'a>(metric_name: &'a str, metric_value: String) -> Vec<Spans<'a>> {
    let mv = metric_value.to_string();
    let metric_name_span = metric_name_component(&metric_name);
    let metric_value_span = metric_value_component(mv);

    let spans_list = vec![metric_name_span, metric_value_span];
    spans_list
}

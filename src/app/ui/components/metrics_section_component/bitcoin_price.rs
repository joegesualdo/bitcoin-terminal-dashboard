use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_duration, format_float_number, round};

fn format_price(price: f64) -> String {
    let rounded_price = round(price, 2);
    let price_formatted = format_float_number(rounded_price);
    format!("${}", price_formatted)
}

pub fn bitcoin_price_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    let bitcoin_price = match initialized_data.stats.bitcoin_price {
        FetchStatus::Complete(bitcoin_price) => format_price(bitcoin_price),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("↻ {}", format_price(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let bitcoin_price_text = format!("{}", bitcoin_price);
    let bitcoin_price_spans = metric_line_component("Price", bitcoin_price_text);
    bitcoin_price_spans
}

use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_duration, format_float_number, round};

fn sats_per_dollar(bitcoin_price: f64) -> f64 {
    1.0 / (bitcoin_price / 100_000_000.0)
}

fn get_sats_per_dollar_format_for_bitcoin_price(bitcoin_price: f64) -> String {
    let sats_per_dollar = sats_per_dollar(bitcoin_price);
    let rounded = round(sats_per_dollar, 0);
    format_float_number(rounded)
}

pub fn sats_per_dollar_component<'a>(initialized_data: &'a InitializedData) -> Vec<Span> {
    let sats_per_dollar = match initialized_data.stats.bitcoin_price {
        FetchStatus::Complete(bitcoin_price) => {
            get_sats_per_dollar_format_for_bitcoin_price(bitcoin_price)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!(
                "{} (loading...)",
                get_sats_per_dollar_format_for_bitcoin_price(old_value)
            ),
            None => "Loading...".to_string(),
        },
    };
    let sats_per_dollar_text = format!("{}", sats_per_dollar);
    let sats_per_dollar_spans = metric_line_component("Sats per dollar", sats_per_dollar_text);
    sats_per_dollar_spans
}

use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{convert_satoshis_to_btc, format_float_number, format_number, round};

fn format_fees(total_fees_for_last_24_hours: u64) -> String {
    let total_fees_in_btc = convert_satoshis_to_btc(total_fees_for_last_24_hours);
    let rounded = round(total_fees_in_btc, 2);
    let rounded_string = format_float_number(rounded);
    format!("{} BTC", rounded_string)
}
pub fn total_fees_for_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let total_fees_for_last_24_hours = match initialized_data.stats.total_fees_for_last_24_hours {
        FetchStatus::Complete(total_fees_for_last_24_hours) => {
            format_fees(total_fees_for_last_24_hours)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format_fees(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let total_fees_for_last_24_hours_text = format!("{}", total_fees_for_last_24_hours);
    let total_fees_for_last_24_hours_spans =
        metric_line_component("Total Fees (24h)", total_fees_for_last_24_hours_text);
    total_fees_for_last_24_hours_spans
}

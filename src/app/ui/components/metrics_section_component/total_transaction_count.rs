use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_number;

pub fn total_transactions_count_component<'a>(initialized_data: &'a InitializedData) -> Vec<Span> {
    let total_transactions_count = match initialized_data.stats.total_transactions_count {
        FetchStatus::Complete(total_transactions_count) => format_number(total_transactions_count),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let total_transactions_count_text = format!("{}", total_transactions_count);
    let total_transactions_count_spans =
        metric_line_component("Total Transaction Count", total_transactions_count_text);
    total_transactions_count_spans
}

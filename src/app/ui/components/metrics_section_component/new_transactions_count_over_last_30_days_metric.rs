use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_number;

pub fn new_transactions_count_over_last_30_days_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let transactions_count_over_last_30_days =
        match initialized_data.stats.transactions_count_over_last_30_days {
            FetchStatus::Complete(transactions_count_over_last_30_days) => {
                format_number(transactions_count_over_last_30_days)
            }
            FetchStatus::NotStarted => "Not Started...".to_string(),
            FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
                Some(old_value) => format!("{} (loading...)", old_value),
                None => "Loading...".to_string(),
            },
        };
    let transactions_count_over_last_30_days_block_text =
        format!("{}", transactions_count_over_last_30_days);
    let transactions_count_over_last_30_days_block_spans = metric_line_component(
        "Transactions count (30 days)",
        transactions_count_over_last_30_days,
    );
    transactions_count_over_last_30_days_block_spans
}

use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_number;

pub fn new_transactions_count_over_last_30_days_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Transactions (30 days)",
        &initialized_data.stats.transactions_count_over_last_30_days,
        |transactions_count_over_last_30_days: &u64| -> String {
            format_number(transactions_count_over_last_30_days.clone())
        },
    )
}

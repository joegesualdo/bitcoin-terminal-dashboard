use tui::text::{Span, Spans};

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_number;

pub fn total_transactions_count_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Total Transactions ",
        &initialized_data.stats.total_transactions_count,
        |total_transactions_count: &u64| -> String {
            format_number(total_transactions_count.clone())
        },
    )
}

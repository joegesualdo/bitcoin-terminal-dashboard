use tui::text::{Span, Spans};

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_number;

pub fn block_height_metric_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Block Height",
        &initialized_data.stats.block_height,
        |block_height: &u64| -> String { format_number(block_height.clone()) },
    )
}

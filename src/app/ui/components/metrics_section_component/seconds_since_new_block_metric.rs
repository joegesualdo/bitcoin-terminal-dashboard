use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_duration;

pub fn seconds_since_new_block_metric_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Time Since Last Block",
        &initialized_data.stats.seconds_since_last_block,
        |seconds_since_last_block: &u64| -> String {
            format_duration(seconds_since_last_block.clone() as i64)
        },
    )
}

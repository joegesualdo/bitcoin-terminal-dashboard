use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_duration;

pub fn average_block_time_for_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Avg. Block Time (2016 blocks)",
        &initialized_data
            .stats
            .average_block_time_for_last_2016_blocks,
        |average_block_time_for_last_2016_blocks: &u64| -> String {
            format_duration(average_block_time_for_last_2016_blocks.clone() as i64)
        },
    )
}

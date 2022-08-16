use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_duration;

pub fn average_block_time_since_last_difficulty_adjustement_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Avg. Block Time (current epoch)",
        &initialized_data
            .stats
            .average_block_time_since_last_difficulty_adjustement,
        |average_block_time_since_last_difficulty_adjustement: &u64| -> String {
            format_duration(average_block_time_since_last_difficulty_adjustement.clone() as i64)
        },
    )
}

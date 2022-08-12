use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_duration;

pub fn average_block_time_for_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Span> {
    let average_block_time_for_last_2016_blocks = match initialized_data
        .stats
        .average_block_time_for_last_2016_blocks
    {
        FetchStatus::Complete(average_block_time_for_last_2016_blocks) => {
            format_duration(average_block_time_for_last_2016_blocks as i64)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format_duration(old_value as i64)),
            None => "Loading...".to_string(),
        },
    };
    let average_block_time_for_last_2016_blocks =
        format!("{}", average_block_time_for_last_2016_blocks);
    let average_block_time_for_last_2016_blocks_spans = metric_line_component(
        "Average Block Time For Last 2016 Blocks",
        average_block_time_for_last_2016_blocks,
    );
    average_block_time_for_last_2016_blocks_spans
}

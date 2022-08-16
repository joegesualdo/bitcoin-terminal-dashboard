use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_duration;

pub fn average_block_time_since_last_difficulty_adjustement_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let average_block_time_since_last_difficulty_adjustement = match initialized_data
        .stats
        .average_block_time_since_last_difficulty_adjustement
    {
        FetchStatus::Complete(average_block_time_since_last_difficulty_adjustement) => {
            format_duration(average_block_time_since_last_difficulty_adjustement as i64)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format_duration(old_value as i64)),
            None => "Loading...".to_string(),
        },
    };
    let average_block_time_since_last_difficulty_adjustement_text =
        format!("{}", average_block_time_since_last_difficulty_adjustement);
    let average_block_time_since_last_difficulty_adjustement_spans = metric_line_component(
        "Average Block Time (current epoch)",
        average_block_time_since_last_difficulty_adjustement,
    );
    average_block_time_since_last_difficulty_adjustement_spans
}

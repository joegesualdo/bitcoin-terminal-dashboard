use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_duration;

pub fn seconds_since_new_block_metric_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let seconds_since_last_block = match initialized_data.stats.seconds_since_last_block {
        FetchStatus::Complete(seconds_since_last_block) => {
            format_duration(seconds_since_last_block as i64)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format_duration(old_value as i64)),
            None => "Loading...".to_string(),
        },
    };
    let seconds_since_last_block_text = format!("{}", seconds_since_last_block);
    let seconds_since_last_block_spans =
        metric_line_component("Time Since Last Block", seconds_since_last_block_text);
    seconds_since_last_block_spans
}

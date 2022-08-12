use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_number;

pub fn block_height_metric_component<'a>(initialized_data: &'a InitializedData) -> Vec<Span> {
    let block_height = match initialized_data.stats.block_height {
        FetchStatus::Complete(block_height) => format_number(block_height),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let block_height = format!("{}", block_height);
    let block_height_spans = metric_line_component("Block Height", block_height);
    block_height_spans
}

use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_number;

pub fn current_difficulty_epoch_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    let current_difficulty_epoch = match initialized_data.stats.current_difficulty_epoch {
        FetchStatus::Complete(current_difficulty_epoch) => format_number(current_difficulty_epoch),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let current_difficulty_epoch_text = format!("{}", current_difficulty_epoch);
    let current_difficulty_epoch_spans =
        metric_line_component("Current Difficulty Epoch", current_difficulty_epoch_text);
    current_difficulty_epoch_spans
}

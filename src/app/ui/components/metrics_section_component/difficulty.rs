use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number};

pub fn difficulty_component<'a>(initialized_data: &'a InitializedData) -> Vec<Span> {
    let difficulty = match initialized_data.stats.difficulty {
        FetchStatus::Complete(difficulty) => format_float_number(difficulty),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let difficulty_text = format!("{}", difficulty);
    let difficulty_spans = metric_line_component("Difficulty", difficulty);
    difficulty_spans
}

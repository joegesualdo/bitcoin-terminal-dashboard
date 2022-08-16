use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

const TRILLION: f64 = 1_000_000_000_000.0;

fn format(difficulty: f64) -> String {
    let difficulty_in_trillions = difficulty / TRILLION;
    let rounded = round(difficulty_in_trillions, 2);
    let formated_difficulty = format_float_number(rounded);
    format!("{} T", formated_difficulty)
}
pub fn difficulty_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    let difficulty = match initialized_data.stats.difficulty {
        FetchStatus::Complete(difficulty) => format(difficulty),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let difficulty_text = format!("{}", difficulty);
    let difficulty_spans = metric_line_component("Difficulty", difficulty);
    difficulty_spans
}

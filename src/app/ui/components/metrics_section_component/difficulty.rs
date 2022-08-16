use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{format_float_number, round};

const TRILLION: f64 = 1_000_000_000_000.0;

pub fn difficulty_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Difficulty",
        &initialized_data.stats.difficulty,
        |difficulty: &f64| -> String {
            let difficulty_in_trillions = difficulty.clone() / TRILLION;
            let rounded = round(difficulty_in_trillions, 2);
            let formated_difficulty = format_float_number(rounded);
            format!("{} T", formated_difficulty)
        },
    )
}

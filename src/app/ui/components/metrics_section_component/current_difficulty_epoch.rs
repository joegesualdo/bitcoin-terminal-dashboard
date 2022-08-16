use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_number;

pub fn current_difficulty_epoch_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Difficulty Epoch",
        &initialized_data.stats.current_difficulty_epoch,
        |current_difficulty_epoch: &u64| -> String {
            format_number(current_difficulty_epoch.clone())
        },
    )
}

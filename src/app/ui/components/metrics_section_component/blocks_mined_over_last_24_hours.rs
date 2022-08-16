use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_number;

pub fn blocks_mined_over_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let blocks_mined_over_last_24_hours =
        match initialized_data.stats.blocks_mined_over_last_24_hours {
            FetchStatus::Complete(blocks_mined_over_last_24_hours) => {
                format_number(blocks_mined_over_last_24_hours)
            }
            FetchStatus::NotStarted => "Not Started...".to_string(),
            FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
                Some(old_value) => format!("{} (loading...)", old_value),
                None => "Loading...".to_string(),
            },
        };
    let blocks_mined_over_last_24_hours_text = format!("{}", blocks_mined_over_last_24_hours);
    let blocks_mined_over_last_24_hours_spans = metric_line_component(
        "Blocks Mined over last 24 hours",
        blocks_mined_over_last_24_hours_text,
    );
    blocks_mined_over_last_24_hours_spans
}

use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

fn format(block_count: f64) -> String {
    let rounded = round(block_count, 2);
    format_float_number(rounded)
}

pub fn block_count_until_retarget_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Span> {
    let block_count_until_retarget = match initialized_data.stats.block_count_until_retarget {
        FetchStatus::Complete(block_count_until_retarget) => format(block_count_until_retarget),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let block_count_until_retarget_text = format!("{}", block_count_until_retarget);
    let block_count_until_retarget_spans = metric_line_component(
        "Block Count Until Retarget",
        block_count_until_retarget_text,
    );
    block_count_until_retarget_spans
}

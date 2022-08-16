use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{format_float_number, round};

fn format(block_count: f64) -> String {
    let rounded = round(block_count, 2);
    format_float_number(rounded)
}

pub fn block_count_until_retarget_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Blocks Until Retarget",
        &initialized_data.stats.block_count_until_retarget,
        |block_count_until_retarget: &f64| -> String { format(block_count_until_retarget.clone()) },
    )
}

use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_number;

pub fn blocks_mined_over_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Blocks Mined (24h) ",
        &initialized_data.stats.blocks_mined_over_last_24_hours,
        |blocks_mined_over_last_24_hours: &u64| -> String {
            format_number(blocks_mined_over_last_24_hours.clone())
        },
    )
}

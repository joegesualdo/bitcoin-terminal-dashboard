use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::round;

pub fn fees_as_a_percent_of_reward_for_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Avg. Fees Vs Reward (2016 blocks)",
        &initialized_data
            .stats
            .fees_as_a_percent_of_reward_for_last_2016_blocks,
        |fees_as_a_percent_of_reward_for_last_2016_blocks: &f64| -> String {
            let percent = fees_as_a_percent_of_reward_for_last_2016_blocks * 100.0;
            let rounded = round(percent, 2);
            format!("{}%", rounded)
        },
    )
}

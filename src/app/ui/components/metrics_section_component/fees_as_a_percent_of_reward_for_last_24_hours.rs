use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::round;

pub fn fees_as_a_percent_of_reward_for_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Avg. Fees vs. Reward (24h)",
        &initialized_data
            .stats
            .fees_as_a_percent_of_reward_for_last_24_hours,
        |fees_as_a_percent_of_reward_for_last_24_hours: &f64| -> String {
            let percent = fees_as_a_percent_of_reward_for_last_24_hours * 100.0;
            let rounded = round(percent, 2);
            format!("{}%", rounded)
        },
    )
}

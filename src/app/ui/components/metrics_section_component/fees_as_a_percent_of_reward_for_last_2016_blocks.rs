use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

fn format(fees_as_a_percent_of_reward_for_last_2016_blocks: f64) -> String {
    let percent = fees_as_a_percent_of_reward_for_last_2016_blocks * 100.0;
    let rounded = round(percent, 2);
    format!("{}%", rounded)
}

pub fn fees_as_a_percent_of_reward_for_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Span> {
    let fees_as_a_percent_of_reward_for_last_2016_blocks = match initialized_data
        .stats
        .fees_as_a_percent_of_reward_for_last_2016_blocks
    {
        FetchStatus::Complete(fees_as_a_percent_of_reward_for_last_2016_blocks) => {
            format(fees_as_a_percent_of_reward_for_last_2016_blocks)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let fees_as_a_percent_of_reward_for_last_2016_blocks_text =
        format!("{}", fees_as_a_percent_of_reward_for_last_2016_blocks);
    let fees_as_a_percent_of_reward_for_last_2016_blocks_spans = metric_line_component(
        "Avg. Fees Vs Reward (2016 blocks)",
        fees_as_a_percent_of_reward_for_last_2016_blocks_text,
    );
    fees_as_a_percent_of_reward_for_last_2016_blocks_spans
}

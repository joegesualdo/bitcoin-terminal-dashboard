use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

fn format(fees_as_a_percent_of_reward_for_last_24_hours: f64) -> String {
    let percent = fees_as_a_percent_of_reward_for_last_24_hours * 100.0;
    let rounded = round(percent, 2);
    format!("{}%", rounded)
}

pub fn fees_as_a_percent_of_reward_for_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Span> {
    let fees_as_a_percent_of_reward_for_last_24_hours = match initialized_data
        .stats
        .fees_as_a_percent_of_reward_for_last_24_hours
    {
        FetchStatus::Complete(fees_as_a_percent_of_reward_for_last_24_hours) => {
            format(fees_as_a_percent_of_reward_for_last_24_hours)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let fees_as_a_percent_of_reward_for_last_24_hours_text =
        format!("{}", fees_as_a_percent_of_reward_for_last_24_hours);
    let fees_as_a_percent_of_reward_for_last_24_hours_spans = metric_line_component(
        "Avg. Fees Vs Reward (24 hours)",
        fees_as_a_percent_of_reward_for_last_24_hours_text,
    );
    fees_as_a_percent_of_reward_for_last_24_hours_spans
}

use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{convert_satoshis_to_btc, format_number};

fn format(block_subsidy_of_most_recent_block: u64) -> String {
    let block_subsidy_denominated_in_btc =
        convert_satoshis_to_btc(block_subsidy_of_most_recent_block);
    format!("{}", block_subsidy_denominated_in_btc)
}

pub fn block_subsidy_of_most_recent_block_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let block_subsidy_of_most_recent_block =
        match initialized_data.stats.block_subsidy_of_most_recent_block {
            FetchStatus::Complete(block_subsidy_of_most_recent_block) => {
                format(block_subsidy_of_most_recent_block)
            }
            FetchStatus::NotStarted => "Not Started...".to_string(),
            FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
                Some(old_value) => format!("{} (loading...)", format(old_value)),
                None => "Loading...".to_string(),
            },
        };
    let block_subsidy_of_most_recent_block_text = format!("{}", block_subsidy_of_most_recent_block);
    let block_subsidy_of_most_recent_block_spans =
        metric_line_component("Block Subsidy", block_subsidy_of_most_recent_block_text);
    block_subsidy_of_most_recent_block_spans
}

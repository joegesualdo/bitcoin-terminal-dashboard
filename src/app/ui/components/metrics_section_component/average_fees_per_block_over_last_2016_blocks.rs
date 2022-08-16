use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{convert_satoshis_to_btc, format_float_number, format_number, round};

fn format(average_fees_per_block_over_last_2016_blocks: u64) -> String {
    let average_fees_per_block_over_last_2016_blocks_in_btc =
        convert_satoshis_to_btc(average_fees_per_block_over_last_2016_blocks);
    let rounded = round(average_fees_per_block_over_last_2016_blocks_in_btc, 2);
    format!("{} BTC", rounded)
}

pub fn average_fees_per_block_over_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let average_fees_per_block_over_last_2016_blocks = match initialized_data
        .stats
        .average_fees_per_block_over_last_2016_blocks
    {
        FetchStatus::Complete(average_fees_per_block_over_last_2016_blocks) => {
            format(average_fees_per_block_over_last_2016_blocks)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", format(old_value)),
            None => "Loading...".to_string(),
        },
    };
    let average_fees_per_block_over_last_2016_blocks_text =
        format!("{}", average_fees_per_block_over_last_2016_blocks);
    let average_fees_per_block_over_last_2016_blocks_spans = metric_line_component(
        "Average Fees Per Block (2016 blocks)",
        average_fees_per_block_over_last_2016_blocks_text,
    );
    average_fees_per_block_over_last_2016_blocks_spans
}

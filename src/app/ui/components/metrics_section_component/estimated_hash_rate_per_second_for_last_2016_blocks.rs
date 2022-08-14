use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

const ONE_QUINTILLION: u64 = 1_000_000_000_000_000_000;
const HASHES_PER_EXA_HASHES: u64 = ONE_QUINTILLION;
fn format(hash_rate: f64) -> String {
    let exa_hashes_per_second = hash_rate / HASHES_PER_EXA_HASHES as f64;
    let rounded_exa_hashes_per_second = round(exa_hashes_per_second, 1);
    format!("{} EH/s", rounded_exa_hashes_per_second)
}

pub fn estimated_hash_rate_per_second_for_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Span> {
    let estimated_hash_rate_per_second_for_last_2016_blocks = match initialized_data
        .stats
        .estimated_hash_rate_per_second_for_last_2016_blocks
    {
        FetchStatus::Complete(estimated_hash_rate_per_second_for_last_2016_blocks) => {
            format(estimated_hash_rate_per_second_for_last_2016_blocks)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let estimated_hash_rate_per_second_for_last_2016_blocks_text =
        format!("{}", estimated_hash_rate_per_second_for_last_2016_blocks);
    let estimated_hash_rate_per_second_for_last_2016_blocks_spans = metric_line_component(
        "Estimated HashRate (2016 blocks)",
        estimated_hash_rate_per_second_for_last_2016_blocks_text,
    );
    estimated_hash_rate_per_second_for_last_2016_blocks_spans
}

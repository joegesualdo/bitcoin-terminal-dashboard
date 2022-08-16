use tui::text::Spans;

use crate::app::state::InitializedData;
use crate::utils::round;

use super::metric_line_fetch_status_component;

const ONE_QUINTILLION: u64 = 1_000_000_000_000_000_000;
const HASHES_PER_EXA_HASHES: u64 = ONE_QUINTILLION;

pub fn estimated_hash_rate_per_second_for_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Estimated Hash Rate (2016 blocks)",
        &initialized_data
            .stats
            .estimated_hash_rate_per_second_for_last_2016_blocks,
        |estimated_hash_rate_per_second_for_last_2016_blocks: &f64| -> String {
            let exa_hashes_per_second =
                estimated_hash_rate_per_second_for_last_2016_blocks / HASHES_PER_EXA_HASHES as f64;
            let rounded_exa_hashes_per_second = round(exa_hashes_per_second, 1);
            format!("{} EH/s", rounded_exa_hashes_per_second)
        },
    )
}

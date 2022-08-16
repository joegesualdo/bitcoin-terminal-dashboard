use tui::text::Spans;

use crate::app::state::InitializedData;
use crate::utils::{convert_satoshis_to_btc, round};

use super::metric_line_fetch_status_component;

pub fn average_fees_per_block_over_last_2016_blocks_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Avg. Fees per Block (2016 blocks)",
        &initialized_data
            .stats
            .average_fees_per_block_over_last_2016_blocks,
        |average_fees_per_block_over_last_2016_blocks: &u64| -> String {
            let average_fees_per_block_over_last_2016_blocks_in_btc =
                convert_satoshis_to_btc(average_fees_per_block_over_last_2016_blocks.clone());
            let rounded = round(average_fees_per_block_over_last_2016_blocks_in_btc, 2);
            format!("{} BTC", rounded)
        },
    )
}

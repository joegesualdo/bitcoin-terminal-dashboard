use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::convert_satoshis_to_btc;

pub fn block_subsidy_of_most_recent_block_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Block Subsidy",
        &initialized_data.stats.block_subsidy_of_most_recent_block,
        |block_subsidy_of_most_recent_block: &u64| -> String {
            let block_subsidy_denominated_in_btc =
                convert_satoshis_to_btc(block_subsidy_of_most_recent_block.clone());
            format!("{}", block_subsidy_denominated_in_btc)
        },
    )
}

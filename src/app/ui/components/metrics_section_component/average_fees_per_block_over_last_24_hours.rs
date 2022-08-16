use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{convert_satoshis_to_btc, round};

pub fn average_fees_per_block_over_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Avg. Fees per Block (24h)",
        &initialized_data
            .stats
            .average_fees_per_block_over_last_24_hours,
        |average_fees_per_block_over_last_24_hours: &u64| -> String {
            let average_fees_per_block_over_last_24_hours_in_btc =
                convert_satoshis_to_btc(average_fees_per_block_over_last_24_hours.clone());
            let rounded = round(average_fees_per_block_over_last_24_hours_in_btc, 2);
            format!("{} BTC", rounded)
        },
    )
}

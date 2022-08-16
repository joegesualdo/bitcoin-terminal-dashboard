use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{convert_satoshis_to_btc, format_float_number, round};

pub fn total_fees_for_last_24_hours_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Total Fees (24h)",
        &initialized_data.stats.total_fees_for_last_24_hours,
        |total_fees_for_last_24_hours: &u64| -> String {
            let total_fees_in_btc = convert_satoshis_to_btc(total_fees_for_last_24_hours.clone());
            let rounded = round(total_fees_in_btc, 2);
            let rounded_string = format_float_number(rounded);
            format!("{} BTC", rounded_string)
        },
    )
}

use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{format_float_number, round};

pub fn sats_per_dollar_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Sats per dollar",
        &initialized_data.stats.bitcoin_price,
        |price: &f64| -> String {
            let sats_per_dollar = 1.0 / (price / 100_000_000.0);
            let sats_per_dollar = sats_per_dollar;
            let rounded = round(sats_per_dollar, 0);
            format_float_number(rounded)
        },
    )
}

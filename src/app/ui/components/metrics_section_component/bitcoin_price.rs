use tui::text::{Span, Spans};

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{format_float_number, round};

pub fn bitcoin_price_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Price",
        &initialized_data.stats.bitcoin_price,
        |price: &f64| -> String {
            let rounded_price = round(price.clone(), 2);
            let price_formatted = format_float_number(rounded_price);
            format!("${}", price_formatted)
        },
    )
}

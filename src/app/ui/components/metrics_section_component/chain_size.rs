use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{format_float_number, round};

pub fn chain_size_metric_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Chain Size",
        &initialized_data.stats.chain_size,
        |chain_size: &u64| -> String {
            let chain_size_in_gigabytes = chain_size.clone() as f64 / 1_000_000_000.0;
            let rounded_chain_size_in_gigabytes = round(chain_size_in_gigabytes, 1);
            let gb_num_string = format_float_number(rounded_chain_size_in_gigabytes);
            format!("{} GB", gb_num_string)
        },
    )
}

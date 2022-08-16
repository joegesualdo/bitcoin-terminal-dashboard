use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::format_number;

pub fn utxo_set_size_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "UTXO Set Size",
        &initialized_data.stats.utxo_set_size,
        |utxo_set_size: &u64| -> String { format_number(utxo_set_size.clone()) },
    )
}

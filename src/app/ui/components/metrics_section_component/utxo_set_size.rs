use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_number;

pub fn utxo_set_size_component<'a>(initialized_data: &'a InitializedData) -> Vec<Span> {
    let utxo_set_size = match initialized_data.stats.utxo_set_size {
        FetchStatus::Complete(utxo_set_size) => format_number(utxo_set_size),
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let utxo_set_size_text = format!("{}", utxo_set_size);
    let utxo_set_size_spans = metric_line_component("UTXO Set Size", utxo_set_size_text);
    utxo_set_size_spans
}

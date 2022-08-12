use tui::text::Span;

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

pub fn chain_size_metric_component<'a>(initialized_data: &'a InitializedData) -> Vec<Span> {
    let chain_size = match initialized_data.stats.chain_size {
        FetchStatus::Complete(chain_size) => {
            let chain_size_in_gigabytes = chain_size as f64 / 1_000_000_000.0;
            let rounded_chain_size_in_gigabytes = round(chain_size_in_gigabytes, 1);
            let gb_num_string = format_float_number(rounded_chain_size_in_gigabytes);
            format!("{} GB", gb_num_string)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let chain_size = format!("{}", chain_size);
    let chain_size_spans = metric_line_component("Chain Size", chain_size);
    chain_size_spans
}

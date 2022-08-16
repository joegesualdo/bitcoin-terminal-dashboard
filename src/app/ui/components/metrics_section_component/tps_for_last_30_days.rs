use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::{format_float_number, format_number, round};

pub fn tps_for_last_30_days_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    let tps_for_last_30_days = match initialized_data.stats.tps_for_last_30_days {
        FetchStatus::Complete(tps_for_last_30_days) => {
            let rounded_tps = round(tps_for_last_30_days, 1);
            let rounded_tps_string = format_float_number(rounded_tps);
            format!("{} tx/s", rounded_tps_string)
        }
        FetchStatus::NotStarted => "Not Started...".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("{} (loading...)", old_value),
            None => "Loading...".to_string(),
        },
    };
    let tps_for_last_30_days_text = format!("{}", tps_for_last_30_days);
    let tps_for_last_30_days_spans =
        metric_line_component("TPS (30 days)", tps_for_last_30_days_text);
    tps_for_last_30_days_spans
}

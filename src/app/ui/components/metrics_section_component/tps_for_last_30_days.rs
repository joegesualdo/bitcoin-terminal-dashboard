use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;
use crate::utils::{format_float_number, round};

pub fn tps_for_last_30_days_component<'a>(initialized_data: &'a InitializedData) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "TPS (30 days)",
        &initialized_data.stats.tps_for_last_30_days,
        |tps_for_last_30_days: &f64| -> String {
            let rounded_tps = round(tps_for_last_30_days.clone(), 1);
            let rounded_tps_string = format_float_number(rounded_tps);
            format!("{} tx/s", rounded_tps_string)
        },
    )
}

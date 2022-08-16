use chrono::{Duration, Utc};
use tui::text::Spans;

use super::metric_line_fetch_status_component;
use crate::app::state::InitializedData;

pub fn estimated_seconds_until_retarget_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    metric_line_fetch_status_component(
        "Estimated retarget date",
        &initialized_data.stats.estimated_seconds_until_retarget,
        |estimated_seconds_until_retarget: &f64| -> String {
            let date_of_estimated_retarget =
                Utc::now() + Duration::seconds(estimated_seconds_until_retarget.clone() as i64);
            date_of_estimated_retarget.format("%a %b %e").to_string()
        },
    )
}

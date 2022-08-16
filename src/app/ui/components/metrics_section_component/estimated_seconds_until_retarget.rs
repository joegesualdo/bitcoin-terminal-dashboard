use chrono::{Duration, Local, Utc};
use tui::text::{Span, Spans};

use super::metrics_line_component::metric_line_component;
use crate::app::state::{FetchStatus, InitializedData};
use crate::utils::format_float_number;

pub fn estimated_seconds_until_retarget_component<'a>(
    initialized_data: &'a InitializedData,
) -> Vec<Spans> {
    let estimated_seconds_until_retarget =
        match initialized_data.stats.estimated_seconds_until_retarget {
            FetchStatus::Complete(estimated_seconds_until_retarget) => {
                let date_of_estimated_retarget =
                    Utc::now() + Duration::seconds(estimated_seconds_until_retarget as i64);
                date_of_estimated_retarget.format("%a %b %e").to_string()
            }
            FetchStatus::NotStarted => "Not Started...".to_string(),
            FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
                Some(old_value) => format!("{} (loading...)", old_value),
                None => "Loading...".to_string(),
            },
        };
    let estimated_seconds_until_retarget_text = format!("{}", estimated_seconds_until_retarget);
    let estimated_seconds_until_retarget_spans = metric_line_component(
        "Estimated retarget date:",
        estimated_seconds_until_retarget_text,
    );
    estimated_seconds_until_retarget_spans
}

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;

mod average_block_time_for_last_2016_blocks;
mod block_height_metric;
mod header_component;
pub mod loading_component;
mod metrics_line_component;
mod new_transactions_count_over_last_30_days_metric;
mod seconds_since_new_block_metric;

use self::average_block_time_for_last_2016_blocks::average_block_time_for_last_2016_blocks_component;
use self::block_height_metric::block_height_metric_component;
use self::header_component::metric_section_header_component;
use self::metrics_line_component::metric_line_component;
use self::new_transactions_count_over_last_30_days_metric::new_transactions_count_over_last_30_days_component;
use self::seconds_since_new_block_metric::seconds_since_new_block_metric_component;
use crate::app::state::{AppState, FetchStatus, InitializedData};
use crate::app::ui::BITCOIN_ORANGE_COLOR;
use crate::app::App;
use crate::utils::{format_duration, format_number, format_number_string};

pub fn metrics_section_component<'a>(
    initialized_data: &'a InitializedData,
    state: &'a AppState,
) -> Paragraph<'a> {
    // Heading
    let section_header = metric_section_header_component("Blockchain stats");

    // Lines
    let blockchain_height = block_height_metric_component(initialized_data);
    let seconds_since_last_block = seconds_since_new_block_metric_component(initialized_data);
    let transactions_count_over_last_30_days =
        new_transactions_count_over_last_30_days_component(initialized_data);
    let average_block_time_for_last_2016_blocks =
        average_block_time_for_last_2016_blocks_component(initialized_data);

    let paragraphs = vec![
        Spans(section_header),
        Spans(blockchain_height),
        Spans(seconds_since_last_block),
        Spans(transactions_count_over_last_30_days),
        Spans(average_block_time_for_last_2016_blocks),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    let foreground_color = Color::LightCyan;
    let style = Style::default().fg(foreground_color);
    let alignment = Alignment::Left;

    Paragraph::new(paragraphs)
        .block(block)
        .style(style)
        .alignment(alignment)
}

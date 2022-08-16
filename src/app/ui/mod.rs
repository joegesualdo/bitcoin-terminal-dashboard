use std::u16::MAX;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use self::components::metrics_section_component::{
    blockchain_data_component, difficulty_data_component, market_data_component,
    mining_data_component, transactions_data_component,
};
use super::state::AppState;
use crate::app::App;
mod components;

const ASCII_ART: &str = r#"
  ___ _ _          _         ___          _    
 | _ |_) |_ __ ___(_)_ _    |   \ __ _ __| |_  
 | _ \ |  _/ _/ _ \ | ' \   | |) / _` (_-< ' \ 
 |___/_|\__\__\___/_|_||_|  |___/\__,_/__/_||_|
"#;

const BITCOIN_ORANGE_COLOR: Color = Color::Rgb(242, 169, 0);

fn title_component<'a>() -> Paragraph<'a> {
    Paragraph::new(ASCII_ART.replacen("\n", "", 1))
        .style(Style::default().fg(BITCOIN_ORANGE_COLOR))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan))
                .border_type(BorderType::Rounded),
        )
}

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // Get Size of Terminal
    let size = rect.size();

    // Split terminal into title section and metrics section
    let app_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(10)].as_ref())
        .split(size);

    let title_chunk = app_chunks[0];
    let metrics_chunk = app_chunks[1];

    // Render Title
    let title_component = title_component();

    // Create wrapper Box for Metrics chunk, with stylings
    let metrics_block = Block::default()
        .borders(Borders::ALL)
        .title("Metrics")
        .border_type(BorderType::Rounded);

    //
    let metric_sections_uis = match app.state() {
        AppState::Init => vec![],
        AppState::Initialized(initialized_data) => {
            vec![
                market_data_component(&initialized_data),
                blockchain_data_component(&initialized_data),
                transactions_data_component(initialized_data),
                difficulty_data_component(initialized_data),
                mining_data_component(initialized_data),
            ]
        }
    };

    type Width = u16;
    enum ScreenWidth {
        XSmall(Width),
        Small(Width),
        Medium(Width),
        Large(Width),
        XLarge(Width),
    }

    let screen_width = match size.width {
        width @ 0..=100 => ScreenWidth::XSmall(width),
        width @ 101..=150 => ScreenWidth::Small(width),
        width @ 151..=200 => ScreenWidth::Medium(width),
        width @ 201..=250 => ScreenWidth::Large(width),
        width @ 251..=MAX => ScreenWidth::XLarge(width),
    };

    let columns_count = match screen_width {
        ScreenWidth::XSmall(_width) => 1,
        ScreenWidth::Small(_width) => 2,
        ScreenWidth::Medium(_width) => 3,
        ScreenWidth::Large(_width) => 4,
        ScreenWidth::XLarge(_width) => 5,
    };

    let horizontal_metrics_chunks_count_per_vertical_chunk = columns_count;

    let minimum_number_of_rows_required =
        (metric_sections_uis.len() as f64 / columns_count as f64).ceil() as usize;

    let vertical_metrics_chunks_count = match screen_width {
        ScreenWidth::XSmall(_width) => metric_sections_uis.len(),
        ScreenWidth::Small(_width) => minimum_number_of_rows_required,
        ScreenWidth::Medium(_width) => minimum_number_of_rows_required,
        ScreenWidth::Large(_width) => minimum_number_of_rows_required,
        ScreenWidth::XLarge(_width) => minimum_number_of_rows_required,
    };

    let width_per_box = 100.0 / horizontal_metrics_chunks_count_per_vertical_chunk as f64;

    let metric_box_width_percent = match screen_width {
        ScreenWidth::XSmall(_width) => 100.0,
        ScreenWidth::Small(_width) => width_per_box,
        ScreenWidth::Medium(_width) => width_per_box,
        ScreenWidth::Large(_width) => width_per_box,
        ScreenWidth::XLarge(_width) => width_per_box,
    };
    let height_per_box = 100.0 / vertical_metrics_chunks_count as f64;

    let metric_box_height_constraint: Constraint = match screen_width {
        ScreenWidth::XSmall(_width) => Constraint::Percentage(height_per_box as u16),
        ScreenWidth::Small(_width) => Constraint::Percentage(height_per_box as u16),
        ScreenWidth::Medium(_width) => Constraint::Percentage(height_per_box as u16),
        ScreenWidth::Large(_width) => Constraint::Percentage(height_per_box as u16),
        ScreenWidth::XLarge(_width) => Constraint::Percentage(height_per_box as u16),
    };

    let vertical_constraints = vec![metric_box_height_constraint; vertical_metrics_chunks_count];
    let vertical_metrics_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints.as_ref())
        .margin(1)
        .split(metrics_chunk);

    // RENDER ==================================================================
    rect.render_widget(title_component, title_chunk);
    rect.render_widget(metrics_block, metrics_chunk);
    // rener each metric box
    vertical_metrics_chunks.iter().enumerate().for_each(
        |(vertical_metrics_chunk_index, vertical_metric_chunk)| {
            let horizontal_constraints =
                vec![
                    Constraint::Percentage(metric_box_width_percent as u16);
                    horizontal_metrics_chunks_count_per_vertical_chunk
                ];
            let horizontal_metrics_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(horizontal_constraints.as_ref())
                .split(*vertical_metric_chunk);
            horizontal_metrics_chunks.iter().enumerate().for_each(
                |(i, _horizontal_metric_chunk)| {
                    let index_of_metric_blocks = (vertical_metrics_chunk_index
                        * horizontal_metrics_chunks_count_per_vertical_chunk)
                        + i;
                    let has_iterated_over_all_metric_blocks: bool =
                        index_of_metric_blocks < metric_sections_uis.len();
                    if has_iterated_over_all_metric_blocks {
                        let (metric_block_label, metric_block_value) =
                            metric_sections_uis[index_of_metric_blocks].clone();
                        rect.render_widget(
                            metric_block_label,
                            horizontal_metrics_chunks[i].clone(),
                        );
                        rect.render_widget(metric_block_value, horizontal_metrics_chunks[i].clone())
                    }
                },
            )
        },
    );
}

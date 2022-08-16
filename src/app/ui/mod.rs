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
                .style(Style::default().fg(Color::Blue))
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

    let metric_blocks = match app.state() {
        AppState::Init => vec![],
        AppState::Initialized(initialized_data) => {
            vec![
                market_data_component(&initialized_data, app.state()),
                blockchain_data_component(&initialized_data, app.state()),
                transactions_data_component(initialized_data, app.state()),
                difficulty_data_component(initialized_data, app.state()),
                mining_data_component(initialized_data, app.state()),
                market_data_component(&initialized_data, app.state()),
                difficulty_data_component(initialized_data, app.state()),
            ]
        }
    };

    let is_small_screen = size.width < 200;
    let max_columns_count = 5;
    let horizontal_metrics_chunks_count_per_vertical_chunk = if is_small_screen {
        1
    } else {
        max_columns_count
    };

    let vertical_metrics_chunks_count = if is_small_screen {
        metric_blocks.len()
    } else {
        (metric_blocks.len() as f64 / horizontal_metrics_chunks_count_per_vertical_chunk as f64)
            .ceil() as usize
    };

    let metric_box_width_percent = if is_small_screen {
        100.0
    } else {
        100.0 / horizontal_metrics_chunks_count_per_vertical_chunk as f64
    };
    let metric_box_height_percent = if is_small_screen {
        100.0 / vertical_metrics_chunks_count as f64
    } else {
        100.0 / vertical_metrics_chunks_count as f64
    };

    let vertical_constraints = vec![
        Constraint::Percentage(metric_box_height_percent as u16);
        vertical_metrics_chunks_count
    ];
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
                        index_of_metric_blocks < metric_blocks.len();
                    if has_iterated_over_all_metric_blocks {
                        let (metric_block_label, metric_block_value) =
                            metric_blocks[index_of_metric_blocks].clone();
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

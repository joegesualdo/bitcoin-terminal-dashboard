#![allow(unused)]
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;

use self::components::metrics_section_component::loading_component::metrics_section_loading_component;
use self::components::metrics_section_component::{
    blockchain_data_component, difficulty_data_component, market_data_component,
    mining_data_component, transactions_data_component,
};
use super::actions::Actions;
use super::state::{AppState, InitializedData, Stats};
use crate::app::state::FetchStatus;
use crate::app::App;
use crate::utils::format_duration;
mod components;

const BITCOIN_ORANGE_COLOR: Color = Color::Rgb(242, 169, 0);

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

const ASCII_ART: &str = r#"
  ___ _ _          _         ___          _    
 | _ |_) |_ __ ___(_)_ _    |   \ __ _ __| |_  
 | _ \ |  _/ _/ _ \ | ' \   | |) / _` (_-< ' \ 
 |___/_|\__\__\___/_|_||_|  |___/\__,_/__/_||_|
"#;
const ASCII_ART_2: &str = r#"
  ____ _______ _____   _____           _____ _    _ 
 |  _ \__   __/ ____| |  __ \   /\    / ____| |  | |
 | |_) | | | | |      | |  | | /  \  | (___ | |__| |
 |  _ <  | | | |      | |  | |/ /\ \  \___ \|  __  |
 | |_) | | | | |____  | |__| / ____ \ ____) | |  | |
 |____/  |_|  \_____| |_____/_/    \_\_____/|_|  |_|
"#;

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

fn help_section_component(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Percentage(90), Constraint::Max(10)])
        .column_spacing(0)
}

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    // Surrounding block
    let metrics_block = Block::default()
        .borders(Borders::ALL)
        .title("Metrics")
        .border_type(BorderType::Rounded);
    // rect.render_widget(metrics_block, size);

    let app_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(10)].as_ref())
        .split(size);

    // Title
    let title_component = title_component();
    rect.render_widget(title_component, app_chunks[0]);

    let metrics_chunks = app_chunks[1];
    //

    // metrics
    rect.render_widget(metrics_block, metrics_chunks);

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
        (100.0 / vertical_metrics_chunks_count as f64)
    } else {
        (100.0 / vertical_metrics_chunks_count as f64)
    };

    let vertical_constraints = vec![
        Constraint::Percentage(metric_box_height_percent as u16);
        vertical_metrics_chunks_count
    ];
    let vertical_metrics_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints.as_ref())
        .margin(1)
        .split(metrics_chunks);

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
            horizontal_metrics_chunks
                .iter()
                .enumerate()
                .for_each(|(i, horizontal_metric_chunk)| {
                    let index_of_metric_blocks = (vertical_metrics_chunk_index
                        * horizontal_metrics_chunks_count_per_vertical_chunk)
                        + i;
                    if (index_of_metric_blocks < (metric_blocks.len())) {
                        let (metric_block_label, metric_block_value) =
                            metric_blocks[index_of_metric_blocks].clone();
                        rect.render_widget(
                            metric_block_label,
                            horizontal_metrics_chunks[i].clone(),
                        );
                        rect.render_widget(metric_block_value, horizontal_metrics_chunks[i].clone())
                    }
                })
        },
    );

    //let help_section_component = help_section_component(app.actions());
    //rect.render_widget(help_section_component, body_chunks[1]);
}

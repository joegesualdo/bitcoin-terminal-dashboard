#![allow(unused)]
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;

use self::components::metrics_section_component::loading_component::metrics_section_loading_component;
use self::components::metrics_section_component::metrics_section_component;
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

fn title_component<'a>() -> Paragraph<'a> {
    Paragraph::new("Bitcoin Terminal Dashboard")
        .style(Style::default().fg(BITCOIN_ORANGE_COLOR))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue))
                .border_type(BorderType::Rounded),
        )
}

fn body_component<'a>(loading: bool, state: &'a AppState) -> Paragraph<'a> {
    match state {
        AppState::Init => metrics_section_loading_component(),
        AppState::Initialized(initialized_data) => {
            metrics_section_component(initialized_data, state)
        }
    }
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
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
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
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    // Title
    let title_component = title_component();
    rect.render_widget(title_component, app_chunks[0]);

    // Body: metrics & Help
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(app_chunks[1]);

    //let body_component = body_component(false, app.state());
    //
    let metrics_chunk = body_chunks[0];
    let help_chunk = body_chunks[1];

    // metrics
    rect.render_widget(metrics_block, body_chunks[0]);

    let market_data_block = Block::default()
        .borders(Borders::ALL)
        .title("Market Data")
        .border_type(BorderType::Rounded);
    let blockchain_data = Block::default()
        .borders(Borders::ALL)
        .title("Blockchain data")
        .border_type(BorderType::Rounded);
    let mining_data = Block::default()
        .borders(Borders::ALL)
        .title("Mining data")
        .border_type(BorderType::Rounded);
    let difficulty_data = Block::default()
        .borders(Borders::ALL)
        .title("Difficulty data")
        .border_type(BorderType::Rounded);
    let random_data = Block::default()
        .borders(Borders::ALL)
        .title("Random data")
        .border_type(BorderType::Rounded);

    let metric_blocks = vec![
        market_data_block,
        blockchain_data,
        mining_data,
        difficulty_data,
        random_data,
    ];

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
        let a = (metric_blocks.len() as f64
            / horizontal_metrics_chunks_count_per_vertical_chunk as f64)
            .ceil() as usize;
        a
    };

    let metric_box_width_percent = if is_small_screen {
        100.0
    } else {
        100.0 / horizontal_metrics_chunks_count_per_vertical_chunk as f64
    };
    let metric_box_height_percent = if is_small_screen {
        (100.0 / vertical_metrics_chunks_count as f64).floor()
    } else {
        (100.0 / vertical_metrics_chunks_count as f64).floor()
    };

    let vertical_constraints = vec![
        Constraint::Percentage(metric_box_height_percent as u16);
        vertical_metrics_chunks_count
    ];
    let vertical_metrics_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vertical_constraints.as_ref())
        .split(metrics_chunk);

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
                        rect.render_widget(
                            metric_blocks[index_of_metric_blocks].clone(),
                            horizontal_metrics_chunks[i].clone(),
                        )
                    }
                })
        },
    );

    let help_section_component = help_section_component(app.actions());
    rect.render_widget(help_section_component, body_chunks[1]);
}

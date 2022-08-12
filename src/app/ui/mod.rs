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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    // Title
    let title_component = title_component();
    rect.render_widget(title_component, chunks[0]);

    // Body & Help
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(chunks[1]);

    let body_component = body_component(false, app.state());
    rect.render_widget(body_component, body_chunks[0]);

    let help_section_component = help_section_component(app.actions());
    rect.render_widget(help_section_component, body_chunks[1]);
}

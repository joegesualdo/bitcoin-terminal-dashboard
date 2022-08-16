use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, BorderType, Borders, Paragraph};

mod average_block_time_for_last_2016_blocks;
mod average_block_time_since_last_difficulty_adjustement;
mod average_fees_per_block_over_last_2016_blocks;
mod average_fees_per_block_over_last_24_hours;
mod bitcoin_price;
mod block_count_until_retarget;
mod block_height_metric;
mod block_subsidy_of_most_recent_block;
mod blocks_mined_over_last_24_hours;
mod chain_size;
mod current_difficulty_epoch;
mod difficulty;
mod estimated_hash_rate_per_second_for_last_2016_blocks;
mod estimated_seconds_until_retarget;
mod fees_as_a_percent_of_reward_for_last_2016_blocks;
mod fees_as_a_percent_of_reward_for_last_24_hours;
mod header_component;
pub mod loading_component;
mod metrics_line_component;
mod new_transactions_count_over_last_30_days_metric;
mod sats_per_dollar;
mod seconds_since_new_block_metric;
mod total_fees_for_last_24_hours;
mod total_transaction_count;
mod tps_for_last_30_days;
mod utxo_set_size;

use self::average_block_time_for_last_2016_blocks::average_block_time_for_last_2016_blocks_component;
use self::average_block_time_since_last_difficulty_adjustement::average_block_time_since_last_difficulty_adjustement_component;
use self::average_fees_per_block_over_last_2016_blocks::average_fees_per_block_over_last_2016_blocks_component;
use self::average_fees_per_block_over_last_24_hours::average_fees_per_block_over_last_24_hours_component;
use self::bitcoin_price::bitcoin_price_component;
use self::block_count_until_retarget::block_count_until_retarget_component;
use self::block_height_metric::block_height_metric_component;
use self::block_subsidy_of_most_recent_block::block_subsidy_of_most_recent_block_component;
use self::blocks_mined_over_last_24_hours::blocks_mined_over_last_24_hours_component;
use self::chain_size::chain_size_metric_component;
use self::current_difficulty_epoch::current_difficulty_epoch_component;
use self::difficulty::difficulty_component;
use self::estimated_hash_rate_per_second_for_last_2016_blocks::estimated_hash_rate_per_second_for_last_2016_blocks_component;
use self::estimated_seconds_until_retarget::estimated_seconds_until_retarget_component;
use self::fees_as_a_percent_of_reward_for_last_2016_blocks::fees_as_a_percent_of_reward_for_last_2016_blocks_component;
use self::fees_as_a_percent_of_reward_for_last_24_hours::fees_as_a_percent_of_reward_for_last_24_hours_component;
use self::metrics_line_component::metric_line_component;
use self::new_transactions_count_over_last_30_days_metric::new_transactions_count_over_last_30_days_component;
use self::sats_per_dollar::sats_per_dollar_component;
use self::seconds_since_new_block_metric::seconds_since_new_block_metric_component;
use self::total_fees_for_last_24_hours::total_fees_for_last_24_hours_component;
use self::total_transaction_count::total_transactions_count_component;
use self::tps_for_last_30_days::tps_for_last_30_days_component;
use self::utxo_set_size::utxo_set_size_component;
use crate::app::state::{FetchStatus, InitializedData};

fn components_to_label_and_value_spans<'a>(
    components: Vec<Vec<Spans<'a>>>,
) -> (Vec<Spans<'a>>, Vec<Spans<'a>>) {
    let labels_spans = components
        .iter()
        .map(|component_span| component_span[0].clone())
        .collect();
    let value_spans = components
        .iter()
        .map(|component_span| component_span[1].clone())
        .collect();
    (labels_spans, value_spans)
}
fn components_to_key_value_paragraphs<'a>(
    components: Vec<Vec<Spans<'a>>>,
    title: &'a str,
) -> (Paragraph<'a>, Paragraph<'a>) {
    let (labels_spans, values_spans) = components_to_label_and_value_spans(components);

    let market_data_block_1 = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let market_data_block_2 = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title_alignment(Alignment::Center)
        .title(title.clone())
        .border_type(BorderType::Rounded);

    let label_paragraph = Paragraph::new(labels_spans)
        .style(Style::default())
        .block(market_data_block_1)
        .alignment(Alignment::Left);

    let value_paragraph = Paragraph::new(values_spans)
        .style(Style::default())
        .block(market_data_block_2)
        .alignment(Alignment::Right);
    return (label_paragraph, value_paragraph);
}

pub fn metric_line_fetch_status_component<'a, T, U>(
    label: &'a str,
    data_to_be_fetched: &'a FetchStatus<T>,
    format: U,
) -> Vec<Spans<'a>>
where
    U: Fn(&T) -> String,
{
    let data: String = match data_to_be_fetched {
        FetchStatus::Complete(data) => format(data),
        FetchStatus::NotStarted => "---".to_string(),
        FetchStatus::InProgress(maybe_old_value) => match maybe_old_value {
            Some(old_value) => format!("↻ {}", format(old_value)),
            None => "🔄".to_string(),
        },
    };
    let data_text = format!("{}", data);
    let data_spans = metric_line_component(label, data_text);
    data_spans
}

pub fn market_data_component<'a>(
    initialized_data: &'a InitializedData,
) -> (Paragraph<'a>, Paragraph<'a>) {
    components_to_key_value_paragraphs(
        vec![
            bitcoin_price_component(initialized_data),
            sats_per_dollar_component(initialized_data),
        ],
        " Market 📈 ",
    )
}

pub fn blockchain_data_component<'a>(
    initialized_data: &'a InitializedData,
) -> (Paragraph<'a>, Paragraph<'a>) {
    components_to_key_value_paragraphs(
        vec![
            block_height_metric_component(initialized_data),
            seconds_since_new_block_metric_component(initialized_data),
            new_transactions_count_over_last_30_days_component(initialized_data),
            average_block_time_for_last_2016_blocks_component(initialized_data),
            chain_size_metric_component(initialized_data),
            utxo_set_size_component(initialized_data),
        ],
        " Blockchain ⛓️  ",
    )
}

pub fn transactions_data_component<'a>(
    initialized_data: &'a InitializedData,
) -> (Paragraph<'a>, Paragraph<'a>) {
    components_to_key_value_paragraphs(
        vec![
            total_transactions_count_component(initialized_data),
            tps_for_last_30_days_component(initialized_data),
            total_fees_for_last_24_hours_component(initialized_data),
        ],
        " Transactions 🖊️ ",
    )
}
pub fn difficulty_data_component<'a>(
    initialized_data: &'a InitializedData,
) -> (Paragraph<'a>, Paragraph<'a>) {
    components_to_key_value_paragraphs(
        vec![
            difficulty_component(initialized_data),
            current_difficulty_epoch_component(initialized_data),
            block_count_until_retarget_component(initialized_data),
            estimated_seconds_until_retarget_component(initialized_data),
            average_block_time_since_last_difficulty_adjustement_component(initialized_data),
        ],
        " Difficulty ⚙️  ",
    )
}

pub fn mining_data_component<'a>(
    initialized_data: &'a InitializedData,
) -> (Paragraph<'a>, Paragraph<'a>) {
    components_to_key_value_paragraphs(
        vec![
            estimated_hash_rate_per_second_for_last_2016_blocks_component(initialized_data),
            block_subsidy_of_most_recent_block_component(initialized_data),
            blocks_mined_over_last_24_hours_component(initialized_data),
            average_fees_per_block_over_last_24_hours_component(initialized_data),
            average_fees_per_block_over_last_2016_blocks_component(initialized_data),
            fees_as_a_percent_of_reward_for_last_24_hours_component(initialized_data),
            fees_as_a_percent_of_reward_for_last_2016_blocks_component(initialized_data),
        ],
        " Mining ⚒️  ",
    )
}

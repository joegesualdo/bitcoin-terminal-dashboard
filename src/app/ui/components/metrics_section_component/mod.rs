use bitcoin_node_query::get_estimated_hash_rate_per_second_for_block_since_last_difficulty_change;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;

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
use self::header_component::metric_section_header_component;
use self::metrics_line_component::metric_line_component;
use self::new_transactions_count_over_last_30_days_metric::new_transactions_count_over_last_30_days_component;
use self::sats_per_dollar::sats_per_dollar_component;
use self::seconds_since_new_block_metric::seconds_since_new_block_metric_component;
use self::total_fees_for_last_24_hours::total_fees_for_last_24_hours_component;
use self::total_transaction_count::total_transactions_count_component;
use self::tps_for_last_30_days::tps_for_last_30_days_component;
use self::utxo_set_size::utxo_set_size_component;
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
    let chain_size = chain_size_metric_component(initialized_data);
    let utxo_set_size = utxo_set_size_component(initialized_data);
    let total_transaction_count = total_transactions_count_component(initialized_data);
    let tps_for_last_30_days = tps_for_last_30_days_component(initialized_data);
    let total_fees_for_last_24_hours = total_fees_for_last_24_hours_component(initialized_data);
    let difficulty = difficulty_component(initialized_data);
    let current_difficulty_epoch = current_difficulty_epoch_component(initialized_data);
    let block_count_until_retarget = block_count_until_retarget_component(initialized_data);
    let estimated_seconds_until_retarget =
        estimated_seconds_until_retarget_component(initialized_data);
    let average_block_time_since_last_difficulty_adjustement =
        average_block_time_since_last_difficulty_adjustement_component(initialized_data);
    let bitcoin_price = bitcoin_price_component(initialized_data);
    let sats_per_dollar = sats_per_dollar_component(initialized_data);
    let estimated_hash_rate_per_second_for_last_2016_blocks =
        estimated_hash_rate_per_second_for_last_2016_blocks_component(initialized_data);
    let block_subsidy_of_most_recent_block =
        block_subsidy_of_most_recent_block_component(initialized_data);
    let blocks_mined_over_last_24_hours =
        blocks_mined_over_last_24_hours_component(initialized_data);
    let average_fees_per_block_over_last_24_hours =
        average_fees_per_block_over_last_24_hours_component(initialized_data);
    let average_fees_per_block_over_last_2016_blocks =
        average_fees_per_block_over_last_2016_blocks_component(initialized_data);
    let fees_as_a_percent_of_reward_for_last_2016_blocks =
        fees_as_a_percent_of_reward_for_last_2016_blocks_component(initialized_data);
    let fees_as_a_percent_of_reward_for_last_24_hours =
        fees_as_a_percent_of_reward_for_last_24_hours_component(initialized_data);

    let paragraphs = vec![
        Spans(section_header),
        Spans(bitcoin_price),
        Spans(sats_per_dollar),
        Spans(blockchain_height),
        Spans(seconds_since_last_block),
        Spans(transactions_count_over_last_30_days),
        Spans(average_block_time_for_last_2016_blocks),
        Spans(chain_size),
        Spans(utxo_set_size),
        Spans(total_transaction_count),
        Spans(tps_for_last_30_days),
        Spans(total_fees_for_last_24_hours),
        Spans(difficulty),
        Spans(current_difficulty_epoch),
        Spans(block_count_until_retarget),
        Spans(estimated_seconds_until_retarget),
        Spans(average_block_time_since_last_difficulty_adjustement),
        Spans(estimated_hash_rate_per_second_for_last_2016_blocks),
        Spans(block_subsidy_of_most_recent_block),
        Spans(blocks_mined_over_last_24_hours),
        Spans(average_fees_per_block_over_last_24_hours),
        Spans(average_fees_per_block_over_last_2016_blocks),
        Spans(fees_as_a_percent_of_reward_for_last_24_hours),
        Spans(fees_as_a_percent_of_reward_for_last_2016_blocks),
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

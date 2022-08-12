use std::cell::RefCell;
use std::env;
use std::io::stdout;
use std::rc::Rc;
use std::thread::{self, sleep};
use std::time::Duration;
mod utils;

use app::state::FetchStatus;
use app::{App, AppReturn};
use bitcoin_node_query;
use eyre::Result;
use inputs::events::Events;
use inputs::{FetchEvent, InputEvent, Resource};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;

pub mod app;
pub mod inputs;

fn client(url: &str, user: &str, pass: &str) -> Result<Client, simple_http::Error> {
    let t = SimpleHttpTransport::builder()
        .url(url)?
        .auth(user, Some(pass))
        .build();
    Ok(Client::with_transport(t))
}
fn get_client() -> Client {
    let password = env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
    let username = env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
    let client = client("127.0.0.1:8332", &username, &password).expect("failed to create client");
    client
}

fn start_loop_for_fetching_seconds_since_last_block(events: &Events) {
    let tx = events.tx.clone();
    let event_tx_for_seconds_since_last_block = tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        // TODO: Handle error
        let _ = event_tx_for_seconds_since_last_block
            .clone()
            .send(InputEvent::FetchResource(Resource::SecondsSinceLastBlock(
                FetchEvent::Start,
            )));
        let seconds_since_last_block = bitcoin_node_query::get_time_since_last_block_in_seconds(&c);
        // TODO: Handle error
        let _ = event_tx_for_seconds_since_last_block
            .clone()
            .send(InputEvent::FetchResource(Resource::SecondsSinceLastBlock(
                FetchEvent::Complete(seconds_since_last_block as u64),
            )));
        sleep(Duration::from_secs(1));
    });
}

fn start_loop_for_fetching_new_block_height(events: &Events) {
    let tx = events.tx.clone();
    let event_tx_for_new_block_height = tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        // TODO: Handle error
        let _ = event_tx_for_new_block_height
            .clone()
            .send(InputEvent::FetchResource(Resource::NewBlockHeight(
                FetchEvent::Start,
            )));
        let block_height = bitcoin_node_query::get_block_height(&c);
        // TODO: Handle error
        let _ = event_tx_for_new_block_height
            .clone()
            .send(InputEvent::FetchResource(Resource::NewBlockHeight(
                FetchEvent::Complete(block_height as u64),
            )));
        sleep(Duration::from_secs(1));
    });
}
fn start_loop_for_fetching_transactions_count_over_last_30_days(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx.clone().send(InputEvent::FetchResource(
            Resource::TransactionsCountOverLast30Days(FetchEvent::Start),
        ));
        let transactions_count_over_last_30_days =
            bitcoin_node_query::get_transactions_count_over_last_30_days(&c);
        let _ = tx.clone().send(InputEvent::FetchResource(
            Resource::TransactionsCountOverLast30Days(FetchEvent::Complete(
                transactions_count_over_last_30_days,
            )),
        ));
        sleep(Duration::from_secs(5 * 60));
    });
}
fn start_loop_for_fetching_average_block_time_for_last_2016_blocks(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx.clone().send(InputEvent::FetchResource(
            Resource::AverageBlockTimeForLast2016Blocks(FetchEvent::Start),
        ));
        let average_block_time_for_last_2016_blocks =
            bitcoin_node_query::get_average_block_time_for_last_2016_blocks(&c);
        let _ = tx.clone().send(InputEvent::FetchResource(
            Resource::AverageBlockTimeForLast2016Blocks(FetchEvent::Complete(
                average_block_time_for_last_2016_blocks,
            )),
        ));
        sleep(Duration::from_secs(5 * 60));
    });
}

fn start_loop_for_fetching_chain_size(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::ChainSize(
                FetchEvent::Start,
            )));
        let chain_size = bitcoin_node_query::get_chain_size(&c);
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::ChainSize(
                FetchEvent::Complete(chain_size),
            )));
        sleep(Duration::from_secs(5 * 60));
    });
}
fn start_loop_for_fetching_utxo_set_size(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::UtxoSetSize(
                FetchEvent::Start,
            )));
        let utxo_set_size = bitcoin_node_query::get_utxo_set_size(&c);
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::UtxoSetSize(
                FetchEvent::Complete(utxo_set_size),
            )));
        sleep(Duration::from_secs(5 * 60));
    });
}
fn start_loop_for_fetching_total_transaction_count(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::TotalTransactionCount(
                FetchEvent::Start,
            )));
        let total_transactions_count = bitcoin_node_query::get_total_transactions_count(&c);
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::TotalTransactionCount(
                FetchEvent::Complete(total_transactions_count),
            )));
        sleep(Duration::from_secs(5 * 60));
    });
}

fn start_loop_for_fetching_tps_for_last_30_days(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::TpsForLast30Days(
                FetchEvent::Start,
            )));
        let tps_for_last_30_days = bitcoin_node_query::get_tps_for_last_30_days(&c);
        let _ = tx
            .clone()
            .send(InputEvent::FetchResource(Resource::TpsForLast30Days(
                FetchEvent::Complete(tps_for_last_30_days),
            )));
        sleep(Duration::from_secs(5 * 60));
    });
}
fn start_loop_for_fetching_total_fees_for_last_24_hours(events: &Events) {
    let tx = events.tx.clone();
    let c = get_client();
    thread::spawn(move || loop {
        let _ = tx.clone().send(InputEvent::FetchResource(
            Resource::TotalFeesForLast24Hours(FetchEvent::Start),
        ));
        let total_fees_for_last_24_hours = bitcoin_node_query::get_total_fee_for_24_hours(&c);
        let _ = tx.clone().send(InputEvent::FetchResource(
            Resource::TotalFeesForLast24Hours(FetchEvent::Complete(total_fees_for_last_24_hours)),
        ));
        sleep(Duration::from_secs(5 * 60));
    });
}

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    start_loop_for_fetching_seconds_since_last_block(&events);
    start_loop_for_fetching_transactions_count_over_last_30_days(&events);
    start_loop_for_fetching_new_block_height(&events);
    start_loop_for_fetching_average_block_time_for_last_2016_blocks(&events);
    start_loop_for_fetching_chain_size(&events);
    // TODO: this is erroring out
    //start_loop_for_fetching_utxo_set_size(&events);
    start_loop_for_fetching_total_transaction_count(&events);
    start_loop_for_fetching_tps_for_last_30_days(&events);
    start_loop_for_fetching_total_fees_for_last_24_hours(&events);

    loop {
        let mut app = app.borrow_mut();

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // Handle inputs
        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
            InputEvent::NextBlockFound(block_height) => app.update_on_new_block_found(block_height),
            InputEvent::FetchResource(resource) => app.on_fetch_resource(resource),
        };
        // Check if we should exit
        if result == AppReturn::Exit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

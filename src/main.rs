#![allow(dead_code)]
#![allow(unused_imports)]
use chrono::{DateTime, TimeZone, Utc};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use std::time::{Duration, SystemTime};
use std::{env, time::SystemTimeError};

use bitcoin_node_query::{
    get_average_block_time, get_block_height, get_chain_size, get_time_since_last_block_in_seconds,
    get_total_fee_for_24_hours, get_total_transactions_count, get_tps_for_last_30_days,
    get_transactions_count_over_last_30_days, get_utxo_set_size,
};

fn readable(mut ss: String) -> String {
    let split_string = ss.split_once(".");
    if let Some((mut first_string, second_string)) = split_string {
        let mut o_s: String = first_string.to_string().clone();
        let mut s = String::new();
        let mut negative = false;
        let values: Vec<char> = o_s.chars().collect();
        if values[0] == '-' {
            o_s.remove(0);
            negative = true;
        }
        for (i, char) in o_s.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                s.insert(0, ',');
            }
            s.insert(0, char);
        }
        if negative {
            s.insert(0, '-');
        }
        return format!("{}.{}", s, second_string);
    } else {
        let mut o_s: String = ss.clone();
        let mut s = String::new();
        let mut negative = false;
        let values: Vec<char> = o_s.chars().collect();
        if values[0] == '-' {
            o_s.remove(0);
            negative = true;
        }
        for (i, char) in o_s.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                s.insert(0, ',');
            }
            s.insert(0, char);
        }
        if negative {
            s.insert(0, '-');
        }
        return s;
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

fn client(url: &str, user: &str, pass: &str) -> Result<Client, simple_http::Error> {
    let t = SimpleHttpTransport::builder()
        .url(url)?
        .auth(user, Some(pass))
        .build();
    Ok(Client::with_transport(t))
}
pub fn format_duration(seconds: i64) -> String {
    let seconds_formatted = seconds % 60;
    let minutes_formatted = (seconds / 60) % 60;
    format!("{:#?}:{:#?}", minutes_formatted, seconds_formatted)
}

fn main() {
    while true {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let average_exchange_price = bitcoin_price::get_average_exchange_spot_price();
        let average_exchange_price_rounded = round(average_exchange_price, 2);

        let average_exchange_price_string = format!("{:.02}", average_exchange_price_rounded);
        println!(
            "average_exchange_price: {}",
            readable(average_exchange_price_string)
        );
        /////////////////////////////////////////////////////////////////////
        //////////Blockchain Data //////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////
        let password =
            env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
        let username =
            env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
        let client =
            client("127.0.0.1:8332", &username, &password).expect("failed to create client");

        let block_height = get_block_height(&client);
        println!("BLOCK HEIGHT: {}", readable(block_height.to_string()));

        let seconds_since_last_block = get_time_since_last_block_in_seconds(&client);
        println!(
            "TIME SINCE LAST BLOCK: {}",
            format_duration(seconds_since_last_block)
        );

        let average_seconds_per_block = get_average_block_time(&client);
        println!(
            "AVERAGE BLOCK TIME (2016): {}",
            format_duration(average_seconds_per_block as i64)
        );

        // Errors out
        // let total_money_supply = get_total_money_supply(&client);
        // println!("TOTAL MONEY SUPPLY: {:#?}", total_money_supply);

        let chain_size = get_chain_size(&client);
        let chain_size_in_gbs = chain_size as f64 / 1_000_000_000.0;
        println!("CHAIN SIZE: {:.2}GB", chain_size_in_gbs);

        // Errors out
        // let utxo_set_size = get_utxo_set_size(&client);
        // println!("UTXO SET SIZE: {:#?}", utxo_set_size);

        /////////////////////////////////////////////////////////////////////
        //////////Blockchain Data //////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////
        let total_transactions_count = get_total_transactions_count(&client);
        println!(
            "TOTAL TRANSACTIONS COUNT: {}",
            readable(total_transactions_count.to_string())
        );

        let tps_for_last_30_days = get_tps_for_last_30_days(&client);
        println!(
            "TRANSACTIONS PER SECOND (last 30 days): {:.2} tx/s",
            tps_for_last_30_days
        );

        // takes a long time
        let transactions_count_over_last_30_days =
            get_transactions_count_over_last_30_days(&client);
        println!(
            "TRANSACTIONS COUNT OVER LAST 30 DAYS: {}",
            readable(transactions_count_over_last_30_days.to_string())
        );

        // takes a long time
        let total_fee_for_24_hours = get_total_fee_for_24_hours(&client);
        println!(
            "TOTAL FEE FOR LAST 24 hours: {:.2}",
            total_fee_for_24_hours as f64 / 100_000_000.0
        );
    }
}

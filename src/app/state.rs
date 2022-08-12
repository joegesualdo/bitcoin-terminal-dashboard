use std::thread::sleep;
use std::time::Duration;

use crate::inputs::{FetchEvent, InputEvent, Resource};

#[derive(Clone)]
pub enum FetchStatus<T> {
    NotStarted,
    InProgress(Option<T>),
    Complete(T),
}

#[derive(Clone)]
pub struct Stats {
    pub block_height: FetchStatus<u64>,
    pub last_block_time: u64,
    pub average_block_time: u64,
    pub seconds_since_last_block: FetchStatus<u64>,
    pub transactions_count_over_last_30_days: FetchStatus<u64>,
    pub average_block_time_for_last_2016_blocks: FetchStatus<u64>,
    pub chain_size: FetchStatus<u64>,
    pub utxo_set_size: FetchStatus<u64>,
    pub total_transactions_count: FetchStatus<u64>,
    pub tps_for_last_30_days: FetchStatus<f64>,
    pub total_fees_for_last_24_hours: FetchStatus<u64>,
}

#[derive(Clone)]
pub struct InitializedData {
    pub duration: Duration,
    pub counter_sleep: u32,
    pub counter_tick: u64,
    pub stats: Stats,
    pub newest_block_found_height: Option<u64>,
}

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized(InitializedData),
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        let block_height = FetchStatus::NotStarted;
        let last_block_time = 38999993832;
        let average_block_time = 9600;
        let seconds_since_last_block = FetchStatus::NotStarted;
        let transactions_count_over_last_30_days = FetchStatus::NotStarted;
        let average_block_time_for_last_2016_blocks = FetchStatus::NotStarted;
        let chain_size = FetchStatus::NotStarted;
        let utxo_set_size = FetchStatus::NotStarted;
        let total_transactions_count = FetchStatus::NotStarted;
        let tps_for_last_30_days = FetchStatus::NotStarted;
        let total_fees_for_last_24_hours = FetchStatus::NotStarted;
        Self::Initialized(InitializedData {
            duration,
            counter_sleep,
            counter_tick,
            stats: Stats {
                block_height,
                last_block_time,
                average_block_time,
                chain_size,
                seconds_since_last_block,
                transactions_count_over_last_30_days,
                average_block_time_for_last_2016_blocks,
                utxo_set_size,
                total_transactions_count,
                tps_for_last_30_days,
                total_fees_for_last_24_hours,
            },
            newest_block_found_height: None,
        })
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized(InitializedData { counter_sleep, .. }) = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized(InitializedData { counter_tick, .. }) = self {
            *counter_tick += 1;
        }
    }
    pub fn handle_new_block_found(&mut self, block_height: u64) {
        if let Self::Initialized(InitializedData {
            newest_block_found_height,
            ..
        }) = self
        {
            *newest_block_found_height = Some(block_height);
        }
    }
    pub fn handle_fetch_resource(&mut self, resource: Resource) {
        if let Self::Initialized(InitializedData {
            stats:
                Stats {
                    block_height,
                    seconds_since_last_block,
                    transactions_count_over_last_30_days,
                    average_block_time_for_last_2016_blocks,
                    chain_size,
                    utxo_set_size,
                    total_transactions_count,
                    tps_for_last_30_days,
                    total_fees_for_last_24_hours,
                    ..
                },
            ..
        }) = self
        {
            match resource {
                Resource::NewBlockHeight(event) => match event {
                    FetchEvent::Start => {
                        *block_height = FetchStatus::InProgress(match block_height {
                            FetchStatus::Complete(old_value) => Some(*old_value),
                            FetchStatus::NotStarted => None,
                            FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                    // InProgress
                        })
                    }
                    FetchEvent::Complete(new_block_height) => {
                        *block_height = FetchStatus::Complete(new_block_height);
                    }
                },
                Resource::SecondsSinceLastBlock(event) => match event {
                    FetchEvent::Start => {
                        *seconds_since_last_block =
                            FetchStatus::InProgress(match seconds_since_last_block {
                                FetchStatus::Complete(old_value) => Some(*old_value),
                                FetchStatus::NotStarted => None,
                                FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                        // InProgress
                            })
                    }
                    FetchEvent::Complete(new_seconds_since_last_block) => {
                        *seconds_since_last_block =
                            FetchStatus::Complete(new_seconds_since_last_block);
                    }
                },
                Resource::TransactionsCountOverLast30Days(event) => match event {
                    FetchEvent::Start => {
                        *transactions_count_over_last_30_days =
                            FetchStatus::InProgress(match transactions_count_over_last_30_days {
                                FetchStatus::Complete(old_value) => Some(*old_value),
                                FetchStatus::NotStarted => None,
                                FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                        // InProgress
                            })
                    }
                    FetchEvent::Complete(new_transactions_count_over_last_30_days) => {
                        *transactions_count_over_last_30_days =
                            FetchStatus::Complete(new_transactions_count_over_last_30_days);
                    }
                },
                Resource::AverageBlockTimeForLast2016Blocks(event) => match event {
                    FetchEvent::Start => {
                        *average_block_time_for_last_2016_blocks =
                            FetchStatus::InProgress(match average_block_time_for_last_2016_blocks {
                                FetchStatus::Complete(old_value) => Some(*old_value),
                                FetchStatus::NotStarted => None,
                                FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                        // InProgress
                            })
                    }
                    FetchEvent::Complete(new_average_block_time_for_last_2016_blocks) => {
                        *average_block_time_for_last_2016_blocks =
                            FetchStatus::Complete(new_average_block_time_for_last_2016_blocks);
                    }
                },
                Resource::ChainSize(event) => match event {
                    FetchEvent::Start => {
                        *chain_size = FetchStatus::InProgress(match chain_size {
                            FetchStatus::Complete(old_value) => Some(*old_value),
                            FetchStatus::NotStarted => None,
                            FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                    // InProgress
                        })
                    }
                    FetchEvent::Complete(new_chain_size) => {
                        *chain_size = FetchStatus::Complete(new_chain_size);
                    }
                },
                Resource::UtxoSetSize(event) => match event {
                    FetchEvent::Start => {
                        *utxo_set_size = FetchStatus::InProgress(match utxo_set_size {
                            FetchStatus::Complete(old_value) => Some(*old_value),
                            FetchStatus::NotStarted => None,
                            FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                    // InProgress
                        })
                    }
                    FetchEvent::Complete(new_utxo_set_size) => {
                        *utxo_set_size = FetchStatus::Complete(new_utxo_set_size);
                    }
                },
                Resource::TotalTransactionCount(event) => match event {
                    FetchEvent::Start => {
                        *total_transactions_count =
                            FetchStatus::InProgress(match total_transactions_count {
                                FetchStatus::Complete(old_value) => Some(*old_value),
                                FetchStatus::NotStarted => None,
                                FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                        // InProgress
                            })
                    }
                    FetchEvent::Complete(new_total_transactions_count) => {
                        *total_transactions_count =
                            FetchStatus::Complete(new_total_transactions_count);
                    }
                },
                Resource::TpsForLast30Days(event) => match event {
                    FetchEvent::Start => {
                        *tps_for_last_30_days =
                            FetchStatus::InProgress(match tps_for_last_30_days {
                                FetchStatus::Complete(old_value) => Some(*old_value),
                                FetchStatus::NotStarted => None,
                                FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                        // InProgress
                            })
                    }
                    FetchEvent::Complete(new_tps_for_last_30_days) => {
                        *tps_for_last_30_days = FetchStatus::Complete(new_tps_for_last_30_days);
                    }
                },
                Resource::TotalFeesForLast24Hours(event) => match event {
                    FetchEvent::Start => {
                        *total_fees_for_last_24_hours =
                            FetchStatus::InProgress(match total_fees_for_last_24_hours {
                                FetchStatus::Complete(old_value) => Some(*old_value),
                                FetchStatus::NotStarted => None,
                                FetchStatus::InProgress(_) => panic!(), // We should never go from InProgress to
                                                                        // InProgress
                            })
                    }
                    FetchEvent::Complete(new_total_fees_for_last_24_hours) => {
                        *total_fees_for_last_24_hours =
                            FetchStatus::Complete(new_total_fees_for_last_24_hours);
                    }
                },
                //TransactionsCountOverLast30Days(FetchEvent<u64>),
                //AverageBlockTimeForLast2016Blocks(FetchEvent<u64>),
            }
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized(InitializedData { counter_sleep, .. }) = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized(InitializedData { counter_tick, .. }) = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized(InitializedData { duration, .. }) = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized(InitializedData { duration, .. }) = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized(InitializedData { duration, .. }) = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}

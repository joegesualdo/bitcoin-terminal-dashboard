use self::key::Key;

pub mod events;
pub mod key;

pub enum FetchEvent<T> {
    Start,
    Complete(T),
}
pub enum Resource {
    NewBlockHeight(FetchEvent<u64>),
    SecondsSinceLastBlock(FetchEvent<u64>),
    TransactionsCountOverLast30Days(FetchEvent<u64>),
    AverageBlockTimeForLast2016Blocks(FetchEvent<u64>),
    ChainSize(FetchEvent<u64>),
    UtxoSetSize(FetchEvent<u64>),
    TotalTransactionCount(FetchEvent<u64>),
    TpsForLast30Days(FetchEvent<f64>),
    TotalFeesForLast24Hours(FetchEvent<u64>),
}

pub enum InputEvent {
    FetchResource(Resource),
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
    NextBlockFound(u64),
}

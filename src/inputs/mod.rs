use self::key::Key;
use crate::app::state::FetchStatus;

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
}

pub enum InputEvent {
    FetchResource(Resource),
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
    NextBlockFound(u64),
}

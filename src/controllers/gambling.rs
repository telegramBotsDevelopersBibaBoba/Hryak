use teloxide::prelude::Dialogue;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[derive(Clone, Default)]
pub enum GuessState {
    #[default]
    Start,
    ReceiveBid,
    ReceiveNumber {
        bid: f64,
    },
    Finish {
        number: u8,
    },
}
pub type GuessDialogue = Dialogue<GuessState, InMemStorage<GuessState>>;

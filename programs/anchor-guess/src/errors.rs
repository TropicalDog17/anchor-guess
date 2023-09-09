use anchor_lang::prelude::*;

#[error_code]
pub enum AnchorGuessError {
    GameAlreadyStarted,
    OverflowError,
    InvalidWordLength,
    OutOfMove,
    ProhibitedAction,
}

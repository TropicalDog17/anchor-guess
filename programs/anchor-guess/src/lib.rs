use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod secret_words;
pub mod state;
// use cgisf_lib::{gen_sentence, SentenceConfigBuilder};
declare_id!("6vWgxZqyG7eMN4HUGzKepGLxNjQNhFSPxznTB7a5AfnP");

#[program]
pub mod anchor_guess {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>) -> Result<()> {
        instructions::setup_game::setup_game(ctx)
    }
    pub fn guess(ctx: Context<Guess>, word: String) -> Result<()> {
        instructions::guess::guess(ctx, word)
    }
    pub fn mutate_secret(ctx: Context<MutateSecret>, new_secret: String) -> Result<()> {
        instructions::mutate_secret::mutate_secret(ctx, new_secret)
    }
}

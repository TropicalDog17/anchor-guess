use crate::state::game::*;
use anchor_lang::prelude::*;
pub fn guess(ctx: Context<Guess>, word: String) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.guess(&word)
}
#[derive(Accounts)]
pub struct Guess<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

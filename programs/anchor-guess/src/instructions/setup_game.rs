use crate::state::game::*;
use anchor_lang::prelude::*;
pub fn setup_game(ctx: Context<SetupGame>) -> Result<()> {
    ctx.accounts.game.setup_game(ctx.accounts.player.key())
}
#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

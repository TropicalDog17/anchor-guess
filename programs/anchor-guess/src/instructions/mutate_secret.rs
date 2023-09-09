use crate::errors::*;
use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn mutate_secret(ctx: Context<MutateSecret>, new_secret: String) -> Result<()> {
    require_neq!(
        ctx.accounts.game.get_player_key().unwrap(),
        ctx.accounts.tester.key(),
        AnchorGuessError::ProhibitedAction
    );
    let game = &mut ctx.accounts.game;
    game.set_secret(new_secret)?;
    Ok(())
}
#[derive(Accounts)]
pub struct MutateSecret<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub tester: Signer<'info>,
}

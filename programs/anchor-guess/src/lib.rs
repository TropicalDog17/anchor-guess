pub mod secret_words;
use anchor_lang::prelude::*;
// use cgisf_lib::{gen_sentence, SentenceConfigBuilder};
declare_id!("6vWgxZqyG7eMN4HUGzKepGLxNjQNhFSPxznTB7a5AfnP");
const WORD_LENGTH: usize = 5;
const MAXIMUM_MOVES: usize = 6;
#[program]
pub mod anchor_guess {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>) -> Result<()> {
        ctx.accounts.game.setup_game(ctx.accounts.player.key())
    }
    pub fn guess(ctx: Context<Play>, word: String) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.guess(&word)
    }
    // TODO: find another way to test
    pub fn mutate_secret(ctx: Context<MutateSecret>, new_secret: String) -> Result<()> {
        require_neq!(ctx.accounts.game.player.key(), ctx.accounts.tester.key(), AnchorGuessError::ProhibitedAction);
        let game = &mut ctx.accounts.game;
        game.secret = new_secret;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Won,
    Loss,
}

#[account]
pub struct Game {
    player: Pubkey,
    move_count: u8,
    state: GameState,
    secret: String,
    guess_state: Vec<LetterGuessed>,
}
#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}
#[derive(Accounts)]
pub struct MutateSecret<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub tester: Signer<'info>,
}
impl Game {
    pub const MAXIMUM_SIZE: usize = 32 + 1 + 1 + 24;
    pub fn setup_game(&mut self, player: Pubkey) -> Result<()> {
        // TODO: fix move_count logic, because default move_count is 0, the start may be evoked multiple times before moves are performed by player
        require_eq!(self.move_count, 0, AnchorGuessError::GameAlreadyStarted);
        self.player = player;
        self.state = GameState::Active;
        self.secret = self.gen_secret().unwrap();
        self.guess_state = Vec::new();
        Ok(())
    }
    pub fn gen_secret(&self) -> Result<String> {
        let clock = Clock::get()?;
        let seed = clock.unix_timestamp as u64;
        let remainder = seed
            .checked_rem(secret_words::SECRET_WORDS.len() as u64)
            .ok_or(AnchorGuessError::OverflowError)? as usize;

        Ok(secret_words::SECRET_WORDS.get(remainder).unwrap().to_string())
    }
    fn is_active(&self) -> bool {
        self.state == GameState::Active
    }
    fn is_letter_in_secret(&self, secret: &str, letter: &char) -> bool {
        secret.contains(*letter)
    }
    fn is_letter_in_correct_position(&self, secret: &str, letter: &char, index: usize) -> bool {
        secret.chars().nth(index).unwrap() == *letter
    }
    pub fn guess(&mut self, word: &str) -> Result<()> {
        require!(self.is_active(), AnchorGuessError::OutOfMove);
        require!(word.len() == 5, AnchorGuessError::InvalidWordLength);

        // Reset guess_state for a new guess
        self.guess_state = Vec::new();
        for (index, letter) in word.chars().enumerate() {
            if self.is_letter_in_correct_position(&self.secret, &letter, index) {
                self.guess_state.push(LetterGuessed::InCorrectPosition)
            } else if self.is_letter_in_secret(&self.secret, &letter) {
                self.guess_state.push(LetterGuessed::InWordButWrongSpot)
            } else {
                self.guess_state.push(LetterGuessed::NotInWord)
            }
        }
        let winning_guess_state = [LetterGuessed::InCorrectPosition; 5].to_vec();
        if self.guess_state == winning_guess_state {
            self.state = GameState::Won
        }
        self.move_count += 1;
        if self.move_count == MAXIMUM_MOVES as u8 {
            self.state = GameState::Loss
        }
        Ok(())
    }
}
#[error_code]
pub enum AnchorGuessError {
    GameAlreadyStarted,
    OverflowError,
    InvalidWordLength,
    OutOfMove,
    ProhibitedAction,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LetterGuessed {
    InCorrectPosition,
    InWordButWrongSpot,
    NotInWord,
}

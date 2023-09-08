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
        ctx.accounts.game.start(ctx.accounts.player.key())
    }
    pub fn guess(ctx: Context<Play>, word: String) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.guess(&word)
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
impl Game {
    pub const MAXIMUM_SIZE: usize = 32 + 1 + 1 + 24;
    pub fn start(&mut self, player: Pubkey) -> Result<()> {
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
    fn is_letter_in_word(&self, word: &str, letter: &char) -> bool {
        word.contains(*letter)
    }
    fn is_letter_in_correct_position(&self, word: &str, letter: &char, index: usize) -> bool {
        word.to_string().get(index..index + 1).unwrap() == letter.to_string()
    }
    pub fn guess(&mut self, word: &str) -> Result<()> {
        require!(self.is_active(), AnchorGuessError::OutOfMove);
        require!(word.len() == 5, AnchorGuessError::InvalidWordLength);

        // Reset guess_state for a new guess

        for (index, letter) in word.chars().enumerate() {
            if self.is_letter_in_correct_position(word, &letter, index) {
                self.guess_state.push(LetterGuessed::InCorrectPosition)
            } else if self.is_letter_in_word(word, &letter) {
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
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LetterGuessed {
    InCorrectPosition,
    InWordButWrongSpot,
    NotInWord,
}

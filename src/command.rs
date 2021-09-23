use crate::game::Game;
use itertools::Itertools;
use tagiron_card::Card;

pub trait Command<T> {
    fn execute(&self, t: &mut T);
}

pub enum GameCommand {
    GoNextTurn,
    Declare { player_name: String, cards: Vec<Card> },
    SelectQuestion { index: usize },
    Initialize
}

impl Command<Game> for GameCommand {
    fn execute(&self, game: &mut Game) {
        match self {
            GameCommand::GoNextTurn => {
                if let Some(idx) = game.get_players().iter().position(|(_, retired)| *retired) {
                    game.rotate_players(idx)
                }
            }
            GameCommand::Declare { player_name, cards } => {
                let is_collect = game.get_answer().iter().sorted().zip(cards.iter().sorted()).all(|(card, other)| {
                    card == other
                });
                if is_collect {
                    game.set_winner(player_name.clone());
                } else {
                    if let Some(idx) = game.get_players().iter().position(|(name, state)| name == player_name) {
                        game.set_retire(idx);
                    }
                }
            }
            GameCommand::SelectQuestion { index } => {
                game.select_question(*index);
            }
            GameCommand::Initialize => {

            }
        }
    }
}

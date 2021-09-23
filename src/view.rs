use std::collections::HashMap;

use crate::game::{Game, Name, Question, QuestionState, Retired};
use tagiron_card::Card;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct GameView {
    players: Vec<(Name, Retired)>,
    cards: Vec<Card>,
    questions: Vec<(Question, QuestionState)>,
    winner: Option<Name>,
}

impl GameView {
    pub fn new(game: &Game, player_name: String) -> Self {
        GameView {
            players: game.get_players().clone(),
            cards: game.get_card(player_name).unwrap().clone(),
            questions: game.get_questions().clone(),
            winner: game.get_winner().clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResultView {
    winner: String,
    cards: HashMap<String, Vec<Card>>,
    answer: Vec<Card>,
}

impl ResultView {
    pub fn new(game: &Game) -> Self {
        ResultView {
            winner: game.get_winner().clone().unwrap(),
            cards: game.get_cards().clone(),
            answer: game.get_answer().clone(),
        }
    }
}

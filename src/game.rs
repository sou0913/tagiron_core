use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use tagiron_card::Card;

pub type Question = String;
pub type Retired = bool;
pub type Name = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QuestionState {
    Opened,
    Reversed,
    Selected,
    Used,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    players: Vec<(Name, Retired)>,
    cards: HashMap<Name, Vec<Card>>,
    answer: Vec<Card>,
    questions: Vec<(Question, QuestionState)>,
    winner: Option<Name>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            players: vec![],
            cards: HashMap::new(),
            answer: vec![],
            questions: vec![],
            winner: None,
        }
    }
}

impl Game {
    pub fn retired_all(&self) -> bool {
        self.players.iter().all(|(_, retired)| *retired)
    }

    pub fn get_cards(&self) -> &HashMap<String, Vec<Card>> {
        &self.cards
    }

    pub fn get_card(&self, name: Name) -> Option<&Vec<Card>> {
        self.cards.get(&name)
    }

    pub fn get_players(&self) -> &Vec<(Name, Retired)> {
        &self.players
    }

    pub fn get_questions(&self) -> &Vec<(Question, QuestionState)> {
        &self.questions
    }

    pub fn set_retire(&mut self, idx: usize) {
        if let Some((name, retired)) = self.players.get_mut(idx) {
            *retired = true;
        }
    }

    pub fn rotate_players(&mut self, mid: usize) {
        self.players.rotate_left(mid);
    }

    pub fn current_player(&self) -> Option<&Name> {
        self.players.first().map(|(name, _)| name)
    }

    pub fn select_question(&mut self, pos: usize) {
        if let Some((question, state)) = self.questions.get_mut(pos) {
            *state = QuestionState::Selected;
        };
    }

    pub fn use_question(&mut self) {
        for (_, state) in self.questions.iter_mut() {
            match state {
                QuestionState::Selected => *state = QuestionState::Used,
                _ => {}
            }
        }
    }

    pub fn get_answer(&self) -> &Vec<Card> {
        &self.answer
    }

    pub fn get_winner(&self) -> &Option<String> {
        &self.winner
    }

    pub fn set_winner(&mut self, name: Name) {
        self.winner = Some(name);
    }
}

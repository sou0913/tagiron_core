use crate::game::Question;
use crate::{constant::QUESTION_TEXTS};
use tagiron_card::{make_cards, Card};
use itertools::Itertools;

pub fn build_questions() -> Vec<Question> {
    QUESTION_TEXTS.map(|s| s.to_string()).to_vec()
}

pub fn build_cards() -> Vec<Card> {
    make_cards()
}

pub fn chunk_cards(cards: Vec<Card>, per: usize) -> Vec<Vec<Card>> {
    cards
        .into_iter()
        .chunks(per)
        .into_iter()
        .map(|c| c.into_iter().collect())
        .collect()
}

#[test]
fn test_chunk_cards() {
    let cards = make_cards();
    for chunk in chunk_cards(cards.clone(), 4) {
        assert_eq!(4, chunk.len());
    }
    // println!("{:?}", chunk_cards(cards, 4));
}

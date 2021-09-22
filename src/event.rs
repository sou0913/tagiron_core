use crate::view::{GameView, ResultView};
use tagiron_card::Card;
use serde_derive::*;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ClientEvent {
    Select {
        player: String,
        idx: usize,
    },
    Declare {
        player: String,
        cards: Vec<Card>
    },
}

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ServerEvent {
    Update {
        view: GameView,
    },
    GameResult {
        view: ResultView,
    }
}
use crate::view::{GameView, ResultView};
use tagiron_card::Card;
use serde_derive::*;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ClientEvent {
    Select {
        player_name: String,
        index: usize,
    },
    Declare {
        player_name: String,
        cards: Vec<Card>
    },
    Restart
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
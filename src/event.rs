use crate::view::{GameView, ResultView};
use tagiron_card::Card;
use serde_derive::*;

/*
expected json scheme:
{
    type: "Select",
    value: {
        player_name: "sasaki",
        index: 1,
    }
}
*/

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
    Restart {
        player_names: Vec<String>,
    }
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
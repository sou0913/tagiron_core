use crate::view::{GameView, ResultView};
use serde_derive::*;
use tagiron_card::Card;

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
        cards: Vec<Card>,
    },
    Restart {
        player_names: Vec<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ServerEvent {
    Update { view: GameView },
    GameResult { view: ResultView },
}

/*
expected json scheme:

Create
{
    type: "Create",
    value: {
        name: "sasaki"
    }
}

List
{
    type: "List",
    value: {
        names: ["sasaki", "sawada"]
    }
}
*/

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum UserEvent {
    Create { name: String },
    Remove { name: String },
    List { names: Vec<String> },
}

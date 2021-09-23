use crate::view::{GameView, ResultView};
use serde::{Deserialize, Serialize};
use serde_derive::*;
use tagiron_card::Card;

pub trait Event<'a>: Serialize + Deserialize<'a> {
    fn from_str(text: impl Into<String>) {
        serde_json::from_str(&text.into()).expect("from String error")
    }

    fn to_str(&self) -> String {
        serde_json::to_string(self).expect("to_string error")
    }
}

impl Event<'_> for ClientEvent {}

impl Event<'_> for UserEvent {}

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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum UserEvent {
    Create { name: String },
    Remove { name: String },
    List { names: Vec<String> },
}

use crate::{
    user::User,
    view::{GameView, ResultView},
};
use serde::{Deserialize, Serialize};
use serde_derive::*;
use tagiron_card::Card;

pub trait Event<'a>: Serialize + Deserialize<'a> {
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

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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

Remove
{
    type: "Remove",
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
    List,
}

struct List {
    names: Vec<String>,
}

impl List {
    pub fn from_users(users: Vec<User>) -> Self {
        Self {
            names: users
                .iter()
                .map(|user| user.get_name().to_owned())
                .collect(),
        }
    }
}

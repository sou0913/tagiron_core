use crate::game::Game;
use crate::event::{ClientEvent, ServerEvent};
use crate::command::{Command, GameCommand};

pub fn client_controller (game: &mut Game, event: ClientEvent) {
    match event {
        ClientEvent::Select { player_name, index } => {
            let command = GameCommand::SelectQuestion { index };
            command.execute(game);
        }
        ClientEvent::Declare { player_name, cards } => {
            let command = GameCommand::Declare { player_name, cards };
            command.execute(game);
        }
        ClientEvent::Restart { player_names } => {
            let command = GameCommand::Initialize { player_names };
            command.execute(game);
        }
    }
    // ServerEvent::Update()
}
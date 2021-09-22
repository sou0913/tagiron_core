use crate::game::Game;
use crate::event::{ClientEvent, ServerEvent};
use crate::command::{Command, GameCommand};

pub fn client_controller (game: &mut Game, event: ClientEvent) {
    match event {
        ClientEvent::Select { player, idx } => {
            let command = GameCommand::SelectQuestion { index: idx };
            command.execute(game);
            return
        }
        ClientEvent::Declare { player, cards } => {

        }
    }
}
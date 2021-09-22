mod game;
mod command;
mod view;
mod event;
mod controller;

use game::Game;
use tagiron_card::Card;
use std::{io, thread};
use std::io::prelude::*;
use anyhow::Result;
use serde_derive::{Serialize, Deserialize};
use crate::controller::client_controller;

fn main() -> Result<()> {
    let mut game = Game::default();
    loop {

        // サーバー側のイベントを表示する
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(3));
            println!("from server");
        });

        // 入力を待ち受ける
        for event in io::stdin().lock().lines() {
            let event = serde_json::from_str(&event.unwrap())?;
            client_controller(&mut game, event);
            println!("client");
        };
    }
}


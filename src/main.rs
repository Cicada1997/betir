type Error = Box<dyn std::error::Error>;

use {
    std::process::exit,

    macroquad::prelude::*,
    hecs::*,
};

pub mod components;
pub mod systems;

pub mod factory;
pub mod timer;

pub mod update;
pub mod event;

use crate::{
    update::update,

    factory::{
        default_keybinds,
    },

    systems::render::draw,
};

#[macroquad::main("gwarp")]
async fn main() {
    match run().await {
        Ok(()) => {},
        Err(e) => {
            eprintln!("An Error occured: {}", e);
            exit(1)
        },
    }
}

async fn run() -> Result<(), Error> {
    let mut ecs = World::new();
    let player = factory::spawn_player(&mut ecs, screen_width() / 2., screen_height() / 2.);    

    let keybinds = default_keybinds();

    loop {
        update(&mut ecs, player, &keybinds);
        draw(&ecs, player);
        next_frame().await;
    }
}

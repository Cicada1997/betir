type Error = Box<dyn std::error::Error>;

use crate::components::weapon;

use {
    std::process::exit,

    macroquad::prelude::*,
    hecs::*,
};

pub mod factory;
pub mod components;
pub mod systems;

use {
    components::transform::*,

    factory::{
        KeyBindings,
        default_keybinds,
    },

    systems::{
        movement, 
        render::{ draw_players, draw_missiles, draw_hud },
        action::{ on_action },
    },
};

#[derive(Clone, Copy)]
pub enum PlayerAction {
    Shoot(),
    Reload()
}

#[derive(Clone, Copy)]
pub enum GameEvent {
    Exit( Option<i32> ),

    ActionPreformed(PlayerAction),

    MovePlayer {
        x: f32,
        y: f32,
    }
}

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


fn handle_input(game_events: &mut Vec<GameEvent>, keybinds: &KeyBindings) {
    let keys = get_keys_down();

    for key in keys {
        if let Some(game_event) = keybinds.get(&key) {
            game_events.push(*game_event);
        }
    }
}

fn update(ecs: &mut World, player: Entity, keybinds: &KeyBindings) {
    let mut game_events = Vec::new();

    handle_input(&mut game_events, &keybinds);

    for event in game_events {
        match event {
            GameEvent::MovePlayer { x, y } => {
                if let Ok(transform) = ecs.query_one_mut::<&mut Transform>(player) {
                    transform.rot = (Vec2::from_angle(transform.rot) + vec2(x, y))
                        .clamp(Vec2::NEG_ONE, Vec2::ONE)
                        .normalize_or_zero()
                        .to_angle();

                    transform.speed = transform.max_speed;
                }
            },

            GameEvent::ActionPreformed(action) => {
                on_action(ecs, action, player);
            },

            GameEvent::Exit(None) => exit(0),

            GameEvent::Exit(Some(code)) => exit(code),
        }
    }

    let dt = get_frame_time();
    movement::update_player ( ecs, dt );
    movement::update_missile( ecs, dt );
    weapon::update_weapons  ( ecs     );
}

fn draw(ecs: &World, player: Entity) {
    clear_background(WHITE);

    draw_players(ecs);
    draw_missiles(ecs);

    draw_hud(ecs, player);
}

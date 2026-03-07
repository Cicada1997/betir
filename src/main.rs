type Error = Box<dyn std::error::Error>;

use std::time::{Duration, Instant};

use crate::components::weapon::Weapon;

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
        render::{ draw_players, draw_missiles },
    },
};

#[derive(Clone, Copy)]
pub enum GameEvent {
    Exit( Option<i32> ),

    Shoot,

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
        draw(&ecs);
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
                if let Ok(transform) = ecs.query_one_mut::<&mut Transform, >(player) {
                    transform.rot = (Vec2::from_angle(transform.rot) + vec2(x, y))
                        .clamp(Vec2::NEG_ONE, Vec2::ONE)
                        .normalize_or_zero()
                        .to_angle();

                    transform.speed = transform.max_speed;
                }
            },

            GameEvent::Shoot => {
                let (pos, rot) = {
                    let transform = ecs.get::<&Transform>(player)
                        .unwrap_or_else(|_| panic!("Unknown sender {} tried to send a missile.", player.id()));

                    (transform.pos, transform.rot)
                };

                if let Ok(weapon) = ecs.query_one_mut::<&mut Weapon>(player) {
                    if weapon.magazine <= 0 && !weapon.reloading {
                        weapon.reloading = true;
                        continue;
                    }

                    let last = Instant::now().duration_since(weapon.last_fired);

                    if weapon.reloading {
                        if last > Duration::from_secs(3) {
                            weapon.magazine = weapon.max_ammo;
                            weapon.reloading = false;
                        } else {
                            continue;
                        }
                    }

                    if last > weapon.cooldown {
                        weapon.last_fired = Instant::now();
                        weapon.magazine -= 1;
                    } else {
                        continue;
                    }
                }

                let _ = factory::spawn_missile(ecs, pos, rot, Some(player));
            },

            GameEvent::Exit(None) => exit(0),

            GameEvent::Exit(Some(code)) => exit(code),
        }
    }

    let dt = get_frame_time();
    movement::update_player(  ecs, dt );
    movement::update_missile( ecs, dt );
}

fn draw(ecs: &World) {
    clear_background(WHITE);

    draw_players(ecs);
    draw_missiles(ecs);
}

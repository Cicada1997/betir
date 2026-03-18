use {
    std::process::exit,

    macroquad::prelude::*,
    hecs::*,
};

use crate::{
    components::{
        transform::*,
        weapon,
    },
    systems::{
        movement, 
        action::{ on_action },
        enemy_spawner::spawn_enemies_natrual,
    },
    factory::{
        KeyBindings,
    },

    event::{
        GameEvent,
    },
};

fn handle_input(game_events: &mut Vec<GameEvent>, keybinds: &KeyBindings) {
    let keys = get_keys_down();

    for key in keys {
        if let Some(game_event) = keybinds.get(&key) {
            game_events.push(*game_event);
        }
    }
}


pub fn update(ecs: &mut World, player: Entity, keybinds: &KeyBindings) {
    let mut game_events = Vec::new();

    handle_input(&mut game_events, &keybinds);

    for event in game_events {
        match event {
            GameEvent::MovePlayer { x, y } => {
                if let Ok(transform) = ecs.query_one_mut::<&mut Transform>(player) {
                    transform.rot = (Vec2::from_angle(transform.rot) + vec2(x, y) * 0.1)
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

    spawn_enemies_natrual(ecs, player);

    movement::update_player  ( ecs, dt );
    movement::update_missile ( ecs, dt );
    weapon::update_guns      ( ecs     );
}

use std::{collections::HashMap, time::{Duration, Instant}};

use macroquad::prelude::*;
use hecs::{Entity, World};

use crate::{
    GameEvent,
    components::{
        transform::*,
        missile::*,
        weapon::*,
        tags::Player,
    },
};

pub fn spawn_player(ecs: &mut World, x: f32, y: f32) -> hecs::Entity {
    ecs.spawn((
        Transform {
            pos:        Vec2 { x, y, },
            last_pos:   Vec2 { x, y, },
            rot:        0.,

            speed:      0.,
            max_speed:  250.,
        },
        Weapon {
            cooldown:   Duration::from_millis(120),
            last_fired: Instant::now() - Duration::from_millis(120),

            magazine: 32,
            max_ammo: 32,
            reloading:  false,

            r#type: WeaponType::SMG,
        },
        Player,
    ))
}

pub fn spawn_missile(ecs: &mut World, spawnpoint: Vec2, rot: f32, sender: Option<Entity>) -> hecs::Entity {
    ecs.spawn((
        Missile {
            spawnpoint,
            sender,
            range:      500.,
        },
        Transform {
            pos:        spawnpoint,
            last_pos:   spawnpoint,
            rot:        rot,

            speed:      900.,
            max_speed:  900.,
         },
    ))

}

pub type KeyBindings = HashMap<KeyCode, GameEvent>;
pub fn default_keybinds() -> HashMap<KeyCode, GameEvent> {
    return HashMap::from([
        (KeyCode::W, GameEvent::MovePlayer { x: 0., y:-1. }),
        (KeyCode::A, GameEvent::MovePlayer { x:-1., y: 0. }),
        (KeyCode::S, GameEvent::MovePlayer { x: 0., y: 1. }),
        (KeyCode::D, GameEvent::MovePlayer { x: 1., y: 0. }),

        (KeyCode::Space, GameEvent::Shoot),

        (KeyCode::Escape, GameEvent::Exit(None)),
    ])
}

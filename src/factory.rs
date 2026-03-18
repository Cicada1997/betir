use {
    std::collections::HashMap,

    macroquad::prelude::*,
    hecs::{Entity, World},
};

use crate::{
    event::{
        GameEvent, 
        PlayerAction, 
    },

    components::{
        fly::*,
        missile::*,
        transform::*,
        weapon::*,
        tags::{ Enemy, Player },
    }, 

    timer::*,
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
        Gun {
            cooldown_timer: Timer::new(0.07, true),

            magazine: 32,
            max_ammo: 48,

            reload_timer:   Timer::new(0.56, false),
        },
        Player,
    ))
}

// TODO: implement flies
pub fn spawn_fly(ecs: &mut World, x: f32, y: f32) -> hecs::Entity {
    ecs.spawn((
        Transform {
            pos:        Vec2 { x, y, },
            last_pos:   Vec2 { x, y, },
            rot:        0.,

            speed:      0.,
            max_speed:  250.,
        },
        Enemy,
        Fly {
            speed:           0.,
            max_speed:       250.,
            detection_range: 800.,

            attack_speed:    2.,
            attack_range:    100.,
            attack_damage:   14.,
        }
    ))
}

pub fn spawn_missile(ecs: &mut World, spawnpoint: Vec2, rot: f32, sender: Option<Entity>) -> hecs::Entity {
    ecs.spawn((
        Missile {
            spawnpoint,
            sender,
            range:      900.,
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

        (KeyCode::R,     GameEvent::ActionPreformed(PlayerAction::Reload())),
        (KeyCode::Space, GameEvent::ActionPreformed(PlayerAction::Shoot())),

        (KeyCode::Escape, GameEvent::Exit(None)),
    ])
}

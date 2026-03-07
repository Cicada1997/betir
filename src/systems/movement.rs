use {
    macroquad::prelude::Vec2,
    hecs::*,
};

use crate::components::{
    missile::*, tags::Player, transform::*,
};

pub fn update_player(ecs: &mut World, dt: f32) {

    for transform in ecs.query_mut::<&mut Transform>().with::<&Player>() {
        transform.last_pos = transform.pos;
        transform.pos += Vec2::from_angle(transform.rot) * transform.speed * dt;

        transform.speed *= 0.95 * dt; // friction/retardation
        if transform.speed < 0. { transform.speed = 0.; }
    }
}

pub fn update_missile(ecs: &mut World, dt: f32) {
    let mut death_note = Vec::new();

    for (eid, transform, missile) in ecs.query_mut::<(Entity, &mut Transform, &Missile)>() {
        if transform.pos.distance(missile.spawnpoint) > missile.range {
            death_note.push(eid);
            continue;
        }

        transform.last_pos = transform.pos;
        transform.pos += Vec2::from_angle(transform.rot) * transform.speed * dt;
    }

    for entity in &death_note {
        ecs.despawn(*entity).unwrap();
    }
} 

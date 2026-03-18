use {
    macroquad::{
        prelude::*,
        rand::RandomRange,
    },

    hecs::*,
};

use crate::{
    factory::spawn_fly,
    components::transform::*,
};

static SPAWN_FLY_PROBABLITITY: f32 = 0.1;
static MAX_ENIMIE_SPAWN_DISTANCE: f32 = 1400.;
static MIN_ENIMIE_SPAWN_DISTANCE: f32 = 450.;

pub fn spawn_enemies_natrual(ecs: &mut World, player: Entity) {
    let player_pos = {
        let p = ecs.get::<&Transform>(player).unwrap().pos;
        p
    };

    if SPAWN_FLY_PROBABLITITY > RandomRange::gen_range(0., 100.) {
        let angle = Vec2::from_angle(RandomRange::gen_range(0., 360.));
        let distance = RandomRange::gen_range(MIN_ENIMIE_SPAWN_DISTANCE, MAX_ENIMIE_SPAWN_DISTANCE);

        let pos = (angle * distance) + player_pos;

        spawn_fly(ecs, pos.x, pos.y);
    }
}

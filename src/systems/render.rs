use {
    macroquad::prelude::*,
    hecs::World,
};

use crate::{
    components::{
        transform::*,
        missile::*,
        tags::Player,
    },
};

pub fn draw_players(ecs: &World) {
    let pwidth = 20.;

    for transform in ecs.query::<&Transform>().with::<&Player>().iter() {
        let dir_vec = Vec2::from_angle(transform.rot);

        let triangle = [
            transform.pos + dir_vec        * pwidth,
            transform.pos + dir_vec.perp() * pwidth,
            transform.pos - dir_vec.perp() * pwidth,
        ];

        draw_triangle(
            triangle[0],
            triangle[1],
            triangle[2],
            RED,
        );
    }
}

pub fn draw_missiles(ecs: &World) {
    for (transform, missile) in ecs.query::<(&Transform, &Missile)>().iter() {
        let r = transform.pos.distance(missile.spawnpoint) / (2. * missile.range);

        draw_circle(transform.pos.x, transform.pos.y, 10., Color { r: 1.-r, g: 40. / 255., b: 20. / 255., a: 1. });
    }
}

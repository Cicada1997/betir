use std::time::Instant;

use {
    macroquad::prelude::*,
    hecs::*,
};

use crate::{
    components::{
        transform::*,
        missile::*,
        weapon::*,
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

pub fn draw_hud(ecs: &World, player: Entity) {
    //                //
    //  Magazine Bar  //
    //                //
    let weapon = ecs.get::<&Weapon>(player).unwrap();
    let sq = {
        let w = screen_width() / 2.;
        let x = (screen_width() - w) / 2.;
        let border = 10.;
        let sq = Rect { y: screen_height() - 80., w, x, h: 30., };

        draw_rectangle(sq.x, sq.y, sq.w, sq.h, BLACK);
        draw_rectangle(sq.x + border, sq.y + border, sq.w - 2. * border, sq.h - 2. * border, GRAY);
        draw_rectangle(sq.x + border, sq.y + border, (sq.w - 2. * border) * (weapon.magazine as f32 / weapon.max_ammo as f32) as f32, sq.h - 2. * border, RED);

        sq
    };

    //                 //
    //  Relaod Circle  //
    //                 //
    let radius = 20.;
    let padding = 5.;
    let ammo_x = sq.x - (radius + 2. * padding);

    if weapon.reloading {
        let reload_indicator_value = 360. * Instant::now().duration_since(weapon.last_fired).as_secs_f32() / weapon.reload_time.as_secs_f32();
        draw_arc(ammo_x, sq.y + (sq.h / 2.), 255, radius, 0., 5., reload_indicator_value, BLACK);
    };

    //                   //
    //  Magazine number  //
    //                   //
    let t_col = match weapon.magazine {
        0..8 => RED,
        8..20 => YELLOW,
        20..30 => GREEN,
        30..=u32::MAX => BLACK,
        
    };
    let font_size = 30.;
    draw_text(&weapon.magazine.to_string(), ammo_x, sq.y + (sq.h - font_size) / 2., font_size, t_col);
}

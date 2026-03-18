use {
    macroquad::prelude::*,
    hecs::*,
};

use crate::components::{
    fly::Fly, missile::*, tags::Player, transform::*, weapon::Gun
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

pub fn draw_flies(ecs: &World) {
    for (transform, _fly) in ecs.query::<(&Transform, &Fly)>().iter() {
        let size = 5.;
        let thickness = 4.;

        draw_line(transform.pos.x + size, transform.pos.y + size, transform.pos.x - size, transform.pos.y - size, thickness, BLACK);
        draw_line(transform.pos.x + size, transform.pos.y - size, transform.pos.x - size, transform.pos.y + size, thickness, BLACK);
    }
}

pub fn draw_hud(ecs: &World, player: Entity) {
    //                //
    //  Magazine Bar  //
    //                //
    let gun = ecs.get::<&Gun>(player).unwrap();

    let sq = {
        let w = screen_width() / 2.;
        let x = (screen_width() - w) / 2.;
        let border = 10.;
        let sq = Rect { y: screen_height() - 80., w, x, h: 30., };

        draw_rectangle(sq.x, sq.y, sq.w, sq.h, BLACK);
        draw_rectangle(sq.x + border, sq.y + border, sq.w - 2. * border, sq.h - 2. * border, GRAY);
        draw_rectangle(sq.x + border, sq.y + border, (sq.w - 2. * border) * (gun.magazine as f32 / gun.max_ammo as f32) as f32, sq.h - 2. * border, RED);

        sq
    };

    //                 //
    //  Relaod Circle  //
    //                 //
    let radius = 20.;
    let padding = 5.;
    let ammo_x = sq.x - (radius + 2. * padding);

    if !gun.reload_timer.is_finished() {
        draw_arc(ammo_x, sq.y + (sq.h / 2.), 255, radius, 0., 5., 360. * gun.reload_timer.percentage_elapsed() as f32, BLACK);
    };

    //                   //
    //  Magazine number  //
    //                   //
    let t_col = match gun.magazine {
        0..8 => RED,
        8..20 => YELLOW,
        20..30 => GREEN,
        30..=u32::MAX => BLACK,
        
    };

    let font_size = 30.;
    draw_text(&gun.magazine.to_string(), ammo_x, sq.y + (sq.h - font_size) / 2., font_size, t_col);

    draw_text(&format!("FPS: {}", get_fps()), 40., 60., 40., GREEN);

    let alive_flies    = ecs.query::<&Fly>().iter().count();
    let alive_enitites = ecs.iter().count();

    print!("\rAlive Flies: {alive_flies} Alive Entities: {alive_enitites}");
}

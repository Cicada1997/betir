use macroquad::math::Vec2;

use {
    hecs::*,
};

use crate::{
    event::PlayerAction,

    components::{
        transform::*,
        weapon::Gun,
    },

    factory,
};

pub fn on_action(ecs: &mut World, action: PlayerAction, player: Entity) {
    match action {
        PlayerAction::Reload() => {
            if let Ok(gun) = ecs.query_one_mut::<&mut Gun>(player) {
                if gun.magazine == gun.max_ammo { return; }

                gun.reload_timer.reset();
                gun.reload_timer.start();
                // gun.last_fired = Instant::now();
                // gun.reloading = true;
            }
        },

        PlayerAction::Shoot() => {
            if let Ok(gun) = ecs.query_one_mut::<&mut Gun>(player) {

                if !gun.cooldown_timer.is_finished() 
                   || gun.reload_timer.enabled
                   || gun.magazine < 1
                    { return; }

                gun.cooldown_timer.reset();
                gun.magazine -= 1;
            }

            let (pos, rot) = {
                let transform = ecs.get::<&Transform>(player)
                    .unwrap_or_else(|_| panic!("Unknown sender {} tried to send a missile.", player.id() ));

                (transform.pos + Vec2::from_angle(transform.rot), transform.rot)
            };

            let _ = factory::spawn_missile(ecs, pos, rot, Some(player));
        }
    }
}

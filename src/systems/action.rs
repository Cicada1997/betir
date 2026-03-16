use macroquad::math::Vec2;

use {
    hecs::*,
    std::time::Instant,
};

use crate::{
    PlayerAction,

    components::{
        transform::*,
        weapon::*,
    },

    factory,
};

pub fn on_action(ecs: &mut World, action: PlayerAction, player: Entity) {
    match action {
        PlayerAction::Reload() => {
            if let Ok(weapon) = ecs.query_one_mut::<&mut Weapon>(player) {
                if weapon.magazine == weapon.max_ammo { return; }

                weapon.last_fired = Instant::now();
                weapon.reloading = true;
            }
        },

        PlayerAction::Shoot() => {
            if let Ok(weapon) = ecs.query_one_mut::<&mut Weapon>(player) {

                if weapon.reloading 
                    || weapon.magazine < 1
                    || weapon.cooldown > Instant::now().duration_since(weapon.last_fired)
                    { return; }

                weapon.last_fired = Instant::now();
                weapon.magazine -= 1;
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

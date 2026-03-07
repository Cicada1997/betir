use {
    hecs::*,
    std::time::{ Duration, Instant },
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
        PlayerAction::Shoot() => {
            let (pos, rot) = {
                let transform = ecs.get::<&Transform>(player)
                    .unwrap_or_else(|_| panic!("Unknown sender {} tried to send a missile.", player.id()));

                (transform.pos, transform.rot)
            };

            if let Ok(weapon) = ecs.query_one_mut::<&mut Weapon>(player) {
                if weapon.magazine <= 0 && !weapon.reloading {
                    weapon.reloading = true;
                    return;
                }

                let last = Instant::now().duration_since(weapon.last_fired);

                if weapon.reloading {
                    if last > Duration::from_secs(3) {
                        weapon.magazine = weapon.max_ammo;
                        weapon.reloading = false;
                    } else {
                        return;
                    }
                }

                if last > weapon.cooldown {
                    weapon.last_fired = Instant::now();
                    weapon.magazine -= 1;
                } else {
                    return;
                }
            }

            let _ = factory::spawn_missile(ecs, pos, rot, Some(player));
        }
    }
}

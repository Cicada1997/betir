use hecs::*;

use crate::timer::Timer;

// pub enum GunType {
//     SMG,
// }
//
pub struct Gun {
    pub cooldown_timer: Timer,

    pub magazine:       u32,
    pub max_ammo:       u32,

    pub reload_timer:   Timer,

    // pub r#type:         GunType,
}

pub fn update_guns(ecs: &mut World) {
    for gun in ecs.query_mut::<&mut Gun>() {
        if gun.reload_timer.is_finished() && gun.reload_timer.enabled {
            gun.magazine = gun.max_ammo;
            gun.reload_timer.stop();
        }
    }
}

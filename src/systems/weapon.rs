use {
    hecs::*,
};

use crate::{
    components::weapon::Gun,
};

pub fn update_guns(ecs: &mut World) {
    for gun in ecs.query_mut::<&mut Gun>() {
        if gun.reload_timer.is_finished() && gun.reload_timer.enabled {
            gun.magazine = gun.max_ammo;
            gun.reload_timer.stop();
        }
    }
}

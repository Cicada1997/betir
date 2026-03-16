use hecs::*;
use std::time::{Duration, Instant};

pub enum WeaponType {
    SMG,
}

pub struct Weapon {
    pub cooldown:    Duration,
    pub last_fired:  Instant,

    pub magazine:    u32,
    pub max_ammo:    u32,

    pub reloading:   bool,
    pub reload_time: Duration,

    pub r#type:      WeaponType,
}

pub fn update_weapons(ecs: &mut World) {
    for weapon in ecs.query_mut::<&mut Weapon>() {
        if weapon.reloading && weapon.reload_time < Instant::now().duration_since(weapon.last_fired) {
            weapon.reloading = false;
            weapon.magazine = weapon.max_ammo;
        }
    }
}

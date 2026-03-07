use std::time::{Duration, Instant};

pub enum WeaponType {
    SMG,
}

pub struct Weapon {
    pub cooldown:   Duration,
    pub last_fired: Instant,

    pub magazine:   u32,
    pub max_ammo:   u32,
    pub reloading:  bool,

    pub r#type:     WeaponType,
}

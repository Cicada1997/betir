use hecs::*;

use crate::timer::Timer;

pub struct Gun {
    pub cooldown_timer: Timer,

    pub magazine:       u32,
    pub max_ammo:       u32,

    pub reload_timer:   Timer,
}

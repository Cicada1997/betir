use {
    macroquad::prelude::*,
    hecs::*,
};

pub struct Missile {
    pub spawnpoint: Vec2,
    pub sender:     Option<Entity>,
    pub range:      f32,
}

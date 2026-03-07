use macroquad::prelude::Vec2;

#[derive(Debug, Clone)]
pub struct Transform {
    pub pos:      Vec2,
    pub last_pos: Vec2,
    pub rot:      f32,

    pub speed:    f32,
    pub max_speed: f32,
}

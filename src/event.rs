#[derive(Clone, Copy)]
pub enum PlayerAction {
    Shoot(),
    Reload()
}

#[derive(Clone, Copy)]
pub enum GameEvent {
    Exit( Option<i32> ),

    ActionPreformed(PlayerAction),

    MovePlayer {
        x: f32,
        y: f32,
    }
}

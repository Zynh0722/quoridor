#[derive(Debug)]
pub struct Player {
    pub pos: PlayerPosition,
    pub walls_remaining: u8,
}

impl Player {
    pub fn new(pos: PlayerPosition, total_players: u8) -> Self {
        Self {
            pos,
            walls_remaining: 20 / total_players,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPosition {
    pub x: usize,
    pub y: usize,
}

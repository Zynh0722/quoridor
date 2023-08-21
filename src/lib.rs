#[derive(Debug)]
pub struct Board {
    pub nodes: [[Option<WallDirection>; 8]; 8],
    pub players: Vec<Player>,
}

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

#[derive(Debug, Clone, Copy)]
pub struct PlayerPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum WallDirection {
    Vertical,
    Horizontal,
}

static DEFAULT_POSITIONS: [PlayerPosition; 4] = [
    PlayerPosition { x: 4, y: 0 },
    PlayerPosition { x: 4, y: 8 },
    PlayerPosition { x: 0, y: 4 },
    PlayerPosition { x: 8, y: 4 },
];

impl WallDirection {
    #[inline]
    pub fn is_vertical(&self) -> bool {
        match self {
            WallDirection::Vertical => true,
            WallDirection::Horizontal => false,
        }
    }

    #[inline]
    pub fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }
}

impl Board {
    pub fn new(num_players: u8) -> Self {
        assert!(num_players <= 4);
        assert!(num_players >= 2);

        let mut players = Vec::new();
        for i in 0..num_players {
            players.push(Player::new(DEFAULT_POSITIONS[i as usize], num_players))
        }
        Self {
            nodes: [[None; 8]; 8],
            players,
        }
    }

    pub fn place_wall(&mut self, x: usize, y: usize, direction: WallDirection) {
        if x < 8 && y < 8 {
            self.nodes[y][x] = Some(direction);
        }
    }
}

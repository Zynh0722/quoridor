use std::borrow::BorrowMut;

use player::PlayerPosition;

#[derive(Debug)]
pub struct Board {
    pub nodes: [[Option<WallDirection>; 8]; 8],
    pub players: Vec<player::Player>,
}

pub mod player;

#[derive(Debug, Clone, Copy)]
pub enum WallDirection {
    Vertical,
    Horizontal,
}

static DEFAULT_POSITIONS: [player::PlayerPosition; 4] = [
    player::PlayerPosition { x: 4, y: 0 },
    player::PlayerPosition { x: 4, y: 8 },
    player::PlayerPosition { x: 0, y: 4 },
    player::PlayerPosition { x: 8, y: 4 },
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
            players.push(player::Player::new(
                DEFAULT_POSITIONS[i as usize].clone(),
                num_players,
            ))
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

    fn simple_move(pos: &PlayerPosition, dir: &MoveDirection) -> PlayerPosition {
        let mut pos = pos.clone();

        match dir {
            MoveDirection::North => pos.y -= 1,
            MoveDirection::East => pos.x += 1,
            MoveDirection::South => pos.y += 1,
            MoveDirection::West => pos.x = pos.x - 1,
            MoveDirection::NorthEast => {
                pos.y -= 1;
                pos.x += 1;
            }
            MoveDirection::NorthWest => {
                pos.y -= 1;
                pos.x -= 1;
            }
            MoveDirection::SouthEast => {
                pos.y += 1;
                pos.x += 1;
            }
            MoveDirection::SouthWest => {
                pos.y += 1;
                pos.x -= 1;
            }
        }

        pos
    }

    pub fn move_player(&mut self, id: usize, dir: &MoveDirection) -> Result<(), MoveError> {
        if self.players.len() <= id || !self.is_valid_move(id, dir) {
            return Err(MoveError);
        }

        let pos = self.players[id].pos.borrow_mut();

        *pos = Self::simple_move(&pos, &dir);

        Ok(())
    }

    pub fn are_bisected(&self, a: &PlayerPosition, b: &PlayerPosition) -> bool {
        if !Self::are_adjacent(&a, &b) {
            return true;
        }

        let min_x = a.x.min(b.x);
        let min_y = a.y.min(b.y);

        let d_x = a.x.max(b.x) - min_x;
        let d_y = a.y.max(b.y) - min_y;

        // println!("a.x\t{}\na.y\t{}\nb.x\t{}\nb.y\t{}", a.x, a.y, b.x, b.y);
        //
        // println!(
        //     "min_x\t{}\nmin_y\t{}\nd_x\t{}\nd_y\t{}",
        //     min_x, min_y, d_x, d_y
        // );

        if d_x > 0 {
            // TODO: Wrapping sub????
            let bisecting_walls = [
                self.nodes.get(min_y).map(|r| r[min_x]),
                self.nodes
                    .get((min_y as isize - 1).max(0) as usize)
                    .map(|r| r[min_x]),
            ]
            .iter()
            .flatten()
            .flatten()
            .filter(|node| node.is_vertical())
            .count();

            if bisecting_walls > 0 {
                return true;
            }
        }

        if d_y > 0 {
            let bisecting_walls = [
                self.nodes[min_y].get(min_x),
                self.nodes[min_y].get((min_x as isize - 1).max(0) as usize),
            ]
            .iter()
            .flat_map(|n| *n)
            .flatten()
            .filter(|n| n.is_horizontal())
            .count();

            if bisecting_walls > 0 {
                return true;
            }
        }

        false
    }

    pub fn are_adjacent(a: &PlayerPosition, b: &PlayerPosition) -> bool {
        let d_x = a.x.max(b.x) - a.x.min(b.x);
        let d_y = a.y.max(b.y) - a.y.min(b.y);

        d_x <= 1 && d_y <= 1 && !(d_x == 0 && d_y == 0)
    }

    pub fn is_valid_move(&self, id: usize, dir: &MoveDirection) -> bool {
        let pos = &self.players[id].pos;
        let tar_pos = Self::simple_move(&pos, dir);

        match dir {
            MoveDirection::North
            | MoveDirection::East
            | MoveDirection::South
            | MoveDirection::West => !self.are_bisected(&pos, &tar_pos),
            MoveDirection::NorthEast
            | MoveDirection::NorthWest
            | MoveDirection::SouthEast
            | MoveDirection::SouthWest => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveError;

#[derive(Debug, Clone)]
pub enum MoveDirection {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

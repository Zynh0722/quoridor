#[derive(Debug)]
pub struct Board {
    pub nodes: [[Option<WallDirection>; 8]; 8],
}

#[derive(Debug, Clone, Copy)]
pub enum WallDirection {
    Vertical,
    Horizontal,
}

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
    pub fn new() -> Self {
        Self {
            nodes: [[None; 8]; 8],
        }
    }

    pub fn place_wall(&mut self, x: usize, y: usize, direction: WallDirection) {
        if x < 8 && y < 8 {
            self.nodes[y][x] = Some(direction);
        }
    }
}

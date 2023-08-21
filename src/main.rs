use quoridor::player::PlayerPosition;

trait Printable {
    fn print(&self);
}

impl Printable for quoridor::Board {
    fn print(&self) {
        let mut board_strings: Vec<String> = Vec::new();
        board_strings.push(format!("+{:-<35}+", ""));
        for row in self.nodes.iter() {
            board_strings.push(format!("|{: <35}|", ""));
            let mut row_string = String::new();
            row_string.push_str(&format!("|"));
            for _ in row.iter() {
                row_string.push_str(&format!("   +"));
            }
            row_string.push_str(&format!("   |"));

            board_strings.push(row_string)
        }
        board_strings.push(format!("|{: <35}|", ""));
        board_strings.push(format!("+{:-<35}+", ""));

        for (y, row) in self.nodes.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if let Some(direction) = node {
                    use quoridor::WallDirection::{Horizontal, Vertical};
                    let x = 4 * x + 4;
                    let y = 2 * y + 2;
                    match direction {
                        Vertical => {
                            board_strings[y + 1].replace_range(x..x + 1, "|");
                            board_strings[y].replace_range(x..x + 1, "|");
                            board_strings[y - 1].replace_range(x..x + 1, "|")
                        }
                        Horizontal => board_strings[y].replace_range(x - 3..x + 4, "-------"),
                    }
                }
            }
        }

        for (id, player) in self.players.iter().enumerate() {
            let x = 4 * player.pos.x + 2;
            let y = 2 * player.pos.y + 1;

            board_strings[y].replace_range(x..x + 1, &id.to_string())
        }

        board_strings.iter().for_each(|s| println!("{}", s));
    }
}

fn main() {
    let mut board = quoridor::Board::new(2);
    board.place_wall(0, 0, quoridor::WallDirection::Vertical);
    board.place_wall(2, 3, quoridor::WallDirection::Horizontal);
    board
        .move_player(0, &quoridor::MoveDirection::West)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::South)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::South)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::South)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::East)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::South)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::West)
        .unwrap();
    board
        .move_player(0, &quoridor::MoveDirection::East)
        .unwrap();
    let uwu = board.are_bisected(
        &PlayerPosition { x: 3, y: 3 },
        &PlayerPosition { x: 3, y: 4 },
    );
    println!("uwu? {}", uwu);
    board.print()
}

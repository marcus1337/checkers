
pub mod tile;

use std::fmt;
use tile::Point;
use tile::Tile;
use tile::Brick;
use tile::BrickType;
use tile::Player;
use tile::Direction;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    OneWin,
    TwoWin,
    Draw,
    OnGoing,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Board{
    tiles: [[Tile; 8]; 8],
}

impl Board{

    pub fn new() -> Self {
        let mut tiles = [[Tile::Empty; 8]; 8];
        for col in 0..4 {
            for row in 0..3 {
                let mut piece_col_one = col*2;
                let mut piece_col_two = piece_col_one;
                if row % 2 == 0 {
                    piece_col_one += 1;
                }else{
                    piece_col_two += 1;
                }
                tiles[piece_col_one][row] = Tile::Brick(Brick::PlayerBrick(Player::One, BrickType::Pawn));
                tiles[piece_col_two][7-row] = Tile::Brick(Brick::PlayerBrick(Player::One, BrickType::Pawn));
            }
        }

        Self {
            tiles: tiles
        }
    }

    pub fn get_tile(&self, point: Point) -> Tile {
        self.tiles[point.col as usize][point.row as usize]
    }

    pub fn has_brick(&self, point: Point) -> bool {
        self.get_tile(point) != Tile::Empty
    }

    pub fn get_brick(&self, point: Point) -> Brick {
        match self.get_tile(point) {
            Tile::Brick(brick) => brick,
            Tile::Empty => panic!("No brick at {:?}", point),
        }
    }

    pub fn remove_brick(&mut self, point: Point) {
        self.tiles[point.col as usize][point.row as usize] = Tile::Empty;
    }
    pub fn place_brick(&mut self, point: Point, brick: Brick) {
        self.tiles[point.col as usize][point.row as usize] = Tile::Brick(brick);
    }

}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = "".to_owned();
        for row in 0..8 {
            for col in 0..8 {
                board_string += self.tiles[col][7 - row].to_string().as_str();
            }
            board_string += "\n";
        }
        write!(f, "{}", board_string)
    }
}


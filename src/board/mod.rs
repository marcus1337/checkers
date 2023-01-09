
pub mod tile;

use std::fmt;
use tile::Point;
use tile::Tile;
use tile::Brick;
use tile::BrickType;
use tile::Player;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    OneWin,
    TwoWin,
    Draw,
    OnGoing,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board{
    tiles: [[Tile; 8]; 8],
}

impl Board{

    pub fn new() -> Self {
        let mut tiles = [[Tile::Empty; 8]; 8];

        for point in Board::get_tile_points() {
            let col = point.col as usize;
            let row = point.row as usize;
            if point.row < 3 {
                tiles[col][row] = Tile::Brick(Brick::PlayerBrick(Player::One, BrickType::Pawn));
            }
            if point.row > 4 {
                tiles[col][row] = Tile::Brick(Brick::PlayerBrick(Player::Two, BrickType::Pawn));
            }
        }

        Self {
            tiles: tiles
        }
    }

    pub fn get_tile_points() -> [Point; 32] {
        let mut points = [Point::new(0,0); 32];
        let mut index = 0;
        for col in (0..8) {
            for row in 0..8 {
                if (col + row) % 2 == 0 {
                    points[index] = Point::new(col, row);
                    index += 1;
                }
            }
        }
        return points;
    }

    pub fn get_tile(&self, point: Point) -> Tile {
        self.tiles[point.col as usize][point.row as usize]
    }

    pub fn has_brick(&self, point: Point) -> bool {
        self.get_tile(point) != Tile::Empty
    }

    pub fn has_player_brick(&self, point: Point, player: Player) -> bool {
        if !self.has_brick(point) {
            return false;
        }
        match self.get_brick(point) {
            Brick::PlayerBrick(p, _) => p == player,
        }
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



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

    pub fn get_brick(&self, point: Point) -> Brick {
        match self.get_tile(point) {
            Tile::Brick(brick) => brick,
            Empty => panic!("No brick at {:?}", point),
        }
    }

    fn remove_brick(&mut self, point: Point) {
        self.tiles[point.col as usize][point.row as usize] = Tile::Empty;
    }
    fn place_brick(&mut self, point: Point, brick: Brick) {
        self.tiles[point.col as usize][point.row as usize] = Tile::Brick(brick);
    }

    fn is_take_move(from: Point, to: Point) -> bool {
        (to.row - from.row).abs() == 2
    }

    pub fn can_jump_brick(&self, from: Point, direction: Direction) -> bool{
        let from_brick = self.get_brick(from);
        if !from_brick.can_step_in_direction(direction){
            return false;
        }

        let mut to = from;
        to.step(direction);
        let mid_point = to;
        to.step(direction);

        let mid_point_has_brick = mid_point.in_bounds() && self.get_tile(mid_point) != Tile::Empty;
        let to_point_is_empty = to.in_bounds() && self.get_tile(to) == Tile::Empty;
        if !mid_point_has_brick || !to_point_is_empty{
            return false
        } 
        let mid_brick = self.get_brick(mid_point);        
        !mid_brick.is_same_player(from_brick)
    }

    pub fn can_move_or_jump_brick(&self, from: Point, direction: Direction) -> bool {
        self.can_jump_brick(from, direction) || self.can_move_brick(from, direction)
    }

    pub fn can_move_brick(&self, from: Point, direction: Direction) -> bool{
        if !self.get_brick(from).can_step_in_direction(direction){
            return false;
        }
        let mut to = from;
        to.step(direction);
        to.in_bounds() && self.get_tile(to) == Tile::Empty
    }

    pub fn move_brick(&mut self, from: Point, to: Point) {
        let brick = self.get_brick(from);
        self.remove_brick(from);
        self.place_brick(to, brick);
        if Board::is_take_move(from, to){
            let mut mid_point = from;
            mid_point.step_towards(to);
            self.remove_brick(mid_point);
        }
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


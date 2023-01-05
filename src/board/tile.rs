
use std::ops::{Add,Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BrickType{
    Pawn,
    King,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player{
    One,
    Two,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Brick{
    PlayerBrick(Player, BrickType)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile{
    Empty,
    Brick(Brick),
}

impl Tile {
    pub fn to_string(&self) -> String {
        let str = match self {
            Tile::Empty => "[ ]",
            Tile::Brick(Brick::PlayerBrick(Player::One, BrickType::Pawn)) => "[X]",
            Tile::Brick(Brick::PlayerBrick(Player::One, BrickType::King)) => "[x]",
            Tile::Brick(Brick::PlayerBrick(Player::Two, BrickType::Pawn)) => "[O]",
            Tile::Brick(Brick::PlayerBrick(Player::Two, BrickType::King)) => "[o]",
        };
        String::from(str)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point{
    pub col: i32,
    pub row: i32,
}

impl Point{

    pub fn new(col: i32, row: i32) -> Self {
        Self{
            col:col, row:row
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.col >= 0 && self.col < 8 && self.row >= 0 && self.row < 8
    }

    pub fn step_towards(&mut self, other: Point){
        if self.col < other.col {
            self.col += 1;
        } else if self.col > other.col {
            self.col -= 1;
        }
        if self.row > other.row {
            self.row -= 1;
        } else if self.row < other.row {
            self.row += 1;
        }
    }

}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.col + other.col, self.row + other.row)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.col - other.col, self.row - other.row)
    }
}


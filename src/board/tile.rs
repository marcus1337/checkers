
use std::ops::{Add, AddAssign, Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    pub fn all () -> [Direction; 4] {
        [Direction::NorthWest, Direction::NorthEast, Direction::SouthWest, Direction::SouthEast]
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BrickType {
    Pawn,
    King,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    One,
    Two,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Brick {
    PlayerBrick(Player, BrickType),
}

impl Brick {
    pub fn is_same_player(&self, other: Brick) -> bool {
        match (self, other) {
            (Brick::PlayerBrick(p1, _), Brick::PlayerBrick(p2, _)) => p1 == &p2
        }
    }

    pub fn has_player(&self, player: Player) -> bool {
        match self {
            Brick::PlayerBrick(self_player, _) => self_player == &player,
        }
    }

    pub fn can_step_in_direction(&self, direction: Direction) -> bool {
        let has_king = match self {
            Brick::PlayerBrick(_, brick_type) => brick_type == &BrickType::King
        };
        
        if has_king {
            return true;
        }

        match direction {
            Direction::NorthEast => self.has_player(Player::One),
            Direction::NorthWest => self.has_player(Player::One),
            Direction::SouthEast => self.has_player(Player::Two),
            Direction::SouthWest => self.has_player(Player::Two),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
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
pub struct Point {
    pub col: i32,
    pub row: i32,
}

impl Point {
    pub fn new(col: i32, row: i32) -> Self {
        Self { col: col, row: row }
    }

    pub fn jump(&mut self, direction: Direction) {
        self.step(direction);
        self.step(direction);
    }

    pub fn step(&mut self, direction: Direction) {
        let direction_point = match direction {
            Direction::NorthEast => Point::new(1, 1),
            Direction::NorthWest => Point::new(-1, 1),
            Direction::SouthEast => Point::new(1, -1),
            Direction::SouthWest => Point::new(-1, -1),
        };
        *self += direction_point;
    }

    pub fn in_bounds(&self) -> bool {
        self.col >= 0 && self.col < 8 && self.row >= 0 && self.row < 8
    }

    pub fn step_towards(&mut self, other: Point) {
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

    pub fn get_direction(from: Point, to: Point) -> Direction {
        let col_diff = to.col - from.col;
        let row_diff = to.row - from.row;
        match (row_diff.signum(), col_diff.signum()) {
            (-1, -1) => Direction::SouthWest,
            (-1, 1) => Direction::SouthEast,
            (1, -1) => Direction::NorthWest,
            (1, 1) => Direction::NorthEast,
            _ => panic!("Invalid move: from = ({}, {}), to = ({}, {})", from.row, from.col, to.row, to.col),
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

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.col += other.col;
        self.row += other.row;
    }
}

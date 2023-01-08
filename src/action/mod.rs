
use super::board::tile::Direction;
use super::board::tile::Point;
use super::board::Board;
use super::board::tile::Tile;
use super::board::tile::Brick;
use super::board::tile::BrickType;
use super::board::tile::Player;

pub struct Action{
    pub from: Point,
    pub to: Point,
}


impl Action{
    pub fn new(from: Point, to: Point) -> Self {
        Self{
            from:from,
            to:to
        }
    }

    pub fn is_step(&self) -> bool {
        (self.to.row - self.from.row).abs() == 1
    }
    pub fn is_jump(&self) -> bool {
        !self.is_step()
    } 

    pub fn get_direction(&self) -> Direction {
        let col_diff = self.to.col - self.from.col;
        let row_diff = self.to.row - self.from.row;
        match (row_diff.signum(), col_diff.signum()) {
            (-1, -1) => Direction::SouthWest,
            (-1, 1) => Direction::SouthEast,
            (1, -1) => Direction::NorthWest,
            (1, 1) => Direction::NorthEast,
            _ => panic!("Invalid move: from = ({}, {}), to = ({}, {})", self.from.row, self.from.col, self.to.row, self.to.col),
        }
    }

    fn can_step(board: &Board, from: Point, to: Point) -> bool {
        if !to.in_bounds() {
            return false;
        }
        let from_brick = board.get_brick(from);
        let to_tile = board.get_tile(to);
        let direction = Point::get_direction(from, to);
        to_tile == Tile::Empty && from_brick.can_step_in_direction(direction)
    }

    fn can_jump(board: &Board, from: Point, to: Point) -> bool {
        if !to.in_bounds() {
            return false;
        }
        
        let direction = Point::get_direction(from, to);
        let mut mid_point = from;
        mid_point.step(direction);
        if !board.has_brick(mid_point) {
            return false;
        }

        let captured_brick = board.get_brick(mid_point);
        let from_brick = board.get_brick(from);
        let to_tile = board.get_tile(to);

        to_tile == Tile::Empty && from_brick.can_step_in_direction(direction) && !captured_brick.is_same_player(from_brick)
    }

    pub fn get_actions(board: &Board, from: Point) -> Vec<Action> {
        let mut actions = Vec::<Action>::new();

        if board.get_tile(from) == Tile::Empty {
            return actions;
        }

        for direction in Direction::all() {
            let mut to = from;
            to.step(direction);
            if Action::can_step(board, from, to) {
                actions.push(Action::new(from, to));
            }
            to.step(direction);
            if Action::can_jump(board, from, to) {
                actions.push(Action::new(from, to));
            }
        }

        actions
    }

}
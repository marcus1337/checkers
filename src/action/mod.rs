
use super::board::tile::Direction;
use super::board::tile::Point;
use super::board::Board;
use super::board::tile::Tile;
use super::board::tile::Brick;
use super::board::tile::BrickType;
use super::board::tile::Player;
mod validate;

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
        Point::get_direction(self.from, self.to)
    }

    pub fn get_actions(board: &Board, from: Point) -> Vec<Action> {
        validate::get_possible_end_points(board, from)
        .into_iter()
        .map(|to| Action::new(from, to))
        .collect()
    }

    pub fn get_jump_actions(board: &Board, from: Point) -> Vec<Action> {
        Action::get_actions(board, from)
        .into_iter()
        .filter(|action| action.is_jump())
        .collect()
    }

}
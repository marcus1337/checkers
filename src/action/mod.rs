
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

    fn get_actions(board: &Board, from: Point) -> Vec<Action> {
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

    pub fn is_promoting(&self, board: &Board) -> bool {
        let from_brick = board.get_brick(self.from);
        match from_brick {
            Brick::PlayerBrick(Player::One, BrickType::Pawn) => self.to.row == 7,
            Brick::PlayerBrick(Player::Two, BrickType::Pawn) => self.to.row == 0,
            _ => false
        }
    }

    pub fn promote(&self, board: &mut Board) {
        let from_brick = board.get_brick(self.from);
        let king_brick = match from_brick {
            Brick::PlayerBrick(player, _) => Brick::PlayerBrick(player, BrickType::King),
        };
        board.place_brick(self.to, king_brick);
    }

    pub fn apply(&self, board: &mut Board) {
        let from_brick = board.get_brick(self.from);
        board.place_brick(self.to, from_brick);
        if self.is_jump() {
            let mut middle = self.from;
            middle.step_towards(self.to);
            board.remove_brick(middle);
        }
        if self.is_promoting(board) {
            self.promote(board);
        }
    }

}
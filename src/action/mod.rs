
use super::board::tile::Direction;
use super::board::tile::Point;
use super::board::Board;
use super::board::tile::Brick;
use super::board::tile::BrickType;
use super::board::tile::Player;
mod validate;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Action{
    pub from: Point,
    pub to: Point,
    pub from_brick: Brick,
    pub potentially_captured_brick: Brick,
    pub ends_turn: bool,
}


impl Action{
    pub fn new(board: &Board, from: Point, to: Point) -> Self {
        let mut action = Self{
            from:from,
            to:to,
            from_brick: board.get_brick(from),
            potentially_captured_brick: board.get_brick(from),
            ends_turn: true,
        };

        if action.is_jump() {
            action.potentially_captured_brick = board.get_brick(action.get_mid_point());
            let mut board_copy = board.clone();
            action.apply(&mut board_copy);
            action.ends_turn = Action::get_jump_actions(&board_copy, to).is_empty();
        }

        action
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
        .map(|to| Action::new(board, from, to))
        .collect()
    }

    pub fn get_step_actions(board: &Board, from: Point) -> Vec<Action> {
        Action::get_actions(board, from)
        .into_iter()
        .filter(|action| action.is_step())
        .collect()
    }

    pub fn get_jump_actions(board: &Board, from: Point) -> Vec<Action> {
        Action::get_actions(board, from)
        .into_iter()
        .filter(|action| action.is_jump())
        .collect()
    }

    pub fn is_promote(&self) -> bool {
        match self.from_brick {
            Brick::PlayerBrick(Player::One, BrickType::Pawn) => self.to.row == 7,
            Brick::PlayerBrick(Player::Two, BrickType::Pawn) => self.to.row == 0,
            _ => false
        }
    }

    fn get_promote_brick(&self) -> Brick {
        match self.from_brick {
            Brick::PlayerBrick(player, _) => Brick::PlayerBrick(player, BrickType::King),
        }
    }

    fn get_mid_point(&self) -> Point {
        let mut middle = self.from;
        middle.step_towards(self.to);
        middle
    }

    pub fn apply(&self, board: &mut Board) {
        board.place_brick(self.to, self.from_brick);
        if self.is_jump() {
            board.remove_brick(self.get_mid_point());
        }
        if self.is_promote() {
            board.place_brick(self.to, self.get_promote_brick());
        }
        board.remove_brick(self.from);
    }

    pub fn revert(&self, board: &mut Board) {
        if self.is_jump() {
            let mid_point = self.get_mid_point();
            board.place_brick(mid_point, self.potentially_captured_brick);
        }
        board.remove_brick(self.to);
        board.place_brick(self.from, self.from_brick);
    }

    pub fn new_null() -> Self {
        Self{
            from: Point::new(0, 0),
            to: Point::new(0, 0),
            from_brick: Brick::PlayerBrick(Player::One, BrickType::Pawn),
            potentially_captured_brick: Brick::PlayerBrick(Player::One, BrickType::Pawn),
            ends_turn: false,
        }
    }

    pub fn is_null(&self) -> bool {
        self.from == self.to
    }

}
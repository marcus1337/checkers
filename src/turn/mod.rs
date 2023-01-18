
pub mod move_validator;

use super::action::Action;
use super::board::tile::Player;
use super::board::Board;
use super::board;
use super::board::tile::Point;
use super::board::GameResult;
use super::History;
use super::Direction;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Turn {
    pub player: Player,
    pub board: Board,
    pub history: History,
}

impl Turn {
    pub fn new() -> Self {
        Self {
            player: Player::One,
            board: Board::new(),
            history: History::new(),
        }
    }

    fn next(&mut self) {
        self.player = match self.player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
    }

    pub fn apply_action(&mut self, action: Action) {
        action.apply(&mut self.board);
        if action.ends_turn {
            self.next();
        }
        self.history.add(action);
    }

    pub fn revert_action(&mut self, action: Action) {
        action.revert(&mut self.board);
        if action.ends_turn {
            self.next();
        }
    }

    pub fn undo(&mut self){
        let action = self.history.get_action();
        self.history.undo();
        self.revert_action(action);
    }

    pub fn redo(&mut self){
        let action = self.history.get_action();
        self.history.redo();
        self.apply_action(action);
    }

    pub fn get_result(&self) -> GameResult {
        let actions = self.get_valid_actions();
        if self.history.can_redo() || !actions.is_empty(){
            return GameResult::OnGoing;
        }
        if self.player == Player::One {
            return GameResult::TwoWin
        }else{
            return GameResult::OneWin;
        }
    }

    pub fn get_valid_actions(&self) -> Vec<Action>{
        if self.history.can_undo() {
            let last_action = self.history.get_action();
            if !last_action.ends_turn {
                return move_validator::get_valid_continuation_actions(&self.board, last_action.to);
            }else{
                return move_validator::get_valid_start_actions(&self.board, self.player)
            }
        }else{
            return move_validator::get_valid_start_actions(&self.board, self.player);
        }
    }

    pub fn find_action(&self, from: Point, direction: Direction) -> Option<Action> {
        return self.get_valid_actions().iter().find(|&action| action.from == from && action.get_direction() == direction).cloned();
    }


}

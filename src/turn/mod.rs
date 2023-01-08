
pub mod move_validator;

use super::action::Action;
use super::board::tile::Player;
use super::board::Board;
use super::board;
use super::board::tile::Point;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    Start,
    InProgress,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Turn {
    pub player: Player,
    pub board: Board,
    pub potentially_last_jump_action: Action,
}

impl Turn {
    pub fn new() -> Self {
        Self {
            player: Player::One,
            board: Board::new(),
            potentially_last_jump_action: Action::new_null(),
        }
    }

    fn next(&mut self) {
        self.player = match self.player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
        self.potentially_last_jump_action = Action::new_null();
    }

    pub fn apply_action(&mut self, action: Action) {
        action.apply(&mut self.board);
        self.set_last_jump_action(action);
        if action.is_step() || Action::get_jump_actions(&self.board, action.to).is_empty() {
            self.next();
        }
    }

    pub fn get_state(&self) -> TurnState {
        if !self.potentially_last_jump_action.is_null() {
            return TurnState::InProgress;
        }else{
            return TurnState::Start;
        }
    }

    pub fn get_jump_action_continuation_point(&self) -> Point {
        self.potentially_last_jump_action.to
    }

    fn set_last_jump_action(&mut self, last_action: Action) {
        self.potentially_last_jump_action = Action::new_null();
        if last_action.is_jump() && !Action::get_jump_actions(&self.board, last_action.to).is_empty() {
            self.potentially_last_jump_action = last_action;
        }
    }

}

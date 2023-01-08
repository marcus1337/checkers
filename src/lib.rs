
extern crate libc;
pub mod board;
pub mod ai;
pub mod action;
pub mod turn;
use self::turn::Turn;
use self::action::Action;
use self::turn::move_validator;
use rand::seq::SliceRandom;
use rand::thread_rng;
use board::tile::Point;
use board::tile::Direction;

#[repr(C)]
pub struct Checkers {
    turn: Turn,
}

impl Checkers{

    #[no_mangle]
    pub extern "C" fn checkers_make() -> Self {
        Self {
            turn: Turn::new(),
        }
    }

    #[no_mangle]
    pub extern "C" fn checkers_can_move(&self, from: Point, direction: Direction) -> bool {
        move_validator::can_step_or_jump(&self.turn, from, direction)
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_action(&self, from: Point, direction: Direction) -> Action {
        let actions = move_validator::get_valid_actions(&self.turn);
        actions.iter().find(|&action| action.from == from && action.get_direction() == direction).unwrap().clone()
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_ai_action(&self) -> Action {
        let actions = move_validator::get_valid_actions(&self.turn);
        let mut rng = thread_rng();
        let random_action = actions.choose(&mut rng).unwrap();
        random_action.clone()
    }

    #[no_mangle]
    pub extern "C" fn checkers_apply_action(&mut self, action: Action) {
        self.turn.apply_action(action);
    }

}


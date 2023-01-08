
extern crate libc;
pub mod board;
pub mod ai;
pub mod action;
pub mod turn;
use self::turn::Turn;
use self::action::Action;
use self::board::Board;
use self::turn::move_executor;
use self::turn::move_validator;
use board::tile::Direction;
use rand::seq::SliceRandom;
use rand::thread_rng;

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



extern crate libc;
pub mod board;
pub mod ai;
pub mod action;
pub mod turn;
use self::turn::Turn;
use self::action::Action;
use self::turn::move_validator;
use board::tile::BrickType;
use board::tile::Point;
use board::tile::Direction;
use board::tile::Brick;
use board::GameResult;
use board::tile::Player;
use board::Board;
use ai::AI;

#[repr(C)]
pub struct Checkers {
    pub turn: Turn,
}

impl Checkers{

    #[no_mangle]
    pub extern "C" fn checkers_make() -> Self {
        Self {
            turn: Turn::new(),
        }
    }

    #[no_mangle]
    pub extern "C" fn checkers_reset(&mut self) {
        *self = Checkers::checkers_make();
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_result(&self) -> GameResult {
        self.turn.get_result()
    }

    #[no_mangle]
    pub extern "C" fn checkers_is_started(&self) -> bool {
        self.turn.board != Board::new()
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_player_turn(&self) -> Player {
        self.turn.player
    }

    #[no_mangle]
    pub extern "C" fn checkers_has_brick(&self, point: Point) -> bool {
        self.turn.board.has_brick(point)
    }
    
    #[no_mangle]
    pub extern "C" fn checkers_get_brick(&self, point: Point) -> Brick {
        self.turn.board.get_brick(point)
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_player_of_brick(brick: Brick) -> Player {
        match brick {
            Brick::PlayerBrick(player,_) => player
        }
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_type_of_brick(brick: Brick) -> BrickType {
        match brick {
            Brick::PlayerBrick(_, brick_type) => brick_type
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
        AI::new().get_action(&self.turn)
    }

    #[no_mangle]
    pub extern "C" fn checkers_apply_action(&mut self, action: Action) {
        self.turn.apply_action(action);
    }

}


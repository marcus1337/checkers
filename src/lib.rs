
extern crate libc;
pub mod board;
pub mod ai;
pub mod action;
pub mod turn;
pub mod history;

use self::turn::Turn;
use self::action::Action;
use board::tile::BrickType;
use board::tile::Point;
use board::tile::Direction;
use board::tile::Brick;
use board::GameResult;
use board::tile::Player;
use board::Board;
use ai::AI;
use ai::evaluate::Evaluator;

use history::History;

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
    pub extern "C" fn checkers_can_undo(&self) -> bool {
        self.turn.history.can_undo()
    }
    #[no_mangle]
    pub extern "C" fn checkers_can_redo(&self) -> bool {
        self.turn.history.can_redo()
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_last_action(&self) -> Action {
        self.turn.history.get_action()
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_num_stored_actions(&self) -> i32 {
        self.turn.history.get_num_actions()
    }

    #[no_mangle]
    pub extern "C" fn get_stored_action(&self, action_number: i32) -> Action {
        self.turn.history.get_action_at_index(action_number)
    }

    #[no_mangle]
    pub extern "C" fn checkers_undo(&mut self) {
        self.turn.undo();
    }

    #[no_mangle]
    pub extern "C" fn checkers_redo(&mut self) {
        self.turn.redo();
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
        return self.turn.find_action(from, direction).is_some();
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_action(&self, from: Point, direction: Direction) -> Action {
        return self.turn.find_action(from, direction).unwrap().clone();
    }

    #[no_mangle]
    pub extern "C" fn checkers_get_ai_action(&self) -> Action {
       // println!("score: {}",Evaluator::new(self.turn.board).get_score());
        //AI::new().get_random_action(self.turn.clone())
        AI::new().get_best_action(self.turn)
    }

    #[no_mangle]
    pub extern "C" fn checkers_apply_action(&mut self, action: Action) {
        self.turn.apply_action(action);
        self.turn.history.add(action);
    }

}


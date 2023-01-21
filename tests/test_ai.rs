#[path = "../src/lib.rs"]
mod lib;
use lib::board::Board;
use lib::board;
use lib::board::tile;
use tile::Point;
use tile::Direction;
use lib::Checkers;
use lib::board::GameResult;
use lib::ai::AI;
use lib::ai::evaluate::Evaluator;

#[cfg(test)]
mod board_tests {
    use board::tile::{BrickType, Player, Brick};

    use super::*;

    #[test]
    fn make_moves() {
        let mut game = Checkers::checkers_make();
        while game.checkers_get_result() == GameResult::OnGoing {
            let action = game.checkers_get_ai_action();
            game.checkers_apply_action(action);
            if game.turn.board.get_occupied_tile_points_by_player(Player::One).len() < 8 || 
                game.turn.board.get_occupied_tile_points_by_player(Player::Two).len() < 8 {
                    break;
            }
        }
    }

    
/*    #[test]
    fn make_moves2() {
        let mut game = Checkers::checkers_make();
        while game.checkers_get_result() != GameResult::OnGoing {
            let action = game.checkers_get_ai_action();
            game.checkers_apply_action(action);

            if game.checkers_get_result() == GameResult::OnGoing {
                let action = AI::new().get_random_action(game.turn);
                game.checkers_apply_action(action);
            }
        }
    }

    #[test]
    fn make_moves3() {
        let mut game = Checkers::checkers_make();
        while game.checkers_get_result() != GameResult::OnGoing {
            println!("TESTT");
            let action = AI::new().get_random_action(game.turn);
            game.checkers_apply_action(action);

            if game.checkers_get_result() == GameResult::OnGoing {
                let action2 = game.checkers_get_ai_action();
                game.checkers_apply_action(action2);
            }
        }

        assert!(false);
    }*/

}
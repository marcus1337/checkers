
#[path = "../src/lib.rs"]
mod lib;
use lib::board::Board;
use lib::board;
use lib::board::tile;
use tile::Point;
use tile::Direction;

#[cfg(test)]
mod board_tests {
    use board::tile::{BrickType, Player, Brick};

    use super::*;

    #[test]
    fn can_redo_without_corruption() {
        let mut checkers = lib::Checkers::checkers_make();
        let action1 = checkers.checkers_get_action(Point::new(0,2), Direction::NorthEast);
        checkers.checkers_apply_action(action1);
        let action2 = checkers.checkers_get_action(Point::new(1,5), Direction::SouthWest);
        checkers.checkers_apply_action(action2);
        checkers.checkers_undo();
        checkers.checkers_undo();
        checkers.checkers_redo();
        checkers.checkers_redo();
        let action3 = checkers.checkers_get_action(Point::new(6,2), Direction::NorthEast);
        checkers.checkers_apply_action(action3);
    }

    #[test]
    fn can_add_after_undo() {
        let mut checkers = lib::Checkers::checkers_make();
        let action1 = checkers.checkers_get_action(Point::new(0,2), Direction::NorthEast);
        checkers.checkers_apply_action(action1);
        let action2 = checkers.checkers_get_action(Point::new(1,5), Direction::SouthWest);
        checkers.checkers_apply_action(action2);
        checkers.checkers_undo();
        checkers.checkers_undo();
        checkers.checkers_apply_action(action1);
        checkers.checkers_apply_action(action2);
        checkers.checkers_undo();
        checkers.checkers_undo();
        checkers.checkers_redo();
        checkers.checkers_redo();
        let action3 = checkers.checkers_get_action(Point::new(6,2), Direction::NorthEast);
        checkers.checkers_apply_action(action3);

        //println!("{}",checkers.turn.board);
        //assert!(false);
    }


}

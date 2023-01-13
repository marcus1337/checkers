
#[path = "../src/lib.rs"]
mod lib;
use lib::board::Board;
use lib::board;
use lib::board::tile;
use tile::Point;
use tile::Direction;

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn make_board() {
        let board = Board::new();
        println!("{}", board);
        //assert!(false);
    }

    #[test]
    fn can_move_bricks() {
        let mut checkers = lib::Checkers::checkers_make();
        let can_move_0_2_right = checkers.checkers_can_move(Point::new(0,2), Direction::NorthEast);
        let can_move_0_2_left = checkers.checkers_can_move(Point::new(0,2), Direction::NorthWest);
        assert_eq!(can_move_0_2_right, true);
        assert_eq!(can_move_0_2_left, false);
    }

    #[test]
    fn move_bricks() {
        let mut checkers = lib::Checkers::checkers_make();

        let from = Point::new(0,2);
        let to = Point::new(1,3);
        let action1 = checkers.checkers_get_action(from, Direction::NorthEast);
        checkers.checkers_apply_action(action1);

        let has_brick_from = checkers.checkers_has_brick(from);
        let has_brick_to = checkers.checkers_has_brick(to);
        assert_eq!(has_brick_from, false);
        assert_eq!(has_brick_to, true);
    }

}

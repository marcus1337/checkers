
#[path = "../src/lib.rs"]
mod lib;
use lib::board::Board;

#[cfg(test)]
mod board_tests {
    use super::*;
    #[test]
    fn make_board() {
        let board = Board::new();
        println!("{}", board);
        assert!(false);
    }
}

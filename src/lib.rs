
extern crate libc;
pub mod board;
pub mod ai;
pub mod action;
use self::board::Board;

#[repr(C)]
pub struct Checkers {
    board: Board,
}

impl Checkers{

    #[no_mangle]
    pub extern "C" fn checkers_make() -> Self {
        Self {
            board: Board::new(),
        }
    }

}


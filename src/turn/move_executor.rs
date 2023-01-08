/*use super::Turn;
use super::board::tile::Direction;
use super::board::tile::Point;
use super::Action;

pub fn step(turn: &mut Turn, from: Point, direction: Direction){
    let mut to = from;
    to.step(direction);
    let action = Action::new(&turn.board, from, to);
    turn.apply_action(action);
}

pub fn jump(turn: &mut Turn, from: Point, direction: Direction){
    let mut to = from;
    to.jump(direction);
    let action = Action::new(&turn.board, from, to);
    turn.apply_action(action);
}*/


use super::Board;
use super::Direction;
use super::Player;
use super::Point;
use super::Tile;

fn can_step(board: &Board, to: Point) -> bool {
    if !to.in_bounds() {
        return false;
    }
    !board.has_brick(to)
}

fn can_jump(board: &Board, from: Point, middle: Point, to: Point) -> bool {
    if !to.in_bounds() || board.has_brick(to) || !board.has_brick(middle) {
        return false;
    }
    let captured_brick = board.get_brick(middle);
    !captured_brick.is_same_player(board.get_brick(from))
}

pub fn get_possible_end_points(board: &Board, from: Point) -> Vec<Point> {
    if !board.has_brick(from) {
        return Vec::<Point>::new();
    }

    let mut end_points = Vec::<Point>::new();
    let from_brick = board.get_brick(from);

    for direction in Direction::all() {
        if from_brick.can_step_in_direction(direction) {
            let mut to = from;
            to.step(direction);
            if can_step(board, to) {
                end_points.push(to);
            }

            let middle = to;
            to.step(direction);
            if can_jump(board, from, middle, to) {
                end_points.push(to);
            }
        }
    }

    end_points
}

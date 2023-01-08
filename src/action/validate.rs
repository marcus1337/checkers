use super::Board;
use super::Player;
use super::Point;
use super::Tile;
use super::Direction;

fn can_step(board: &Board, from: Point, to: Point) -> bool {
    if !to.in_bounds() {
        return false;
    }
    let from_brick = board.get_brick(from);
    let to_tile = board.get_tile(to);
    let direction = Point::get_direction(from, to);
    to_tile == Tile::Empty && from_brick.can_step_in_direction(direction)
}

fn can_jump(board: &Board, from: Point, to: Point) -> bool {
    if !to.in_bounds() {
        return false;
    }

    let direction = Point::get_direction(from, to);
    let mut mid_point = from;
    mid_point.step(direction);
    if !board.has_brick(mid_point) {
        return false;
    }

    let captured_brick = board.get_brick(mid_point);
    let from_brick = board.get_brick(from);
    let to_tile = board.get_tile(to);

    to_tile == Tile::Empty && from_brick.can_step_in_direction(direction) && !captured_brick.is_same_player(from_brick)
}

pub fn get_possible_end_points(board: &Board, from: Point) -> Vec<Point> {
    
    if !board.has_brick(from) {
        return Vec::<Point>::new();
    }

    let mut end_points = Vec::<Point>::new();
    for direction in Direction::all() {
        let mut to = from;
        to.step(direction);
        if can_step(board, from, to) {
            end_points.push(to);
        }
        to.step(direction);
        if can_jump(board, from, to) {
            end_points.push(to);
        }
    }

    end_points
}
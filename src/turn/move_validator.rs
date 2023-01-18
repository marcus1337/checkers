use super::board::tile::Direction;
use super::board::tile::Point;
use super::Action;
use super::Player;
use super::Board;

fn can_step(board: &Board, from: Point, direction: Direction) -> bool {
    let mut to = from;
    to.step(direction);
    if !to.in_bounds() || !board.has_brick(from) {
        return false;
    }
    Action::get_step_actions(&board, from)
        .iter()
        .any(|action| action.get_direction() == direction)
}

fn can_jump(board: &Board, from: Point, direction: Direction) -> bool {
    let mut mid_point = from;
    mid_point.step(direction);
    let mut to = from;
    to.jump(direction);
    if !to.in_bounds() || !board.has_brick(from) || !board.has_brick(mid_point) || board.has_brick(to) {
        return false;
    }
    if board.get_brick(mid_point).is_same_player(board.get_brick(from)) {
        return false;
    }

    Action::get_jump_actions(&board, from)
        .iter()
        .any(|action| action.get_direction() == direction)
}

fn get_step_actions(board: &Board, player: Player) -> Vec<Action> {
    let mut actions = Vec::new();
    for from in board.get_occupied_tile_points_by_player(player) {
        for direction in Direction::all() {
            if can_step(&board, from, direction) {
                let mut to = from;
                to.step(direction);
                let action = Action::new(&board, from, to);
                actions.push(action);
            }
        }
    }
    actions
}

fn get_jump_actions(board: &Board, player: Player) -> Vec<Action> {
    let mut actions = Vec::new();
    for from in board.get_occupied_tile_points_by_player(player) {
        for direction in Direction::all() {
            if can_jump(&board, from, direction) {
                let mut to = from;
                to.jump(direction);
                let action = Action::new(&board, from, to);
                actions.push(action);
            }
        }
    }
    actions
}

pub fn get_valid_start_actions(board: &Board, player: Player) -> Vec<Action> {
    let jump_actions = get_jump_actions(board, player);
    let step_actions = get_step_actions(board, player);
    if !jump_actions.is_empty(){
        return jump_actions;
    }else{
        return step_actions;
    }
}

pub fn get_valid_continuation_actions(board: &Board, last_to: Point) -> Vec<Action> {
    let player = board.get_brick(last_to).get_player();
    let mut actions = Vec::new();
    for action in get_jump_actions(board, player) {
        if action.from == last_to {
            actions.push(action);
        }
    }
    actions
}



use super::board::tile::Direction;
use super::board::tile::Point;
//use super::board::Board;
use super::Action;
use super::Turn;
use super::TurnState;

fn can_jump_somewhere(turn: &Turn) -> bool {
    for point in turn.board.get_occupied_tile_points_by_player(turn.player) {
        if !Action::get_jump_actions(&turn.board, point).is_empty() {
            return true;
        }
    }
    false
}

fn has_to_jump(turn: &Turn) -> bool {
    can_jump_somewhere(turn)
}

pub fn can_step_or_jump(turn: &Turn, from: Point, direction: Direction) -> bool {
    if can_step(turn, from, direction) || can_jump(turn, from, direction) {
        return true;
    }
    false
}

pub fn can_step(turn: &Turn, from: Point, direction: Direction) -> bool {
    let board = turn.board;
    let player = turn.player;

    let mut to = from;
    to.step(direction);

    if !to.in_bounds() || !board.has_player_brick(from, player) || has_to_jump(turn) {
        return false;
    }

    Action::get_step_actions(&board, from)
        .iter()
        .any(|action| action.get_direction() == direction)
}

pub fn can_jump(turn: &Turn, from: Point, direction: Direction) -> bool {
    let board = turn.board;
    let player = turn.player;

    let mut to = from;
    to.jump(direction);

    if !to.in_bounds() || !board.has_player_brick(from, player) {
        return false;
    }

    if has_to_jump(turn) && turn.get_state() == TurnState::InProgress {
        let last_to_point = turn.get_jump_action_continuation_point();
        if last_to_point != from {
            return false;
        }
    }

    Action::get_jump_actions(&board, from)
        .iter()
        .any(|action| action.get_direction() == direction)
}

pub fn get_valid_actions(turn: &Turn) -> Vec<Action> {
    let mut actions = Vec::new();

    for from in turn.board.get_occupied_tile_points_by_player(turn.player) {
        for direction in Direction::all() {
            if can_step(&turn, from, direction) {
                let mut to = from;
                to.step(direction);
                let action = Action::new(&turn.board, from, to);
                actions.push(action);
            }
            if can_jump(&turn, from, direction) {
                let mut to = from;
                to.jump(direction);
                let action = Action::new(&turn.board, from, to);
                actions.push(action);
            }
        }
    }

    actions
}

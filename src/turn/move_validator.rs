use super::board::tile::Direction;
use super::board::tile::Point;
use super::board::Board;
use super::Action;
use super::Turn;
use super::TurnState;

fn has_to_jump(turn: &Turn) -> bool {
    turn.get_state() == TurnState::InProgress
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

    if !board.has_player_brick(from, player) || has_to_jump(turn) {
        return false;
    }

    Action::get_step_actions(&board, from)
        .iter()
        .any(|action| action.get_direction() == direction)
}

pub fn can_jump(turn: &Turn, from: Point, direction: Direction) -> bool {
    let board = turn.board;
    let player = turn.player;

    if !board.has_player_brick(from, player) {
        return false;
    }

    if has_to_jump(turn) {
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

    for from in Board::get_tile_points() {
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

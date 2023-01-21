
use super::GameResult;
//use super::Board;
use super::Action;
use super::evaluate::Evaluator;
use super::Turn;
use super::Player;
use rayon::prelude::*;

fn max_minimax(turn: Turn, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    let mut best_score = std::i32::MIN;
    for action in turn.get_valid_actions() {
        let mut turn_copy = turn.clone();
        turn_copy.apply_action(action);
        turn_copy.history.add(action);
        let score = minimax(turn_copy.clone(), depth-1, false, alpha, beta);
        best_score = std::cmp::max(best_score, score);
        alpha = std::cmp::max(alpha, best_score);
        if beta <= alpha {
            return best_score;
        }
    }
    best_score
}

fn min_minimax(turn: Turn, depth: i32, alpha: i32, mut beta: i32) -> i32 {
    let mut best_score = std::i32::MAX;
    for action in turn.get_valid_actions() {
        let mut turn_copy = turn.clone();
        turn_copy.apply_action(action);
        turn_copy.history.add(action);
        let score = minimax(turn_copy.clone(), depth-1, true, alpha, beta);
        best_score = std::cmp::min(best_score, score);
        beta = std::cmp::min(beta, best_score);
        if beta <= alpha {
            return best_score;
        }
    }
    best_score
}

fn minimax(turn: Turn, depth: i32, maximizing_player: bool, alpha: i32, beta: i32) -> i32 {
    if turn.get_result() != GameResult::OnGoing || depth == 0 {
        return Evaluator::new(turn.board).get_score(); 
    }
    if maximizing_player{
        return max_minimax(turn, depth, alpha, beta);
    }else{
        return min_minimax(turn, depth, alpha, beta);
    }
}

pub fn get_minimax_action(turn: Turn, depth: i32) -> Action {
    let mut chosen_action = Action::new_null();
    let maximizing_player = turn.player == Player::One;
    let mut best_score = if maximizing_player { std::i32::MIN } else {std::i32::MAX};

    let possible_actions = turn.get_valid_actions();
    let scores: Vec<i32> = possible_actions.par_iter().map(|&action| {
        let mut turn_copy = turn.clone();
        turn_copy.apply_action(action);
        turn_copy.history.add(action);
        minimax(turn_copy, depth, !maximizing_player, std::i32::MIN, std::i32::MAX)
    }).collect();

    for i in 0 .. scores.len() {
        if maximizing_player && scores[i] > best_score {
            best_score = scores[i];
            chosen_action = possible_actions[i];
        }else if !maximizing_player && scores[i] < best_score {
            best_score = scores[i];
            chosen_action = possible_actions[i];
        }
    }

    chosen_action
}
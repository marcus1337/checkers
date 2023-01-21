pub mod evaluate;
pub mod minimax;

use rand::seq::SliceRandom;
use rand::thread_rng;
use super::Action;
use super::turn::Turn;
use super::Board;
use super::Player;
use super::GameResult;
use super::BrickType;


pub struct AI {

}

impl AI{

    pub fn new() -> Self{
        Self {

        }
    }

    pub fn get_best_action(&self, turn: Turn) -> Action {
        minimax::get_minimax_action(turn.clone(), 8)
    }

    pub fn get_random_action(&self, turn: Turn) -> Action {
        let actions = turn.get_valid_actions();
        let mut rng = thread_rng();
        let random_action = actions.choose(&mut rng).unwrap();
        random_action.clone()
    }
}
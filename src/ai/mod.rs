
use rand::seq::SliceRandom;
use rand::thread_rng;
use super::Action;
use super::turn::move_validator;
use super::turn::Turn;

pub struct AI {

}

impl AI{

    pub fn new() -> Self{
        Self {

        }
    }

    pub fn get_action(&self, turn: &Turn) -> Action {
        let actions = turn.get_valid_actions();
        let mut rng = thread_rng();
        let random_action = actions.choose(&mut rng).unwrap();
        random_action.clone()
    }
}
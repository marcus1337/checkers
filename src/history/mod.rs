
use super::action::Action;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct History { //Circular buffer
    pub actions: [Action; 100],
    action_index: i32,
    num_stored_undo_actions: i32,
    num_stored_redo_actions: i32,
}

impl History{
    pub fn new() -> Self {
        Self {
            actions: [Action::new_null(); 100],
            action_index: 0,
            num_stored_undo_actions: 0,
            num_stored_redo_actions: 0,
        }
    }

    pub fn add(&mut self, action: Action){
        self.num_stored_redo_actions = 0;
        self.num_stored_undo_actions += 1;
        self.actions[self.action_index as usize] = action;
        self.action_index = (self.action_index + 1) % self.actions.len() as i32;
    }

    fn get_previous_index(&self) -> i32 {
        (self.action_index + self.actions.len() as i32 - 1) % self.actions.len() as i32
    }

    pub fn get_action(&self) -> Action {
        let previous_index = self.get_previous_index();
        self.actions[previous_index as usize].clone()
    }

    pub fn is_empty(&self) -> bool {
        !self.can_undo() && !self.can_redo()
    }

    pub fn undo(&mut self) {
        self.action_index = self.get_previous_index();
        self.num_stored_undo_actions -= 1;
        self.num_stored_redo_actions += 1;
    }

    pub fn redo(&mut self) {
        self.action_index = (self.action_index + 1) % self.actions.len() as i32;
        self.num_stored_undo_actions += 1;
        self.num_stored_redo_actions -= 1;
    }

    pub fn can_undo(&self) -> bool {
        self.num_stored_undo_actions > 0
    }
    pub fn can_redo(&self) -> bool {
        self.num_stored_redo_actions > 0
    }

}
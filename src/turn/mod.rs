
use super::board::tile::Player;
use super::board::Board;
use super::board::GameResult;
use super::action::Action;

enum TurnState {
    Start,
    InProgress,
}

struct Turn{
    pub player: Player,
    pub state: TurnState,
    pub board: Board,
}

impl Turn {

    pub fn new() -> Self {
        Self { player: Player::One, state: TurnState::Start, board: Board::new() }
    }

    fn next(&mut self) {
        self.player = match self.player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
        self.state = TurnState::Start;
    }

    pub fn apply_action(&mut self, action: Action){
        self.state = TurnState::InProgress; 
        action.apply(&mut self.board);
        if action.is_step() || Action::get_jump_actions(&self.board, action.to).is_empty() {
            self.next();
        }
    }

}
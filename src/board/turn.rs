
use tile::Point;
use tile::Brick;
use tile::Player;
use board::Board;

struct Turn{
    last_from: Point,
    last_to: Point,
}

impl Turn{

    pub fn new() -> Self {
        Self {
            last_from: Point::new(-1, -1),
            last_to: Point::new(-1, -1),
        }
    }

    pub fn set_last_move(&mut self, from: Point, to: Point) {
        self.last_from = from;
        self.last_to = to;
    }

    //can_current_turn_move
    //has_current_made_move
    //is_current_turn_done

    pub fn get_player_turn(&mut self, board: &Board) -> Player {
        //let brick_player_type = board.get_brick(self.last_to_point);

        //board -- get possible moves from <last_to_point> --> no "take move" --> return other player.
        //if last_from_point == <-1,-1> return Player::One
        
        Player::One
    }

}


use tile::Point;
use tile::Brick;
use tile::Player;
use board::Board;

struct Turn{
    last_to_point: Point,
}

impl Turn{

    pub fn new() -> Self {
        Self {
            last_to_point: Point::new(-1, -1),
        }
    }

    pub fn set_to_point(&mut self, to_point: Point) {
        self.last_to_point = to_point;
    }

    pub fn get_player_turn(&mut self, board: &Board) -> Player {
        //let brick_player_type = board.get_brick(self.last_to_point);

        //board -- get possible moves from <last_to_point> --> no "take move" --> return other player.
        //if last_from_point == <-1,-1> return Player::One
        
        Player::One
    }

}

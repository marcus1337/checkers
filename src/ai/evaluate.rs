
use super::Board;
use super::Player;
use super::BrickType;

pub struct Evaluator {
    board: Board,
}

impl Evaluator {
    pub fn new(board: Board) -> Self {
        Self { board: board }
    }

    fn get_num_bricks(&self, player: Player, brick_type: BrickType) -> i32{
        let mut num_bricks = 0;
        for point in self.board.get_occupied_tile_points_by_player(player){
            if self.board.get_brick(point).is_pawn() && brick_type == BrickType::Pawn {
                num_bricks += 1;
            }
            if self.board.get_brick(point).is_king() && brick_type == BrickType::King {
                num_bricks += 1;
            }
        }
        num_bricks
    }

    fn get_num_pawns(&self, player: Player) -> i32 {
        self.get_num_bricks(player, BrickType::Pawn)
    }

    fn get_num_kings(&self, player: Player) -> i32 {
        self.get_num_bricks(player, BrickType::King)
    }

    fn get_player_score(&self, player: Player) -> i32 {
        let num_pawns = self.get_num_pawns(player);
        let num_kings = self.get_num_kings(player);
        let king_score = 2;
        num_pawns + num_kings * king_score
    }

    pub fn get_score(&self) -> i32 {
        self.get_player_score(Player::One) - self.get_player_score(Player::Two)
    }
}

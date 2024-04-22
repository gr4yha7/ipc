use std::{borrow::BorrowMut, error::Error};
use pleco_engine::{
  engine::PlecoSearcher, time::uci_timer::PreLimits
};
use pleco::Board;

const END_GAME: &str = "endgame";
pub struct ChessEngine {
  searcher: PlecoSearcher,
  game_mode: GameMode
}

enum GameMode {
  Easy,
  Hard
}

impl ChessEngine {
  fn new() -> Self {
    Self {
      searcher: PlecoSearcher::init(false),
      game_mode: GameMode::Easy
    }
  }

  fn start_game(self) -> String {
    let board = Board::start_pos();
    board.fen()
  }

  fn play_move(&mut self, fen_str: String) -> Result<String, Box<dyn Error>> {
    let mut current_board = Board::from_fen(fen_str.as_str()).unwrap();
    if current_board.checkmate() {
      // current_board.turn()
      return Ok(END_GAME.to_string());
    }
    
    // Set limits for easy and hard game modes
    let mut limits = PreLimits::blank();
    match self.game_mode {
        GameMode::Easy => {
          limits.depth = Some(10)
        },
        GameMode::Hard => {
          limits.mate = Some(5) // checkmate in five moves
        }
    }

    self.searcher.search(&current_board, &limits);
    let engine_move = self.searcher.await_move();


    // let engine_move = threadpool().search(&current_board, &limits);
    let new_board: String;
    if is_valid_move(current_board.borrow_mut(), engine_move.to_string().as_str()) {
      current_board.apply_move(engine_move);
      if current_board.checkmate() {
        return Ok(END_GAME.to_string());
      }
      new_board = current_board.fen();
      Ok(new_board)
    } else {
      Err("invalid move".into())
    }
  }

  pub fn get_move(&mut self, fen: String, depth: u16) -> String {
    let mut limit = PreLimits::blank();
    limit.depth = Some(depth);
    let board = Board::from_fen(fen.as_str()).unwrap();
    // let mut s = PlecoSearcher::init(false);

    self.searcher.search(&board, &limit);
    let bit_move = self.searcher.await_move();

    bit_move.to_string()
  }
}

pub fn init() -> ChessEngine {
  let chess_engine = ChessEngine::new();
  chess_engine
}

fn is_valid_move(board: &mut Board, mov: &str) -> bool {
  let all_moves = board
      .generate_moves()
      .iter()
      .map(|m| m.stringify())
      .collect::<Vec<String>>();

  return all_moves.contains(&mov.to_string());
}

// Actor call sequence ---> run syscall when bottomup msg comes in from a child subnet
// don't charge gas?

#[cfg(test)]
mod tests {
    use pleco::board::fen::OPENING_POS_FEN;
    // use fendermint_vm_actor_interface::eam::EthAddress;

    use super::*;

    
    #[test]
    fn test_setup_chess_board() {
        let engine = init();
        assert_eq!(engine.start_game(), OPENING_POS_FEN);
        // let addr = EthAddress::from_id(49); // 0xff00000000000000000000000000000000000031
        // println!("actor addr = {:?}", addr);
    }

    #[test]
    fn it_works() {
      let mut engine = init();
      let result = engine.get_move(
          "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
          10,
      );
      assert_eq!(result, "e2e4");
    }
}


use std::{borrow::BorrowMut, error::Error};

// use pleco::bots::ParallelMiniMaxSearcher;
use pleco_engine::{
  engine::PlecoSearcher,threadpool::threadpool, time::uci_timer::PreLimits
};
use pleco::{tools::pleco_arc::Arc, BitMove, Board};


// Get fen string from player (subnet)
// Construct board from fen string (Board::from_fen())
// Get AI move (await_move()) --- get best move from pleco searcher
// Check if AI move is valid ---> if board.generateMoves() contains AI move
// Apply move
// Get fen string from board (board.fen()) ---> send to subnet (frontend)

pub struct ChessEngine {
  searcher: Arc<PlecoSearcher>
}

impl ChessEngine {
  fn new() -> Self {
    Self {
      searcher: Arc::new(PlecoSearcher::init(false))
    }
  }

  fn start_game(self) -> String {
    let board = Board::start_pos();
    board.fen()
  }

  fn play_move(&self, fen_str: &str) -> Result<String, Box<dyn Error>> {
    let mut current_board = Board::from_fen(fen_str).unwrap();
    if current_board.checkmate() {
      // todo! end game
    }
    // todo!(
    //   // set appropriate limits for easy and hard game modes
    // );
    let mut prelimits = PreLimits::blank();
    prelimits.depth = Some(10);

    let limits = PreLimits::create(prelimits);

    // let engine_move = self.searcher.await_move();
    let engine_move = threadpool().search(&current_board, &limits);
    let new_board: String;
    if is_valid_move(current_board.borrow_mut(), engine_move.stringify().as_str()) {
      current_board.apply_move(engine_move);
      if current_board.checkmate() {
        // todo! end game
      }
      new_board = current_board.fen();
      Ok(new_board)
    } else {
      Err("invalid move".into())
    }
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
    use fendermint_vm_actor_interface::eam::EthAddress;

    use super::*;

    
    #[test]
    fn test_setup_chess_board() {
        let engine = init();
        assert_eq!(engine.start_game(), OPENING_POS_FEN);
        let addr = EthAddress::from_id(49);
        println!("actor addr = {:?}", addr);
        // assert_eq!([0u8; 20], addr.to_vec());
    }
}


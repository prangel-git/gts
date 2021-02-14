use super::super::abstract_game::*;

// Identity of tic tac toe players
#[derive(Debug, PartialEq)]
pub enum AgentId{
  X,
  O,
}

type Action = u8;

// Representation of the tic tac toe board
pub struct Board {
  moves_x: u16,             // As a binary string. Puts a 1 in the positions where X moved
  moves_y: u16,             // As a binary string. Puts a 1 in the positions where Y moved
  current_player: AgentId, // Player that will make the next move
}


impl Environment<Action, AgentId> for Board {
  fn initial_state() -> Self {
        Board { moves_x: 0, moves_y: 0, current_player: AgentId::X }
    }

  fn update(&mut self, 
    agent_id: &AgentId, 
    a: &Action
  ) -> bool {
      // Checks whether the board is empty in position a
      if !self.is_valid(agent_id, a) {
        return false;
      } else {
        let m = 1 << a;
        if *agent_id == AgentId::X {
            self.moves_x |= m; 
            self.current_player = AgentId::O
          } else {
            self.moves_y |= m; 
            self.current_player = AgentId::X
          }
        return true;      
      }
    }

  fn is_valid(&self, 
    agent_id: &AgentId, 
    &a: &Action
  ) -> bool {
      let x_empty = !(((self.moves_x >> a) & 1) == 1);
      let y_empty = !(((self.moves_y >> a) & 1) == 1);
      let valid_player = self.is_valid_player(&agent_id);
      return x_empty & y_empty & valid_player;
    }

  fn is_valid_player(&self, 
    agent_id: &AgentId
  ) -> bool {
      return *agent_id == self.current_player;
    }

  fn is_terminal(&self) -> bool {
        if is_winning(self.moves_x) { return true; }
        else if is_winning(self.moves_y) { return true; }
        else if is_filled(&self) { return true; }
        else { return false; }
    }
}

// Checks whether one of the players has a winning position.
fn is_winning(position: u16) -> bool {

  let winning_masks = vec![
    0b111u16, 
    0b111000u16, 
    0b111000000u16, 
    0b1001001u16, 
    0b10010010u16, 
    0b100100100u16, 
    0b100010001u16, 
    0b1010100u16];

  for mask in winning_masks{
    if position & mask == mask { return true;}
  }
  return false;
}

// Checks whether the whole board is filled
fn is_filled(board: &Board) -> bool {
  let full = 0b111111111u16;
  let fill = (board.moves_x | board.moves_y) & full;
  return fill == full;
}

#[test]
fn manual_game() {
  let mut board = Board::initial_state();
  assert_eq!(board.moves_x, 0);
  assert_eq!(board.moves_y, 0);
  assert_eq!(board.current_player, AgentId::X);

  assert_eq!(board.update(&AgentId::X, &4), true);
  assert_eq!(board.moves_x, 0b10000);
  assert_eq!(board.current_player, AgentId::O);

  assert_eq!(board.update(&AgentId::O, &5), true);
  assert_eq!(board.moves_y, 0b100000);
  assert_eq!(board.current_player, AgentId::X);

  assert_eq!(board.update(&AgentId::X, &0), true);
  assert_eq!(board.moves_x, 0b10001);
  assert_eq!(board.current_player, AgentId::O);

  assert_eq!(board.update(&AgentId::O, &0), false);

  assert_eq!(board.update(&AgentId::O, &1), true);
  assert_eq!(board.moves_y, 0b100010);
  assert_eq!(board.current_player, AgentId::X);

  assert_eq!(board.update(&AgentId::X, &8), true);
  assert_eq!(board.moves_x, 0b100010001);
  assert_eq!(board.current_player, AgentId::O);

  assert_eq!(is_filled(&board), false);
  assert_eq!(is_winning(board.moves_y), false);
  assert_eq!(is_winning(board.moves_x), true);
  assert_eq!(board.is_terminal(), true);
}
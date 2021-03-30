/// Actions will be a number from 0 to 8 representing the position on the tic tac toe board.
pub type Action = u8;

mod agentid;
pub use self::agentid::AgentId;

mod board;
pub use self::board::Board;

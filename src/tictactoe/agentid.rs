use std::fmt;
/// Identity of tic tac toe players
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AgentId {
    X,
    O,
}

/// Display trait for tic tac toe players.
impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgentId::O => write!(f, "O"),
            AgentId::X => write!(f, "X"),
        }
    }
}

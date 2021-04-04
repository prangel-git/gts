use std::fmt::{Debug, Display};
use std::{io, str::FromStr};

use crate::abstractions::Agent;
use crate::abstractions::Environment;

/// A human player. It stores the identity of the player in a given environment.
pub struct HumanPlayer<AgentId>
where
    AgentId: Copy + Eq,
{
    agent_id: AgentId,
}

/// Methods used by a human player
impl<AgentId> HumanPlayer<AgentId>
where
    AgentId: Copy + Eq,
{
    pub fn new(agent_id: AgentId) -> Self {
        Self { agent_id }
    }
}

/// Implements the agent trait for human players. The idea is that this player will
/// the action from the keyboard.
impl<Action, AgentId, T> Agent<Action, AgentId, T> for HumanPlayer<AgentId>
where
    Action: FromStr,
    AgentId: Debug + Copy + Eq,
    T: Environment<Action, AgentId> + Display,
{
    /// Returns the identity of the agent in the environment T.
    fn identity(&self) -> AgentId {
        return self.agent_id;
    }

    /// Returns the agent's action given an environment.
    fn action(&mut self, env: &T) -> Option<Action> {
        println!("The current board looks like:");
        println!("{}", env);
        println!("You are player: {:?}.", self.agent_id);

        println!("Please enter your action: ");

        let mut buf = String::new();

        match io::stdin().read_line(&mut buf) {
            Ok(_) => println!("Input read correctly"),
            Err(error) => println!("Error reading input {}", error),
        }

        match buf.trim().parse::<Action>() {
            Ok(act) => Some(act),
            Err(_) => self.action(env),
        }
    }
}

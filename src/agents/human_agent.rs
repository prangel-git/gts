
use std::{io, str::FromStr};
use std::fmt::Display;

use super::super::abstract_game::*;

struct HumanPlayer<AgentId>
where
AgentId: Display + Copy 
{
    agent_id: AgentId,
}

impl<Action, AgentId, T> Agent<Action, AgentId, T> for HumanPlayer<AgentId> 
where 
Action: FromStr, 
AgentId: Display + Copy,
T: Environment<Action, AgentId> + Display {
    // Creates a new agent with a given identity. 
  fn new(agent_id: &AgentId) -> Self {
      HumanPlayer{agent_id: *agent_id}
  }

  // Returns the identity of the agent in the environment T.
  fn agent_identity(&self) -> AgentId {
      return self.agent_id;
  }

  // Returns the agent's action given an environment.
  fn action(&mut self, env: &T) -> Action {
    
    let player_str = self.agent_id.to_string();
    let env_str = env.to_string();

    println!("The current board looks like:");
    println!("{}", env_str);
    println!("You are player: {}.", player_str);

    println!("Please enter your action: ");

    let mut buf = String::new();
    
    match io::stdin().read_line(&mut buf) {
        Ok(_) => println!("Input read correctly"),
        Err(error) => println!("Error reading input {}", error),
    }
    
    match buf.parse::<Action>() {
        Ok(a) => a,
        Err(_) => self.action(env),
    }
  }
}
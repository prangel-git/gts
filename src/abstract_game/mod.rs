// Contains the traits and functions required to implement a board game.


// Functions required to represent an environment.
pub trait Environment {
  type Action;            // Type of the action supported by the environment.
  type AgentIdentity;     // Type of the identity of an agent interacting with the environment.

  // Produces an initial environment
  fn initial_state() -> Self;       
  
  // Returns true iff the environment gets updated when 'agent' performs action 'a'.
  fn update(&mut self, 
    agent_id: &Self::AgentIdentity, 
    a: &Self::Action
  ) -> bool;

  // Returns true iff 'agent' can perform action 'a'.
  fn is_valid(&self, 
    agent_id: &Self::AgentIdentity, 
    a: &Self::Action
  ) -> bool;
  
  // Returns true iff 'agent' can perform an action.
  fn is_valid_player(&self, 
    agent_id: &Self::AgentIdentity
  ) -> bool;

  // Returns true if the environment is in a terminal position.
  fn is_terminal(&self) -> bool;
}

// Functions required to implement a valid agent for an environment T.
pub trait Agent<T:Environment> {
  // Creates a new agent with a given identity. 
  fn new(agent_id: &T::AgentIdentity) -> Self;

  // Returns the identity of the agent in the environment T.
  fn agent_identity(&self) -> T::AgentIdentity;

  // Returns the agent's action given an environment.
  fn action(&mut self, env: &T) -> T::Action;
}

// Produces one move of the game
pub fn one_move<T: Environment + Clone, R:Agent<T>>(
  env: & mut T, 
  agent: &mut R) 
  -> T {  

    env.update(
      &agent.agent_identity(), 
      &agent.action(env));
    
    return env.clone();
}

// TODO: Figure out how to have an arbitrary number of players

// Plays a game with two players
pub fn play_game<T: Environment + Clone, R:Agent<T>>(
  env: &mut T, 
  agent_1: &mut R, 
  agent_2 : &mut R
) -> Vec<T> {
  let mut game_log = Vec::new();

  while !env.is_terminal() {

    while env.is_valid_player(&agent_1.agent_identity()) {
      env.update(&agent_1.agent_identity(), &agent_1.action(env));
      game_log.push(env.clone())
    }
    
    while env.is_valid_player(&agent_2.agent_identity()) {
      env.update(&agent_2.agent_identity(), &agent_2.action(env));
      game_log.push(env.clone());
    }
  }

  return game_log;
}
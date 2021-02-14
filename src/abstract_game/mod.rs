// Contains the traits and functions required to implement a board game.


// Functions required to represent an environment. 
// Action: Type of the actions on the environment.
// AgentId: Type of the agent identificaiton.
pub trait Environment<Action, AgentId> {
  // Produces an initial environment
  fn initial_state() -> Self;       
  
  // Returns true iff the environment gets updated when 'agent' performs action 'a'.
  fn update(&mut self, 
    agent_id: &AgentId, 
    a: &Action
  ) -> bool;

  // Returns true iff 'agent' can perform action 'a'.
  fn is_valid(&self, 
    agent_id: &AgentId, 
    a: &Action
  ) -> bool;
  
  // Returns true iff 'agent' can perform an action.
  fn is_valid_player(&self, 
    agent_id: &AgentId
  ) -> bool;

  // Returns true if the environment is in a terminal position.
  fn is_terminal(&self) -> bool;
}

// Functions required to implement a valid agent for an environment T.
pub trait Agent<Action, AgentId, T> where T: Environment<Action, AgentId> {
  // Returns the identity of the agent in the environment T.
  fn agent_identity(&self) -> AgentId;

  // Returns the agent's action given an environment.
  fn action(&mut self, env: &T) -> Action;
}

// Produces one move of the game
pub fn one_move<Action, AgentId, T: Environment<Action, AgentId> + Clone, R:Agent<Action, AgentId, T>>(
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
pub fn play_game<Action, AgentId, T: Environment<Action, AgentId> + Clone, R:Agent<Action, AgentId, T>> (
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
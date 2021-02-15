// Functions required to represent an environment.
// Action: Type of the actions on the environment.
// AgentId: Type of the agent identificaiton.
pub trait Environment<Action, AgentId>
where
    AgentId: Eq,
{
    // Produces an initial environment
    fn initial_state() -> Self;

    // Returns true iff the environment gets updated when 'agent' performs action 'a'.
    fn update(&mut self, agent_id: &AgentId, a: &Action) -> bool;

    // Returns a vector with the valid actions for a given agent
    fn valid_actions(&self, agent_id: &AgentId) -> Vec<Action>;

    // Returns true iff the environment accepts 'action'.
    fn is_valid(&self, action: &Action) -> bool;

    // Returns true if the environment is in a terminal position.
    fn is_terminal(&self) -> bool;

    // Returns the identity of the current agent
    fn turn(&self) -> AgentId;

    // Returns the winner of a final game
    fn winner(&self) -> Option<AgentId>;
}

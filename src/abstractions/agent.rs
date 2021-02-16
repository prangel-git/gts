use super::environment::Environment;

// Functions required to implement a valid agent for an environment T.
// Action: Type of the action performed by agent.
// AgentId: Type of the agent identity in the environment.
pub trait Agent<Action, AgentId, T>
where
    AgentId: Eq,
    T: Environment<Action, AgentId>,
{
    // Returns the identity of the agent in the environment T.
    fn identity(&self) -> AgentId;

    // Returns the agent's action given an environment.
    fn action(&mut self, env: &T) -> Action;

    // TODO: It might be best if method action returns an Option<Action> instead.
}

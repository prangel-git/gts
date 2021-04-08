/// Functions required to represent an environment.
/// Action: Type of the actions that the environment accepts.
/// AgentId: Type of the agent identity in the environment.
pub trait Environment<Action, AgentId> {
    type ActionIter: Iterator<Item = Action>;

    /// Produces an initial environment
    fn initial_state() -> Self;

    /// Returns true iff the environment gets updated when 'agent' performs action 'a'.
    fn update(&mut self, a: &Action) -> bool;

    /// Returns what would happen if an agent attempts a given action
    fn what_if(&self, a: &Action) -> Self;

    /// Returns an iterator with the valid actions for a given agent
    fn valid_actions(&self) -> Self::ActionIter;

    /// Returns true iff the environment accepts 'action'.
    fn is_valid(&self, action: &Action) -> bool;

    /// Returns true if the environment is in a terminal position.
    fn is_terminal(&self) -> bool;

    /// Returns the identity of the current agent
    fn turn(&self) -> AgentId;

    /// Returns the winner of a final game
    fn winner(&self) -> Option<AgentId>;
}

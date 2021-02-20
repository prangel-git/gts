use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Environment;

pub fn mcts<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    cache: &mut HashMap<T, (u32, u32)>,
) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    todo!();
}

use super::abstract_game::Environment;


fn minmax_search<Action, AgentId, T: Environment<Action, AgentId> + Copy + Clone>(
    env: &T,
    agent: &AgentId,
    reward: impl Fn(&T, &AgentId) -> f64,
    depth: u8
) -> f64 {
    if env.is_terminal() | (depth == 0) {
        return reward(env, agent);
    } else {
        let mut new_env;
        let new_depth = depth - 1;

        let actions = env.valid_actions(agent);
        
        let mut best_score = f64::NEG_INFINITY;
        let mut current_score;

        for action in actions {
            new_env = env.clone();
            new_env.update(agent, &action);

            current_score = minmax_search(&new_env, agent, &reward, new_depth);

            if current_score > best_score {
                best_score = current_score;
            } else {};
        }
        
        return best_score;
    }
}
use super::agent::Agent;
use super::environment::Environment;

// TODO: Figure out how to play with a list of players.
// Note that players can have different types as far as they
// implement the Agent trait.

/// Plays a game in Envirnment 'env', and two agents.
pub fn play<Action, AgentId, T, R, S>(
    env: &mut T,
    agent_1: &mut R,
    agent_2: &mut S,
) -> Vec<(AgentId, Action)>
where
    AgentId: Eq,
    T: Environment<Action, AgentId>,
    R: Agent<Action, AgentId, T>,
    S: Agent<Action, AgentId, T>,
{
    let mut game_log = Vec::new();

    while !env.is_terminal() {
        let identity = agent_1.identity();
        if identity == env.turn() {
            let action = agent_1.action(env);
            match action {
                Some(a) => {
                    env.update(&a);
                    game_log.push((identity, a));

                    if env.is_terminal() {
                        break;
                    }
                }
                None => {}
            }
        }

        let identity = agent_2.identity();
        if identity == env.turn() {
            let action = agent_2.action(env);
            match action {
                Some(a) => {
                    env.update(&a);
                    game_log.push((identity, a));

                    if env.is_terminal() {
                        break;
                    }
                }
                None => {}
            }
        }
    }

    return game_log;
}

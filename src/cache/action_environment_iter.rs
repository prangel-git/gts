use std::rc::Rc;

use crate::abstractions::Environment;


/// Struct for keeping an interator with pairs of actions, and environments.
pub struct ActionEnvironmentIter<Action, AgentId, T>
where
T: Environment<Action, AgentId>
{
    environment: Rc<T>,
    to_visit: Box<T::ActionIter>,
    visited: Vec<(Action, Rc<T>)>,
    index: usize,
}

impl<Action, AgentId, T> ActionEnvironmentIter<Action, AgentId, T> 
where 
T: Environment<Action, AgentId>
{
    pub fn new(env: &Rc<T>) -> Self {
        ActionEnvironmentIter{
            environment: env.clone(),
            to_visit: Box::new(env.valid_actions()),
            visited: Vec::new(),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0
    }
}

impl<Action, AgentId, T> Iterator for ActionEnvironmentIter<Action, AgentId, T> 
where
Action: Copy,
T: Environment<Action, AgentId>
{
    type Item = (Action, Rc<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.visited.len() <= self.index {
            match self.to_visit.next() {
                None => { None },
                Some(a) => {
                    let (action, env) = (a, Rc::new(self.environment.what_if(&a)));
                    
                    self.visited.push((action, Rc::clone(&env)));
                    self.index += 1;

                    Some((action, env.clone()))
                }
            }
        } else {
            let (action, env) = &self.visited[self.index];
            self.index += 1;
            Some((*action, env.clone()))
        }
    }
}
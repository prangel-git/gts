use std::rc::Rc;

use crate::abstractions::Environment;

pub struct Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId>,
{
    env: Rc<T>,
    turn: AgentId,
    visited: Vec<(Rc<T>, Action)>,
    to_visit: Box<dyn Iterator<Item = Action>>,
    index: usize,
    pub data: D,
}

impl<T, Action, AgentId, D> Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId>,
{
    pub fn new(env: &Rc<T>, data: D) -> Self {
        Node {
            env: env.clone(),
            turn: env.turn(),
            visited: Vec::new(),
            to_visit: env.valid_actions(),
            index: 0,
            data,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn environment(&self) -> &Rc<T> {
        &self.env
    }

    pub fn turn(&self) -> &AgentId {
        &self.turn
    }
}

impl<T, Action, AgentId, D> Iterator for Node<T, Action, AgentId, D>
where
    Action: Copy,
    T: Environment<Action, AgentId>,
{
    type Item = (Rc<T>, Action);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.visited.len() {
            match self.to_visit.next() {
                Some(a) => {
                    let output = (Rc::new(self.env.what_if(&a)), a);
                    self.visited.push(output.clone());
                    self.index += 1;
                    Some(output)
                }
                None => None,
            }
        } else {
            let index = self.index;
            self.index += 1;
            Some(self.visited[index].clone())
        }
    }
}

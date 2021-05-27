use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::abstractions::Environment;

use super::minmax_data::MinMaxData;

pub type NodeRcRefCell<T, Action, AgentId> =
    Rc<RefCell<Node<T, Action, AgentId, MinMaxData<Action>>>>;
pub type Cache<T, Action, AgentId> = HashMap<Rc<T>, NodeRcRefCell<T, Action, AgentId>>;

pub struct Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId>,
    D: Default,
{
    env: Rc<T>,
    turn: AgentId,
    visited: Vec<(Rc<RefCell<Self>>, Action)>,
    to_visit: Box<dyn Iterator<Item = Action>>,
    index: usize,
    pub data: D,
}

impl<T, Action, AgentId, D> Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId>,
    D: Default,
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
    D: Default,
{
    type Item = (Rc<RefCell<Self>>, Action);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.visited.len() {
            match self.to_visit.next() {
                Some(a) => {
                    let env_next = Rc::new(self.env.what_if(&a));
                    let node_next = Rc::new(RefCell::new(Self::new(&env_next, D::default())));
                    let output = (node_next, a);
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

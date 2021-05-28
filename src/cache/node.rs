use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::abstractions::Environment;

use super::minmax_data::MinMaxData;
use super::utils::node_partial_cmp;

pub type NodeRRMM<T, Action, AgentId> =
    NodeRR<T, Action, AgentId, MinMaxData<Action>>;
pub type CacheMM<T, Action, AgentId> = Cache<T, Action, AgentId, MinMaxData<Action>>;

type NodeRR<T, Action, AgentId, D> = Rc<RefCell<Node<T, Action, AgentId, D>>>;

type Cache<T, Action, AgentId, D> = HashMap<Rc<T>, NodeRR<T, Action, AgentId, D>>;
type CacheRR<T, Action, AgentId, D> = Rc<RefCell<Cache<T, Action, AgentId, D>>>;


pub struct Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId>,
{
    env: Rc<T>,
    turn: AgentId,
    visited: Vec<(Rc<RefCell<Self>>, Action)>,
    to_visit: Box<dyn Iterator<Item = Action>>,
    index: usize,
    pub data: D,
    pub cache_ptr: CacheRR<T, Action, AgentId, D>,
}

impl<T, Action, AgentId, D> Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId>,
    D: Default,
{
    pub fn new(env: &Rc<T>) -> Self {
        Node {
            env: env.clone(),
            turn: env.turn(),
            visited: Vec::new(),
            to_visit: env.valid_actions(),
            index: 0,
            data: D::default(),
            cache_ptr: Rc::new(RefCell::new(Cache::new())),
        }
    }

    pub fn with_cache(env: &Rc<T>, data: D, cache_ptr: CacheRR<T, Action, AgentId, D>) -> Self {
        Node {
            env: env.clone(),
            turn: env.turn(),
            visited: Vec::new(),
            to_visit: env.valid_actions(),
            index: 0,
            data,
            cache_ptr,
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
                    let node_next = Rc::new(RefCell::new(Self::new(&env_next)));
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

impl<T, Action, AgentId> Node<T, Action, AgentId, MinMaxData<Action>>
where
    T: Environment<Action, AgentId>,
{
    pub fn sort_children(&mut self) {
        let is_maximizer = self.data.is_maximizer;
        if is_maximizer {
            self.visited
                .sort_by(|(a, _), (b, _)| node_partial_cmp(b, a))
        } else {
            self.visited
                .sort_by(|(a, _), (b, _)| node_partial_cmp(a, b))
        }
    }
}

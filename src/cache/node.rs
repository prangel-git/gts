use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Environment;

use super::minmax_data::MinMaxData;
use super::utils::get_or_insert;
use super::utils::node_partial_cmp;

pub type NodeRRMM<T, Action, AgentId> = NodeRR<T, Action, AgentId, MinMaxData<Action>>;
pub type CacheMM<T, Action, AgentId> = Cache<T, Action, AgentId, MinMaxData<Action>>;

pub(super) type NodeRR<T, Action, AgentId, D> = Rc<RefCell<Node<T, Action, AgentId, D>>>;

type Cache<T, Action, AgentId, D> = HashMap<Rc<T>, NodeRR<T, Action, AgentId, D>>;
pub(super) type CacheRR<T, Action, AgentId, D> = Rc<RefCell<Cache<T, Action, AgentId, D>>>;

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
    cache_ptr: CacheRR<T, Action, AgentId, D>,
}

impl<T, Action, AgentId, D> Node<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId> + Hash + Eq,
    D: Default,
{
    pub fn new(env: &Rc<T>) -> Self {
        let node = Node {
            env: env.clone(),
            turn: env.turn(),
            visited: Vec::new(),
            to_visit: env.valid_actions(),
            index: 0,
            data: D::default(),
            cache_ptr: Rc::new(RefCell::new(Cache::new())),
        };

        get_or_insert(&env, node.cache_ptr.clone());

        return node;
    }

    pub fn with_cache(env: &Rc<T>, cache_ptr: CacheRR<T, Action, AgentId, D>) -> Self {
        Node {
            env: env.clone(),
            turn: env.turn(),
            visited: Vec::new(),
            to_visit: env.valid_actions(),
            index: 0,
            data: D::default(),
            cache_ptr,
        }
    }

    pub fn rebase_cache(&self) {
        let env = self.env.clone();
        let root = get_or_insert(&env, self.cache_ptr.clone());

        let mut cache_ptr = self.cache_ptr.borrow_mut();
        cache_ptr.clear();
        cache_ptr.insert(env, root.clone());

        self.add_descendants(&mut cache_ptr);
    }

    fn add_descendants(&self, cache: &mut Cache<T, Action, AgentId, D>) {
        for (child, _) in &self.visited {
            let child_ptr = child.borrow();
            cache.insert(child_ptr.env.clone(), child.clone());
            child.borrow().add_descendants(cache);
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

    pub fn cache_len(&self) -> usize {
        self.cache_ptr.borrow_mut().len()
    }

    pub fn cache_get(&self, env: &T) -> Option<Rc<RefCell<Node<T, Action, AgentId, D>>>> {
        match self.cache_ptr.borrow().get(env) {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }
}

impl<T, Action, AgentId, D> Iterator for Node<T, Action, AgentId, D>
where
    Action: Copy,
    T: Environment<Action, AgentId> + Hash + Eq,
    D: Default,
{
    type Item = (Rc<RefCell<Self>>, Action);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.visited.len() {
            match self.to_visit.next() {
                Some(a) => {
                    let env_next = Rc::new(self.env.what_if(&a));
                    let node_next_ptr = get_or_insert(&env_next, self.cache_ptr.clone());
                    let output = (node_next_ptr, a);
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

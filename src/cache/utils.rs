use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Environment;

use super::node::CacheRR;
use super::node::Node;
use super::node::NodeRR;
use super::node::NodeRRMM;

/// Gets or insert a node into the cache
pub fn get_or_insert<T, Action, AgentId, D>(
    cache_ptr: CacheRR<T, Action, AgentId, D>,
    key: &Rc<T>,
) -> NodeRR<T, Action, AgentId, D>
where
    T: Environment<Action, AgentId> + Eq + Hash,
    D: Default,
{
    let mut cache = cache_ptr.borrow_mut();
    let output = cache
        .entry(key.clone())
        .or_insert(Rc::new(RefCell::new(Node::with_cache(&key, cache_ptr.clone()))));
    output.clone()
}

// Partial comparison for two nodes.
pub fn node_partial_cmp<T, Action, AgentId>(
    lhs: &NodeRRMM<T, Action, AgentId>,
    rhs: &NodeRRMM<T, Action, AgentId>,
) -> std::cmp::Ordering
where
    T: Environment<Action, AgentId>,
{
    let lhs_value = lhs.borrow().data.value;
    let rhs_value = rhs.borrow().data.value;

    lhs_value
        .partial_cmp(&rhs_value)
        .unwrap_or_else(|| panic!("Comparing to NaN"))
}

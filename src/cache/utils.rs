use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Environment;

use super::minmax_data::MinMaxData;

use super::node::Cache;
use super::node::Node;
use super::node::NodeRcRefCell;

/// Gets or insert a node into the cache
pub fn get_or_insert<T, Action, AgentId>(
    cache: &mut Cache<T, Action, AgentId>,
    key: &Rc<T>,
) -> NodeRcRefCell<T, Action, AgentId>
where
    T: Environment<Action, AgentId> + Eq + Hash,
{
    let output = cache
        .entry(key.clone())
        .or_insert(Rc::new(RefCell::new(Node::new(
            &key,
            MinMaxData::default(),
        ))));
    output.clone()
}

// Partial comparison for two nodes.
pub fn node_partial_cmp<T, Action, AgentId>(
    lhs: &NodeRcRefCell<T, Action, AgentId>,
    rhs: &NodeRcRefCell<T, Action, AgentId>,
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

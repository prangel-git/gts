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
    is_maximizer: bool,
) -> NodeRcRefCell<T, Action, AgentId>
where
    T: Environment<Action, AgentId> + Eq + Hash,
{
    let output = cache
        .entry(key.clone())
        .or_insert(Rc::new(RefCell::new(Node::new(
            &key,
            MinMaxData::new(is_maximizer),
        ))));
    output.clone()
}

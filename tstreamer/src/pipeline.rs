mod node;
pub(crate) use node::Node;

mod edge;
pub(crate) use edge::Edge;

mod state;
use serde::{Deserialize, Serialize};
pub use state::State;

/// A `Pipeline` of [`Node`]s and [`Edge`]s connecting them.
#[derive(Serialize, Deserialize)]
pub struct Pipeline<S: State> {
    graph: petgraph::graph::Graph<Node<S>, Edge<S>>,
}

use crate::element::Element;

use super::State;

/// [`Config`] for a [`Node`] specifying the type of element and it's
/// configuration in the form of a JSON object.
pub struct Config {
    element: crate::element::any::Kind,
    config: serde_json::Value,
}

/// A `Node` in a [`Pipeline`]. Wraps an [`Element`] and its children.
pub(crate) struct Node<S: State> {
    config: serde_json::Value,
    element: Box<dyn Element>,
    state: S,
}

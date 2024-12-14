use super::State;

pub struct Edge<S: State> {
    state: std::marker::PhantomData<S>,
}

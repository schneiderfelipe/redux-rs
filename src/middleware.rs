use crate::Store;

pub trait Middleware<State, Action> {
    fn next(&self, store: &Store<State, Action>, action: Action) -> Option<Action>;
}

/// Function signature for a middleware.
///
/// Middleware provides the possibility to intercept actions dispatched before they reach the reducer.
///
/// It receives a mutable reference to the store and the action currently dispatching.
/// The return type is an `Option` to indicate whether or not to proceed in the dispatching chain.
/// `Some(Action)` indicates to proceed with the specified action (might be changed to trigger further changes), `None` halts the complete chain, including the reducer and subscriptions.
///
/// # Example
///
/// The following will decrement before incrementing, never actually incrementing.
///
/// ```
/// # use redux_rs::{Store, Middleware};
/// #
/// type State = i8;
///
/// enum Action {
///     Increment,
///     Decrement
/// }
///
/// fn shall_not_increment_middleware(store: &Store<State, Action>, action: Action) -> Option<Action> {
///     match action {
///         Action::Increment => Some(Action::Decrement),
///         Action::Decrement => None
///     }
/// }
///
/// fn reducer(state: &State, action: Action) -> State {
///     match action {
///         Action::Increment => state + 1,
///         Action::Decrement => state - 1
///     }
/// }
///
/// let mut store = Store::new(reducer, 0);
/// store.add_middleware(shall_not_increment_middleware);
/// ```
impl<State, Action, Function> Middleware<State, Action> for Function
where
    Function: Fn(&Store<State, Action>, Action) -> Option<Action>,
{
    fn next(&self, store: &Store<State, Action>, action: Action) -> Option<Action> {
        self(store, action)
    }
}

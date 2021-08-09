// Idea from <https://www.reddit.com/r/rust/comments/5bn5pn/would_love_feedback_on_my_new_library_reduxrs_a/d9pyafm?utm_source=share&utm_medium=web2x&context=3>.
pub trait Reducible<State, Action> {
    fn reduce(&self, state: &State, action: &Action) -> State;
}

/// Function signature for a reducer.
///
/// # Example
///
/// ```
/// # use redux_rs::Reducible;
/// #
/// enum Action {
///     Increment,
///     Decrement
/// }
///
/// let reducer = |state: &u8, action: &Action| -> u8 {
///     match action {
///         Action::Increment => state + 1,
///         Action::Decrement => state - 1
///     }
/// };
///
/// assert_eq!(reducer(&0, &Action::Increment), reducer.reduce(&0, &Action::Increment));
/// ```
impl<State, Action, Function> Reducible<State, Action> for Function
where
    Function: Fn(&State, &Action) -> State,
{
    fn reduce(&self, state: &State, action: &Action) -> State {
        self(state, action)
    }
}

#[macro_export]
/// Combines multiple reducers into a single one.
///
/// The first one gets called first, chained into the second one and so on...
///
/// # Usage
///
/// ```
/// # use redux_rs::{combine_reducers, Reducible};
/// #
/// # type State = u8;
/// #
/// # type Action = bool;
/// #
/// # fn first_reducer(_: &State, _: &Action) -> State {
/// #     0
/// # }
/// #
/// # fn second_reducer(_: &State, _: &Action) -> State {
/// #     0
/// # }
/// #
/// # fn third_reducer(_: &State, _: &Action) -> State {
/// #     0
/// # }
/// #
/// let reducer = combine_reducers!(State, &Action, first_reducer, second_reducer, third_reducer);
/// ```
/// (`State` and `Action` being the actual types.)
///
/// # Example
///
/// ```
/// # use redux_rs::{combine_reducers, Reducible};
/// #
/// enum Action {
///     Increment,
///     Decrement
/// }
///
/// fn counter_reducer(state: &u8, action: &Action) -> u8 {
///     match action {
///         Action::Increment => state + 1,
///         Action::Decrement => state - 1
///     }
/// }
///
/// fn add_two_reducer(state: &u8, _: &Action) -> u8 {
///     state + 2
/// }
///
/// fn main() {
///     let reducer = combine_reducers!(u8, &Action, counter_reducer, add_two_reducer);
/// }
/// ```
macro_rules! combine_reducers {
    ($state: ty, $action: ty, $reducer: ident) => ($reducer);
    ($state: ty, $action: ty, $first: ident, $($second: ident),+) => (
        |state: &$state, action: $action| -> $state {
            (combine_reducers!($state, $action, $($second),+)).reduce(&$first(state, action), action)
        }
    )
}

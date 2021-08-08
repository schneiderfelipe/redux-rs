use crate::{Middleware, Subscription, Vec};

/// A container holding a state and providing the possibility to dispatch actions.
///
/// A store is defined by the state is holds and the actions it can dispatch.
pub struct Store<State, Action> {
    reducer: StoreReducer<State, Action>,
    state: State,
    middleware: Vec<Box<dyn Middleware<State, Action>>>,
    subscriptions: Vec<Box<dyn Subscription<State>>>,
}

// TODO: should be part of a trait
pub type StoreReducer<State, Action> = fn(&State, &Action) -> State;

impl<State, Action> Store<State, Action> {
    /// Creates a new store.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// type State = i8;
    ///
    /// enum Action {
    ///     Increment,
    ///     Decrement
    /// }
    ///
    /// fn reducer(state: &State, action: &Action) -> State {
    ///     match action {
    ///         Action::Increment => state + 1,
    ///         Action::Decrement => state - 1
    ///     }
    /// }
    ///
    /// let mut store = Store::new(reducer, 0);
    /// ```
    pub fn new(reducer: StoreReducer<State, Action>, initial_state: State) -> Self {
        Self {
            reducer,
            state: initial_state,
            middleware: Vec::new(),
            subscriptions: Vec::new(),
        }
    }

    /// Returns the current state.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// # let store = Store::new(|&u8, ()| 0, 0);
    /// #
    /// println!("Current state: {}", store.state());
    /// ```
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Dispatches an action which is handles by the reducer, after the store got passed through the middleware.
    /// This can modify the state within the store.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// # type State = i8;
    /// #
    /// enum Action {
    ///     DoSomething,
    ///     DoSomethingElse
    /// }
    ///
    /// // ...
    ///
    /// # fn reducer(state: &u8, action: &Action) -> u8 {
    /// #     0
    /// # }
    /// #
    /// # let mut store = Store::new(reducer, 0);
    /// #
    /// store.dispatch(Action::DoSomething);
    /// println!("Current state: {}", store.state());
    /// ```
    pub fn dispatch(&mut self, action: Action) {
        if self.middleware.is_empty() {
            self.dispatch_reducer(&action);
        } else {
            self.dispatch_middleware(0, action);
        }
    }

    /// Runs one middleware.
    fn dispatch_middleware(&mut self, index: usize, action: Action) {
        if index == self.middleware.len() {
            self.dispatch_reducer(&action);
            return;
        }

        if let Some(action) = self.middleware[index](self, action) {
            self.dispatch_middleware(index + 1, action);
        }
    }

    /// Runs the reducer.
    fn dispatch_reducer(&mut self, action: &Action) {
        self.state = (&self.reducer)(self.state(), action);
        self.dispatch_subscriptions();
    }

    /// Runs all subscriptions.
    fn dispatch_subscriptions(&self) {
        self.subscriptions
            .iter()
            .for_each(|subscription| subscription(self.state()));
    }

    /// Subscribes a callback to any change of the state.
    ///
    /// Subscriptions will be called, whenever an action is dispatched.
    ///
    /// See [`Subscription`](type.Subscription.html).
    ///
    /// # Example
    ///
    /// ```
    /// use redux_rs::{Store, Subscription};
    /// #
    /// # type State = u8;
    /// # let initial_state = 0;
    /// #
    /// # fn reducer(_: &State, action: &bool) -> State {
    /// #     0
    /// # }
    ///
    /// let mut store = Store::new(reducer, initial_state);
    ///
    /// let listener = |state: &State| {
    ///     println!("Something changed! New value: {}", state);
    /// };
    ///
    /// store.subscribe(listener);
    /// ```
    pub fn subscribe<S: Subscription<State> + 'static>(&mut self, callback: S) {
        self.subscriptions.push(Box::new(callback));
    }

    /// Adds a custom middleware to the store.
    ///
    /// Middleware provides the possibility to intercept actions dispatched before they reach the reducer.
    ///
    /// See [`Middleware`](type.Middleware.html).
    pub fn add_middleware<M: Middleware<State, Action> + 'static>(&mut self, middleware: M) {
        self.middleware.push(Box::new(middleware));
    }

    /// Replaces the currently used reducer.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// # pub struct State(u8);
    /// #
    /// # impl State {
    /// #     pub fn something_else() -> State {
    /// #         State(1)
    /// #     }
    /// # }
    /// #
    /// # enum Action {
    /// #     SomeAction
    /// # }
    /// #
    /// # fn reducer(state: &State, action: &Action) -> State {
    /// #     State(0)
    /// # }
    /// #
    /// # let mut store = Store::new(reducer, State(0));
    /// #
    /// store.dispatch(Action::SomeAction);
    ///
    /// store.replace_reducer(|state: &State, action: &Action| {
    ///     State::something_else()
    /// });
    ///
    /// store.dispatch(Action::SomeAction);
    /// ```
    pub fn replace_reducer(&mut self, reducer: StoreReducer<State, Action>) {
        self.reducer = reducer;
    }
}

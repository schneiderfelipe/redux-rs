// Inspired by <https://github.com/jaredonline/redux-rs/blob/master/examples/simple.rs>.

use redux_rs::Store;
use std::default::Default;

#[derive(Clone, Debug)]
struct Todo {
    name: &'static str,
}

#[derive(Clone, Debug)]
struct TodoState {
    todos: Vec<Todo>,
}

impl TodoState {
    fn new() -> TodoState {
        TodoState { todos: vec![] }
    }
}

enum TodoAction {
    Insert(&'static str),
}

impl Default for TodoState {
    fn default() -> Self {
        TodoState::new()
    }
}

fn main() {
    let mut store = Store::new(
        // Our reducer.
        |state: &TodoState, action: TodoAction| {
            // TODO: we could benefit from modifying the state in place here, but
            // in order to not lose the benefits of immutability, we can just take
            // ownership of the state.
            let mut todos = state.todos.clone();
            match action {
                TodoAction::Insert(name) => {
                    let todo = Todo { name: name };
                    todos.push(todo);
                }
            };

            TodoState { todos }
        },
        // Our initial state.
        TodoState::new(),
    );
    store.dispatch(TodoAction::Insert("Clean the bathroom"));

    println!("{:?}", store.state());
}

mod env;

use candid::CandidType;
use env::{CanisterEnv, EmptyEnv, Environment};
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

type TimestampMillis = u64;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

struct RuntimeState {
    env: Box<dyn Environment>,
    data: Data
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnv {}),
            data: Data::default(),
        }
    }
}

#[derive(CandidType, Default, Deserialize)]
struct Data {
    todos: Vec<TodoItem>
}

#[derive(CandidType, Clone, Deserialize)]
struct TodoItem {
    id: u32,
    content: String,
    done: bool,
    created_at: TimestampMillis,
}

#[init]
fn init() {
    let env = Box::new(CanisterEnv::new());
    let data = Data::default();
    let runtime_state = RuntimeState { env, data };

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}

#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state| ic_cdk::storage::stable_save((&state.borrow().data,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (data,): (Data,) = ic_cdk::storage::stable_restore().unwrap();
    let env = Box::new(CanisterEnv::new());
    let runtime_state = RuntimeState { env, data };

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}

#[update]
fn add(content: String) -> u32 {
    RUNTIME_STATE.with(|state| add_impl(content, &mut state.borrow_mut()))
}

fn add_impl(content: String, runtime_state: &mut RuntimeState) -> u32 {
    let id = (runtime_state.data.todos.len() as u32) + 1;
    let item = TodoItem { id, content, done: false, created_at: runtime_state.env.now() };
    runtime_state.data.todos.push(item);
    id
}

#[query]
fn get() -> Vec<TodoItem> {
    RUNTIME_STATE.with(|state| get_impl(&state.borrow())) 
}

fn get_impl(runtime_state: &RuntimeState) -> Vec<TodoItem> {
    runtime_state.data.todos.clone()
}

#[cfg(test)]
mod tests {
    use env::TestEnv;

    use super::*;

    #[test]
    fn add_then_get() {
        let mut runtime_state = RuntimeState {
            env: Box::new(TestEnv { now: 1 }),
            data: Data::default(),
        };

        let content = "Hello world".to_string();
        add_impl(content.clone(), &mut runtime_state);
        let results = get_impl(&runtime_state);
        assert_eq!(results.len(), 1);

        let result = results.first().unwrap();
        assert_eq!(result.content, content);
        assert_eq!(result.done, false);
        assert_eq!(result.created_at, 1);
    }
}

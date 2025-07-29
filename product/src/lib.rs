mod bindings;
mod domain;

use crate::bindings::exports::golem::api::{load_snapshot, save_snapshot};
use crate::bindings::exports::golem::product_exports::api::*;
use std::cell::RefCell;
use std::env;

struct Component;

thread_local! {
    static STATE: RefCell<Option<domain::product::Product>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut domain::product::Product) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let product = domain::product::Product::new(worker_name);
            *state = Some(product);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn initialize_product(name: String, brand: String, description: String, tags: Vec<String>) -> () {
        with_state(|state| {
            println!("Initializing product {}", state.product_id);
            state.update(name, brand, description, tags);
        });
    }

    fn get() -> Option<Product> {
        STATE.with_borrow(|state| {
            println!("Getting product");

            state.clone().map(|state| state.into())
        })
    }
}
impl save_snapshot::Guest for Component {
    fn save() -> Vec<u8> {
        with_state(|state| {
            domain::product::serdes::serialize(state).expect("Failed to serialize state")
        })
    }
}

impl load_snapshot::Guest for Component {
    fn load(bytes: Vec<u8>) -> Result<(), String> {
        with_state(|state| {
            let value = domain::product::serdes::deserialize(&bytes)?;
            if value.product_id != state.product_id {
                Err("Invalid state".to_string())
            } else {
                state.clone_from(&value);
                Ok(())
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);

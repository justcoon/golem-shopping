mod bindings;

use crate::bindings::exports::golem::product::api::*;
use std::cell::RefCell;
use std::env;

use rand::prelude::*;

struct Component;

thread_local! {
    static STATE: RefCell<Option<Product>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut Product) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("WORKER_NAME").expect("WORKER_NAME must be set");
            let product = Product {
                product_id: worker_name,
                name: "undefined".to_string(),
                description: "undefined".to_string(),
            };
            *state = Some(product);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn initialize_product(name: String, description: String) -> () {
        with_state(|state| {
            println!("Initializing product {}", state.product_id);

            state.name = name;
            state.description = description;
        });
    }

    fn get() -> Option<Product> {
        STATE.with_borrow(|state| {
            println!("Getting product");

            state.clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);

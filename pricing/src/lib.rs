mod bindings;

use crate::bindings::exports::golem::shopping_pricing::api::*;
use crate::bindings::golem::api::host::*;
use std::cell::RefCell;
use std::env;

use rand::prelude::*;

struct Component;

thread_local! {
    static STATE: RefCell<Option<Pricing>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut Pricing) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("WORKER_NAME").expect("WORKER_NAME must be set");
            let value = Pricing { asset_id: worker_name, msrp_prices: vec![], list_prices: vec![] };
            *state = Some(value);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn initialize_pricing(msrp_prices: Vec<PricingItem>, list_prices: Vec<PricingItem>) -> () {
        with_state(|state| {
            println!("Initializing pricing {}", state.asset_id);
            state.msrp_prices = msrp_prices;
            state.list_prices = list_prices;
        });
    }

    // fn update_pricing(msrp_prices: Vec<PricingItem>, list_prices: Vec<PricingItem>) {
    //     with_state(|state| {
    //         println!("Update pricing {}", state.asset_id);
    //         state.msrp_prices = msrp_prices;
    //         state.list_prices = list_prices;
    //     });
    // }

    fn get() -> Option<Pricing> {
        STATE.with_borrow(|state| {
            println!("Getting pricing");

            state.clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);

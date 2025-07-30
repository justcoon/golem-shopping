mod bindings;
mod domain;
use crate::bindings::exports::golem::api::{load_snapshot, save_snapshot};
use crate::bindings::exports::golem::pricing_exports::api::*;
use std::cell::RefCell;

use std::env;

struct Component;

thread_local! {
    static STATE: RefCell<Option<domain::pricing::Pricing>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut domain::pricing::Pricing) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let value = domain::pricing::Pricing::new(worker_name);
            *state = Some(value);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn initialize_pricing(
        msrp_prices: Vec<PricingItem>,
        list_prices: Vec<PricingItem>,
        sale_prices: Vec<SalePricingItem>,
    ) -> () {
        with_state(|state| {
            println!("Initializing pricing {}", state.product_id);
            state.set_prices(
                msrp_prices.into_iter().map(|item| item.into()).collect(),
                list_prices.into_iter().map(|item| item.into()).collect(),
                sale_prices.into_iter().map(|item| item.into()).collect(),
            );
        });
    }

    fn update_pricing(
        msrp_prices: Vec<PricingItem>,
        list_prices: Vec<PricingItem>,
        sale_prices: Vec<SalePricingItem>,
    ) -> () {
        with_state(|state| {
            println!("Update pricing {}", state.product_id);
            state.update_prices(
                msrp_prices.into_iter().map(|item| item.into()).collect(),
                list_prices.into_iter().map(|item| item.into()).collect(),
                sale_prices.into_iter().map(|item| item.into()).collect(),
            );
        });
    }

    fn get_price(currency: String, zone: String) -> Option<PricingItem> {
        STATE.with_borrow(|state| {
            println!("Getting pricing for currency: {} zone: {}", currency, zone);
            state
                .clone()
                .and_then(|pricing| pricing.get_price(currency, zone).map(|item| item.into()))
        })
    }

    fn get() -> Option<Pricing> {
        STATE.with_borrow(|state| {
            println!("Getting pricing");

            state.clone().map(|state| state.into())
        })
    }
}

impl save_snapshot::Guest for Component {
    fn save() -> Vec<u8> {
        with_state(|state| {
            domain::pricing::serdes::serialize(state).expect("Failed to serialize state")
        })
    }
}

impl load_snapshot::Guest for Component {
    fn load(bytes: Vec<u8>) -> Result<(), String> {
        with_state(|state| {
            let value = domain::pricing::serdes::deserialize(&bytes)?;
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

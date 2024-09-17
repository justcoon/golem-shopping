mod bindings;

use crate::bindings::exports::golem::pricing::api::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;

struct Component;

thread_local! {
    static STATE: RefCell<Option<Pricing>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut Pricing) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let value = Pricing { asset_id: worker_name, msrp_prices: vec![], list_prices: vec![] };
            *state = Some(value);
        }

        f(state.as_mut().unwrap())
    })
}

fn get_price(currency: String, zone: String, pricing: Pricing) -> Option<PricingItem> {
    let list_price =
        pricing.list_prices.into_iter().find(|x| x.zone == zone && x.currency == currency);

    if list_price.is_some() {
        list_price
    } else {
        pricing.msrp_prices.into_iter().find(|x| x.zone == zone && x.currency == currency)
    }
}

fn merge_items(updates: Vec<PricingItem>, current: Vec<PricingItem>) -> Vec<PricingItem> {
    if updates.is_empty() {
        current
    } else if current.is_empty() {
        updates
    } else {
        let mut merge_map: HashMap<(String, String), PricingItem> = HashMap::new();

        for item in updates {
            merge_map.insert((item.zone.clone(), item.currency.clone()), item);
        }

        for item in current {
            if !merge_map.contains_key(&(item.zone.clone(), item.currency.clone())) {
                merge_map.insert((item.zone.clone(), item.currency.clone()), item);
            }
        }

        merge_map.into_values().collect()
    }
}

impl Guest for Component {
    fn initialize_pricing(msrp_prices: Vec<PricingItem>, list_prices: Vec<PricingItem>) -> () {
        with_state(|state| {
            println!("Initializing pricing {}", state.asset_id);
            state.msrp_prices = msrp_prices;
            state.list_prices = list_prices;
        });
    }

    fn update_pricing(msrp_prices: Vec<PricingItem>, list_prices: Vec<PricingItem>) -> () {
        with_state(|state| {
            println!("Update pricing {}", state.asset_id);
            state.msrp_prices = merge_items(msrp_prices, state.msrp_prices.clone());
            state.list_prices = merge_items(list_prices, state.list_prices.clone());
        });
    }

    fn get_price(currency: String, zone: String) -> Option<PricingItem> {
        STATE.with_borrow(|state| {
            println!("Getting pricing for currency: {} zone: {}", currency, zone);
            state.clone().and_then(|pricing| get_price(currency, zone, pricing))
        })
    }

    fn get() -> Option<Pricing> {
        STATE.with_borrow(|state| {
            println!("Getting pricing");

            state.clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);

mod bindings;

use crate::bindings::exports::golem::it::api::*;
use crate::bindings::golem::api::host::*;
use std::cell::RefCell;
use std::env;

use rand::prelude::*;

struct Component;

fn reserve_inventory() -> Result<(), &'static str> {
    // generate a random float 32:
    let mut rng = rand::thread_rng();
    let random_float: f32 = rng.gen();

    // Reserve inventory for the items in the cart.
    // If the inventory is not available, return an error.
    // Otherwise, return a success result.
    if random_float < 0.1 {
        return Err("Inventory not available");
    } else {
        Ok(())
    }
}

#[allow(unused)]
fn release_inventory() -> Result<(), &'static str> {
    // Release inventory for the items in the cart.
    // If the inventory is not available, return an error.
    // Otherwise, return a success result.
    Ok(())
}

fn charge_credit_card() -> Result<(), &'static str> {
    // Charge the user's credit card for the items in the cart.
    // If the charge fails, return an error.
    // Otherwise, return a success result.
    Ok(())
}

fn generate_order() -> String {
    // Save the order to the database.
    // Return the order ID.
    "238738674".to_string()
}

fn dispatch_order() -> Result<(), &'static str> {
    // Dispatch the order to the warehouse.
    // If the order cannot be dispatched, return an error.
    // Otherwise, return a success result.
    Ok(())
}

fn get_total_price(items: Vec<OrderItem>) -> f32 {
    let mut total = 0f32;

    for item in items {
        total += item.price * item.quantity as f32;
    }

    total
}

thread_local! {
    static STATE: RefCell<Option<Order>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut Order) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("WORKER_NAME").expect("WORKER_NAME must be set");
            let value = Order {
                order_id: worker_name,
                user_id: "".to_string(),
                items: vec![],
                total: 0f32,
                currency: "USD".to_string(),
                timestamp: 0,
            };
            *state = Some(value);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn initialize_order(data: CreateOrder) {
        with_state(|state| {
            println!("Initializing order for user {}", data.user_id);
            state.user_id = data.user_id;
            state.currency = data.currency;
            state.timestamp = data.timestamp;
            state.total = data.total;
            state.items = data.items;
        });
    }

    fn add_item(product_id: String, quantity: u32) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Adding item with product ID {} to the order of user {}",
                product_id, state.user_id
            );

            let mut updated = false;
            for item in &mut state.items {
                if item.product_id == product_id {
                    item.quantity += quantity;
                    updated = true;
                }
            }

            if !updated {
                state.items.push(OrderItem {
                    product_id,
                    quantity,
                    name: "undefined".to_string(),
                    price: 0f32,
                });
            }

            state.total = get_total_price(state.items.clone());

            Ok(())
        })
    }

    fn remove_item(product_id: String) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Removing item with product ID {} from the order of user {}",
                product_id, state.user_id
            );

            state.items.retain(|item| item.product_id != product_id);

            state.total = get_total_price(state.items.clone());

            Ok(())
        })
    }

    fn update_item_quantity(product_id: String, quantity: u32) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Updating quantity of item with product ID {} to {} in the order of user {}",
                product_id, quantity, state.user_id
            );

            for item in &mut state.items {
                if item.product_id == product_id {
                    item.quantity = quantity;
                }
            }
            state.total = get_total_price(state.items.clone());
            Ok(())
        })
    }

    fn get() -> Option<Order> {
        STATE.with_borrow(|state| {
            println!("Getting order");

            state.clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);

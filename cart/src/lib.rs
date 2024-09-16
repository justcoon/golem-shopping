mod bindings;

use crate::bindings::exports::golem::shopping_cart::api::*;
use crate::bindings::golem::api::host::*;
use std::cell::RefCell;
use std::env;

use rand::prelude::*;
use uuid::Uuid;

struct Component;

fn generate_order_id() -> String {
    // Save the order to the database.
    // Return the order ID.
    Uuid::new_v4().to_string()
}

fn dispatch_order() -> Result<(), &'static str> {
    // Dispatch the order to the warehouse.
    // If the order cannot be dispatched, return an error.
    // Otherwise, return a success result.
    Ok(())
}

fn get_total_price(items: Vec<CartItem>) -> f32 {
    let mut total = 0f32;

    for item in items {
        total += item.price * item.quantity as f32;
    }

    total
}

thread_local! {
    static STATE: RefCell<Option<Cart>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut Cart) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("WORKER_NAME").expect("WORKER_NAME must be set");
            let value = Cart {
                user_id: worker_name,
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
    fn add_item(product_id: String, quantity: u32) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Adding item with product ID {} to the cart of user {}",
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
                state.items.push(CartItem {
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
                "Removing item with product ID {} from the cart of user {}",
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
                "Updating quantity of item with product ID {} to {} in the cart of user {}",
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

    fn checkout() -> CheckoutResult {
        let result: Result<OrderConfirmation, &'static str> = with_state(|state| {
            let order_id = generate_order_id();

            dispatch_order()?;

            state.items.clear();
            state.total = 0f32;

            println!("Checkout for order {}", order_id);

            Ok(OrderConfirmation { order_id })
        });

        match result {
            Ok(OrderConfirmation { order_id }) => {
                CheckoutResult::Success(OrderConfirmation { order_id })
            }
            Err(err) => CheckoutResult::Error(err.to_string()),
        }
    }

    fn get() -> Option<Cart> {
        STATE.with_borrow(|state| {
            println!("Getting cart");

            state.clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);

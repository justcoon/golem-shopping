mod bindings;

use crate::bindings::exports::golem::order::api::*;
use std::cell::RefCell;
use std::env;

use crate::bindings::golem::pricing::api::PricingItem;
use crate::bindings::golem::product_stub::stub_product::Product;
use rand::prelude::*;

struct Component;

fn get_total_price(items: Vec<OrderItem>) -> f32 {
    let mut total = 0f32;

    for item in items {
        total += item.price * item.quantity as f32;
    }

    total
}

fn get_product_worker_urn(product_id: String) -> String {
    let component_id = std::env::var("PRODUCT_COMPONENT_ID").expect("PRODUCT_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/{}", product_id)
}

fn get_pricing_worker_urn(product_id: String) -> String {
    let component_id = std::env::var("PRICING_COMPONENT_ID").expect("PRICING_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/{}", product_id)
}

fn get_product(product_id: String) -> Option<Product> {
    println!("Getting product: {}", product_id);

    use bindings::golem::product_stub::stub_product::*;
    use bindings::golem::rpc::types::Uri;

    let api = Api::new(&Uri { value: get_product_worker_urn(product_id) });

    api.blocking_get()
}

fn get_pricing(product_id: String, currency: String, zone: String) -> Option<PricingItem> {
    println!("Getting pricing: {}, currency: {}, zone: {}", product_id, currency, zone);

    use bindings::golem::pricing_stub::stub_pricing::*;
    use bindings::golem::rpc::types::Uri;

    let api = Api::new(&Uri { value: get_pricing_worker_urn(product_id) });

    api.blocking_get_price(&currency, &zone)
}

thread_local! {
    static STATE: RefCell<Option<Order>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut Order) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let value = Order {
                order_id: worker_name,
                user_id: "".to_string(),
                order_status: OrderStatus::New,
                items: vec![],
                shipping_address: None,
                billing_address: None,
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
            println!("Initializing order {} for user {}", state.order_id, data.user_id);
            state.user_id = data.user_id;
            state.currency = data.currency;
            state.timestamp = data.timestamp;
            state.billing_address = data.billing_address;
            state.shipping_address = data.shipping_address;
            state.total = data.total;
            state.items = data.items;
        });
    }

    fn add_item(product_id: String, quantity: u32) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Adding item with product ID {} to the order {} of user {}",
                product_id, state.order_id, state.user_id
            );

            if state.order_status == OrderStatus::New {
                let mut updated = false;
                for item in &mut state.items {
                    if item.product_id == product_id {
                        item.quantity += quantity;
                        updated = true;
                    }
                }

                if !updated {
                    let product = get_product(product_id.clone());
                    let pricing = get_pricing(
                        product_id.clone(),
                        state.currency.clone(),
                        "global".to_string(),
                    );
                    match (product, pricing) {
                        (Some(product), Some(pricing)) => {
                            state.items.push(OrderItem {
                                product_id,
                                name: product.name,
                                price: pricing.price,
                                quantity,
                            });
                        },
                        (None, _) => {
                            return Err("Product not found".to_string());
                        },
                        _ => {
                            return Err("Pricing not found".to_string());
                        }
                    }
                }

                state.total = get_total_price(state.items.clone());

                Ok(())
            } else {
                Err(format!("Can not update order with status {:?}", state.order_status))
            }
        })
    }

    fn remove_item(product_id: String) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Removing item with product ID {} from the order {} of user {}",
                product_id, state.order_id, state.user_id
            );
            if state.order_status == OrderStatus::New {
                state.items.retain(|item| item.product_id != product_id);

                state.total = get_total_price(state.items.clone());

                Ok(())
            } else {
                Err(format!("Can not update order with status {:?}", state.order_status))
            }
        })
    }

    fn update_item_quantity(product_id: String, quantity: u32) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Updating quantity of item with product ID {} to {} in the order {} of user {}",
                product_id, quantity, state.order_id, state.user_id
            );
            if state.order_status == OrderStatus::New {
                for item in &mut state.items {
                    if item.product_id == product_id {
                        item.quantity = quantity;
                    }
                }

                state.total = get_total_price(state.items.clone());

                Ok(())
            } else {
                Err(format!("Can not update order with status {:?}", state.order_status))
            }
        })
    }

    fn cancel_order() -> Result<(), String> {
        with_state(|state| {
            println!("Cancelling order {} of user {}", state.order_id, state.user_id);

            if state.order_status == OrderStatus::New {
                println!("Cancelling order {} of user {}", state.order_id, state.user_id);
                state.order_status = OrderStatus::Cancelled;
                Ok(())
            } else {
                println!("Cancelling order {} of user {}", state.order_id, state.user_id);

                Err(format!("Can not cancel order with status {:?}", state.order_status))
            }
        })
    }

    fn ship_order() -> Result<(), String> {
        with_state(|state| {
            if state.order_status == OrderStatus::New {
                println!("Shipping order {} of user {}", state.order_id, state.user_id);
                state.order_status = OrderStatus::Shipped;
                Ok(())
            } else {
                println!("Shipping order {} of user {}", state.order_id, state.user_id);

                Err(format!("Can not cancel order with status {:?}", state.order_status))
            }
        })
    }

    fn update_billing_address(address: Address) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Updating billing address in the order {} of user {}",
                state.order_id, state.user_id
            );
            if state.order_status == OrderStatus::New {
                state.billing_address = Some(address);
                Ok(())
            } else {
                Err(format!("Can not update order with status {:?}", state.order_status))
            }
        })
    }

    fn update_shipping_address(address: Address) -> Result<(), String> {
        with_state(|state| {
            println!(
                "Updating shipping address in the order {} of user {}",
                state.order_id, state.user_id
            );
            if state.order_status == OrderStatus::New {
                state.shipping_address = Some(address);
                Ok(())
            } else {
                Err(format!("Can not update order with status {:?}", state.order_status))
            }
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

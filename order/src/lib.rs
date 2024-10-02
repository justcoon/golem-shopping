mod bindings;
mod domain;

use crate::bindings::exports::golem::api::{load_snapshot, save_snapshot};
use crate::bindings::exports::golem::order::api::*;
use crate::bindings::golem::pricing::api::PricingItem;
use crate::bindings::golem::product_stub::stub_product::Product;
use std::cell::RefCell;

struct Component;

fn get_worker_urn(component_id: String, worker_name: String) -> String {
    format!("urn:worker:{component_id}/{}", worker_name)
}

fn get_product_worker_urn(product_id: String) -> String {
    let component_id = std::env::var("PRODUCT_COMPONENT_ID").expect("PRODUCT_COMPONENT_ID not set");
    get_worker_urn(component_id, product_id)
}

fn get_pricing_worker_urn(product_id: String) -> String {
    let component_id = std::env::var("PRICING_COMPONENT_ID").expect("PRICING_COMPONENT_ID not set");
    get_worker_urn(component_id, product_id)
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

fn action_not_allowed_error(status: domain::order::OrderStatus) -> Error {
    Error::ActionNotAllowed(ActionNotAllowedError {
        message: "Can not update order with status".to_string(),
        status: status.into(),
    })
}

fn item_not_found_error(product_id: String) -> Error {
    Error::ItemNotFound(ItemNotFoundError { message: "Item not found".to_string(), product_id })
}

fn pricing_not_found_error(product_id: String) -> Error {
    Error::PricingNotFound(PricingNotFoundError {
        message: "Pricing not found".to_string(),
        product_id,
    })
}

fn product_not_found_error(product_id: String) -> Error {
    Error::ProductNotFound(ProductNotFoundError {
        message: "Product not found".to_string(),
        product_id,
    })
}

thread_local! {
    static STATE: RefCell<Option<domain::order::Order>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut domain::order::Order) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name =
                std::env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let value = domain::order::Order::new(worker_name, "".to_string());
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
            state.billing_address = data.billing_address.map(|v| v.into());
            state.shipping_address = data.shipping_address.map(|v| v.into());
            state.total = data.total;
            state.items = data.items.into_iter().map(|item| item.into()).collect();
        });
    }

    fn add_item(product_id: String, quantity: u32) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Adding item with product {} to the order {} of user {}",
                product_id, state.order_id, state.user_id
            );

            if state.order_status == domain::order::OrderStatus::New {
                let updated = state.update_item_quantity(product_id.clone(), quantity);

                if !updated {
                    let product = get_product(product_id.clone());
                    let pricing = get_pricing(
                        product_id.clone(),
                        state.currency.clone(),
                        domain::order::PRICING_ZONE_DEFAULT.to_string(),
                    );
                    match (product, pricing) {
                        (Some(product), Some(pricing)) => {
                            state.add_item(domain::order::OrderItem {
                                product_id,
                                name: product.name,
                                price: pricing.price,
                                quantity,
                            });
                        }
                        (None, _) => return Err(product_not_found_error(product_id)),
                        _ => return Err(pricing_not_found_error(product_id)),
                    }
                }

                Ok(())
            } else {
                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn remove_item(product_id: String) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Removing item with product {} from the order {} of user {}",
                product_id, state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                if state.remove_item(product_id.clone()) {
                    Ok(())
                } else {
                    Err(item_not_found_error(product_id))
                }
            } else {
                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn update_item_quantity(product_id: String, quantity: u32) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Updating quantity of item with product {} to {} in the order {} of user {}",
                product_id, quantity, state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                let updated = state.update_item_quantity(product_id.clone(), quantity);

                if updated {
                    Ok(())
                } else {
                    Err(item_not_found_error(product_id))
                }
            } else {
                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn cancel_order() -> Result<(), Error> {
        with_state(|state| {
            println!("Cancelling order {} of user {}", state.order_id, state.user_id);

            if state.order_status == domain::order::OrderStatus::New {
                println!("Cancelling order {} of user {}", state.order_id, state.user_id);
                state.order_status = domain::order::OrderStatus::Cancelled;
                Ok(())
            } else {
                println!("Cancelling order {} of user {}", state.order_id, state.user_id);

                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn ship_order() -> Result<(), Error> {
        with_state(|state| {
            if state.order_status == domain::order::OrderStatus::New {
                println!("Shipping order {} of user {}", state.order_id, state.user_id);
                state.order_status = domain::order::OrderStatus::Shipped;
                Ok(())
            } else {
                println!("Shipping order {} of user {}", state.order_id, state.user_id);
                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn update_billing_address(address: Address) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Updating billing address in the order {} of user {}",
                state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                state.billing_address = Some(address.into());
                Ok(())
            } else {
                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn update_shipping_address(address: Address) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Updating shipping address in the order {} of user {}",
                state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                state.shipping_address = Some(address.into());
                Ok(())
            } else {
                Err(action_not_allowed_error(state.order_status))
            }
        })
    }

    fn get() -> Option<Order> {
        STATE.with_borrow(|state| {
            println!("Getting order");

            state.clone().map(|state| state.into())
        })
    }
}

impl save_snapshot::Guest for Component {
    fn save() -> Vec<u8> {
        with_state(|state| {
            domain::order::serdes::serialize(state).expect("Failed to serialize state")
        })
    }
}

impl load_snapshot::Guest for Component {
    fn load(bytes: Vec<u8>) -> Result<(), String> {
        with_state(|state| {
            let value = domain::order::serdes::deserialize(&bytes)?;
            if value.order_id != state.order_id {
                Err("Invalid state".to_string())
            } else {
                state.clone_from(&value);
                Ok(())
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);

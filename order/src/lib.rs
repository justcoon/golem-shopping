mod bindings;
mod domain;

use crate::bindings::exports::golem::api::{load_snapshot, save_snapshot};
use crate::bindings::exports::golem::order_exports::api::*;
use crate::bindings::golem::pricing_exports::api::PricingItem;
use crate::bindings::golem::product_exports::api::Product;
use std::cell::RefCell;
use std::str::FromStr;
use email_address::EmailAddress;

struct Component;

fn get_product(product_id: String) -> Option<Product> {
    println!("Getting product: {}", product_id);

    use bindings::golem::product_client::product_client::*;

    let api = Api::new(product_id.as_str());

    api.blocking_get()
}

fn get_pricing(product_id: String, currency: String, zone: String) -> Option<PricingItem> {
    println!("Getting pricing: {}, currency: {}, zone: {}", product_id, currency, zone);

    use bindings::golem::pricing_client::pricing_client::*;

    let api = Api::new(product_id.as_str());

    api.blocking_get_price(&currency, &zone)
}

fn action_not_allowed_error(status: domain::order::OrderStatus) -> ActionNotAllowedError {
    ActionNotAllowedError {
        message: "Can not update order with status".to_string(),
        status: status.into(),
    }
}

fn item_not_found_error(product_id: String) -> ItemNotFoundError {
    ItemNotFoundError { message: "Item not found".to_string(), product_id }
}

fn pricing_not_found_error(product_id: String) -> PricingNotFoundError {
    PricingNotFoundError { message: "Pricing not found".to_string(), product_id }
}

fn product_not_found_error(product_id: String) -> ProductNotFoundError {
    ProductNotFoundError { message: "Product not found".to_string(), product_id }
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
    fn initialize_order(data: CreateOrder) -> Result<(), InitOrderError> {
        with_state(|state| {
            println!("Initializing order {} for user {}", state.order_id, data.user_id);
            if state.order_status == domain::order::OrderStatus::New {
                let now = chrono::Utc::now();
                state.user_id = data.user_id;
                state.email = Some(data.email);
                state.currency = data.currency;
                state.created_at = now;
                state.updated_at = now;
                state.billing_address = data.billing_address.map(|v| v.into());
                state.shipping_address = data.shipping_address.map(|v| v.into());
                state.total = data.total;
                state.items = data.items.into_iter().map(|item| item.into()).collect();

                Ok(())
            } else {
                Err(InitOrderError::ActionNotAllowed(action_not_allowed_error(state.order_status)))
            }
        })
    }


    fn update_email(email: String) -> Result<(), UpdateEmailError> {
        with_state(|state| {
            println!("Updating email {} for the order {} of user {}", email, state.order_id, state.user_id);

            if state.order_status == domain::order::OrderStatus::New {
                match EmailAddress::from_str(email.as_str()) {
                    Ok(_) => {
                        state.set_email(email);
                        Ok(())
                    }
                    Err(e) => Err(UpdateEmailError::EmailNotValid(EmailNotValidError {
                        message: format!("Invalid email: {e}"),
                    })),
                }
            } else {
                Err(UpdateEmailError::ActionNotAllowed(action_not_allowed_error(state.order_status)))
            }
        })
    }

    fn add_item(product_id: String, quantity: u32) -> Result<(), AddItemError> {
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
                                product_name: product.name,
                                product_brand: product.brand,
                                price: pricing.price,
                                quantity,
                            });
                        }
                        (None, _) => {
                            return Err(AddItemError::ProductNotFound(product_not_found_error(
                                product_id,
                            )));
                        }
                        _ => {
                            return Err(AddItemError::PricingNotFound(pricing_not_found_error(
                                product_id,
                            )))
                        }
                    }
                }

                Ok(())
            } else {
                Err(AddItemError::ActionNotAllowed(action_not_allowed_error(state.order_status)))
            }
        })
    }

    fn remove_item(product_id: String) -> Result<(), RemoveItemError> {
        with_state(|state| {
            println!(
                "Removing item with product {} from the order {} of user {}",
                product_id, state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                if state.remove_item(product_id.clone()) {
                    Ok(())
                } else {
                    Err(RemoveItemError::ItemNotFound(item_not_found_error(product_id)))
                }
            } else {
                Err(RemoveItemError::ActionNotAllowed(action_not_allowed_error(state.order_status)))
            }
        })
    }

    fn update_item_quantity(
        product_id: String,
        quantity: u32,
    ) -> Result<(), UpdateItemQuantityError> {
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
                    Err(UpdateItemQuantityError::ItemNotFound(item_not_found_error(product_id)))
                }
            } else {
                Err(UpdateItemQuantityError::ActionNotAllowed(action_not_allowed_error(
                    state.order_status,
                )))
            }
        })
    }

    fn cancel_order() -> Result<(), CancelOrderError> {
        with_state(|state| {
            println!("Cancelling order {} of user {}", state.order_id, state.user_id);

            if state.order_status == domain::order::OrderStatus::New {
                println!("Cancelling order {} of user {}", state.order_id, state.user_id);
                state.set_order_status(domain::order::OrderStatus::Cancelled);
                Ok(())
            } else {
                println!("Cancelling order {} of user {}", state.order_id, state.user_id);
                Err(CancelOrderError::ActionNotAllowed(action_not_allowed_error(
                    state.order_status,
                )))
            }
        })
    }

    fn ship_order() -> Result<(), ShipOrderError> {
        with_state(|state| {
            println!("Shipping order {} of user {}", state.order_id, state.user_id);
            if state.order_status != domain::order::OrderStatus::New {
                Err(ShipOrderError::ActionNotAllowed(action_not_allowed_error(state.order_status)))
            } else if state.items.is_empty() {
                Err(ShipOrderError::EmptyItems(EmptyItemsError { message: "Empty items".to_string() }))
            } else if state.billing_address.is_none() {
                Err(ShipOrderError::BillingAddressNotSet(BillingAddressNotSetError {
                    message: "Billing address not set".to_string(),
                }))
            } else if state.email.is_none() {
                Err(ShipOrderError::EmptyEmail(EmptyEmailError { message: "Email not set".to_string() }))
            } else {
                state.set_order_status(domain::order::OrderStatus::Shipped);
                Ok(())
            }
        })
    }

    fn update_billing_address(address: Address) -> Result<(), UpdateAddressError> {
        with_state(|state| {
            println!(
                "Updating billing address in the order {} of user {}",
                state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                state.set_billing_address(address.into());
                Ok(())
            } else {
                Err(UpdateAddressError::ActionNotAllowed(action_not_allowed_error(
                    state.order_status,
                )))
            }
        })
    }

    fn update_shipping_address(address: Address) -> Result<(), UpdateAddressError> {
        with_state(|state| {
            println!(
                "Updating shipping address in the order {} of user {}",
                state.order_id, state.user_id
            );
            if state.order_status == domain::order::OrderStatus::New {
                state.set_shipping_address(address.into());
                Ok(())
            } else {
                Err(UpdateAddressError::ActionNotAllowed(action_not_allowed_error(
                    state.order_status,
                )))
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

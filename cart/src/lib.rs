mod bindings;
mod domain;

use crate::bindings::exports::golem::api::{load_snapshot, save_snapshot};
use crate::bindings::exports::golem::cart::api::*;
use crate::bindings::golem::pricing_stub::stub_pricing::PricingItem;
use crate::bindings::golem::product_stub::stub_product::Product;
use std::cell::RefCell;
use std::env;

use uuid::Uuid;

struct Component;

fn generate_order_id() -> String {
    Uuid::new_v4().to_string()
}

fn get_product_worker_urn(product_id: String) -> String {
    let component_id = std::env::var("PRODUCT_COMPONENT_ID").expect("PRODUCT_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/{}", product_id)
}

fn get_pricing_worker_urn(product_id: String) -> String {
    let component_id = std::env::var("PRICING_COMPONENT_ID").expect("PRICING_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/{}", product_id)
}

fn get_order_worker_urn(oder_id: String) -> String {
    let component_id = std::env::var("ORDER_COMPONENT_ID").expect("ORDER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/{}", oder_id)
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

fn validate_cart(cart: domain::cart::Cart) -> Result<(), Error> {
    if cart.items.is_empty() {
        Err(Error::EmptyItems(EmptyItemsError { message: "Empty items".to_string() }))
    } else if cart.billing_address.is_none() {
        Err(Error::BillingAddressNotSet(BillingAddressNotSetError {
            message: "Billing address not set".to_string(),
        }))
    } else {
        Ok(())
    }
}

fn create_order(order_id: String, cart: domain::cart::Cart) -> Result<String, Error> {
    println!("Creating order: {}", order_id);

    validate_cart(cart.clone())?;

    use bindings::golem::order_stub::stub_order::*;
    use bindings::golem::rpc::types::Uri;

    let api = Api::new(&Uri { value: get_order_worker_urn(order_id.clone()) });

    let order = cart.into();

    api.blocking_initialize_order(&order);

    Ok(order_id)
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
    static STATE: RefCell<Option<domain::cart::Cart>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut domain::cart::Cart) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name = env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let value = domain::cart::Cart::new(worker_name);
            *state = Some(value);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn add_item(product_id: String, quantity: u32) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Adding item with product {} to the cart of user {}",
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
                let product = get_product(product_id.clone());
                let pricing =
                    get_pricing(product_id.clone(), state.currency.clone(), "global".to_string());
                match (product, pricing) {
                    (Some(product), Some(pricing)) => {
                        state.items.push(domain::cart::CartItem {
                            product_id,
                            name: product.name,
                            price: pricing.price,
                            quantity,
                        });
                    }
                    (None, _) => {
                        return Err(product_not_found_error(product_id));
                    }
                    _ => return Err(pricing_not_found_error(product_id)),
                }
            }

            state.total = domain::cart::get_total_price(state.items.clone());

            Ok(())
        })
    }

    fn remove_item(product_id: String) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Removing item with product {} from the cart of user {}",
                product_id, state.user_id
            );

            if state.items.iter().any(|item| item.product_id == product_id) {
                state.items.retain(|item| item.product_id != product_id);
                state.total = domain::cart::get_total_price(state.items.clone());
                Ok(())
            } else {
                Err(item_not_found_error(product_id))
            }
        })
    }

    fn update_item_quantity(product_id: String, quantity: u32) -> Result<(), Error> {
        with_state(|state| {
            println!(
                "Updating quantity of item with product {} to {} in the cart of user {}",
                product_id, quantity, state.user_id
            );

            let mut updated = false;

            for item in &mut state.items {
                if item.product_id == product_id {
                    item.quantity = quantity;
                    updated = true;
                }
            }

            if updated {
                state.total = domain::cart::get_total_price(state.items.clone());

                Ok(())
            } else {
                Err(item_not_found_error(product_id))
            }
        })
    }

    fn checkout() -> Result<OrderConfirmation, Error> {
        with_state(|state| {
            let order_id = generate_order_id();
            println!("Checkout for order {}", order_id);

            create_order(order_id.clone(), state.clone())?;

            state.items.clear();
            state.total = 0f32;
            state.shipping_address = None;
            state.billing_address = None;
            state.previous_order_ids.push(order_id.clone());

            Ok(OrderConfirmation { order_id })
        })
    }

    fn update_billing_address(address: Address) -> Result<(), Error> {
        with_state(|state| {
            println!("Updating billing address in the cart of user {}", state.user_id);

            state.billing_address = Some(address.into());
            Ok(())
        })
    }

    fn update_shipping_address(address: Address) -> Result<(), Error> {
        with_state(|state| {
            println!("Updating shipping address in the cart of user {}", state.user_id);

            state.shipping_address = Some(address.into());
            Ok(())
        })
    }

    fn get() -> Option<Cart> {
        STATE.with_borrow(|state| {
            println!("Getting cart");

            state.clone().map(|state| state.into())
        })
    }
}

impl save_snapshot::Guest for Component {
    fn save() -> Vec<u8> {
        with_state(|state| serde_json::to_vec_pretty(&state).expect("Failed to serialize state"))
    }
}

impl load_snapshot::Guest for Component {
    fn load(bytes: Vec<u8>) -> Result<(), String> {
        with_state(|state| {
            let value: domain::cart::Cart =
                serde_json::from_slice(&bytes).map_err(|err| err.to_string())?;
            if value.user_id != state.user_id {
                Err("Invalid state".to_string())
            } else {
                state.clone_from(&value);
                Ok(())
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);

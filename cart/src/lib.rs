mod bindings;

use crate::bindings::exports::golem::cart::api::*;
use std::cell::RefCell;
use std::env;

use crate::bindings::golem::pricing_stub::stub_pricing::PricingItem;
use crate::bindings::golem::product_stub::stub_product::Product;

use crate::bindings::golem::order::api::OrderItem;
use rand::prelude::*;
use uuid::Uuid;

struct Component;

fn generate_order_id() -> String {
    Uuid::new_v4().to_string()
}

fn get_total_price(items: Vec<CartItem>) -> f32 {
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
    println!("Getting pricing: {}", product_id);

    use bindings::golem::pricing_stub::stub_pricing::*;
    use bindings::golem::rpc::types::Uri;

    let api = Api::new(&Uri { value: get_pricing_worker_urn(product_id) });

    api.blocking_get_price(&zone, &currency)
}

fn create_order(order_id: String, cart: Cart) -> Result<String, &'static str> {
    println!("Creating order: {}", order_id);

    use bindings::golem::order_stub::stub_order::*;
    use bindings::golem::rpc::types::Uri;

    let api = Api::new(&Uri { value: get_order_worker_urn(order_id.clone()) });

    let shipping_address = cart.shipping_address.map(|x| Address {
        street1: x.street1,
        street2: x.street2,
        state_or_region: x.state_or_region,
        phone_number: x.phone_number,
        postal_code: x.postal_code,
        business_name: x.business_name,
        name: x.name,
        city: x.city,
        country: x.country,
    });

    let billing_address = cart.billing_address.map(|x| Address {
        street1: x.street1,
        street2: x.street2,
        state_or_region: x.state_or_region,
        phone_number: x.phone_number,
        postal_code: x.postal_code,
        business_name: x.business_name,
        name: x.name,
        city: x.city,
        country: x.country,
    });

    let order = CreateOrder {
        user_id: cart.user_id,
        items: cart
            .items
            .into_iter()
            .map(|item| OrderItem {
                product_id: item.product_id,
                quantity: item.quantity,
                price: item.price,
                name: item.name,
            })
            .collect(),
        total: cart.total,
        currency: cart.currency,
        timestamp: cart.timestamp,
        shipping_address,
        billing_address,
    };

    api.blocking_initialize_order(&order);

    Ok(order_id)
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
                shipping_address: None,
                billing_address: None,
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
                let product = get_product(product_id.clone());
                let pricing =
                    get_pricing(product_id.clone(), state.currency.clone(), "global".to_string());
                match (product, pricing) {
                    (Some(product), Some(pricing)) => {
                        state.items.push(CartItem {
                            product_id,
                            name: product.name,
                            price: pricing.price,
                            quantity,
                        });
                    }
                    _ => {
                        return Err("Product or pricing not found".to_string());
                    }
                }
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
            println!("Checkout for order {}", order_id);

            create_order(order_id.clone(), state.clone())?;

            state.items.clear();
            state.total = 0f32;
            state.shipping_address = None;
            state.billing_address = None;

            Ok(OrderConfirmation { order_id })
        });

        match result {
            Ok(OrderConfirmation { order_id }) => {
                CheckoutResult::Success(OrderConfirmation { order_id })
            }
            Err(err) => CheckoutResult::Error(err.to_string()),
        }
    }

    fn update_billing_address(address: Address) -> Result<(), String> {
        with_state(|state| {
            println!("Updating billing address in the cart of user {}", state.user_id);

            state.billing_address = Some(address);
            Ok(())
        })
    }

    fn update_shipping_address(address: Address) -> Result<(), String> {
        with_state(|state| {
            println!("Updating shipping address in the cart of user {}", state.user_id);

            state.shipping_address = Some(address);
            Ok(())
        })
    }

    fn get() -> Option<Cart> {
        STATE.with_borrow(|state| {
            println!("Getting cart");

            state.clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);

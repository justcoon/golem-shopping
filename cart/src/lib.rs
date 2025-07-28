mod bindings;
mod domain;

use crate::bindings::exports::golem::api::{load_snapshot, save_snapshot};
use crate::bindings::exports::golem::cart_exports::api::*;
use crate::bindings::golem::pricing_exports::api::PricingItem;
use crate::bindings::golem::product_exports::api::Product;
use email_address::EmailAddress;
use std::cell::RefCell;
use std::str::FromStr;
use uuid::Uuid;

struct Component;

fn generate_order_id() -> String {
    Uuid::new_v4().to_string()
}

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

fn validate_cart(cart: domain::cart::Cart) -> Result<(), CheckoutError> {
    if cart.items.is_empty() {
        Err(CheckoutError::EmptyItems(EmptyItemsError { message: "Empty items".to_string() }))
    } else if cart.billing_address.is_none() {
        Err(CheckoutError::BillingAddressNotSet(BillingAddressNotSetError {
            message: "Billing address not set".to_string(),
        }))
    } else if cart.email.is_none() {
        Err(CheckoutError::EmptyEmail(EmptyEmailError { message: "Email not set".to_string() }))
    } else {
        Ok(())
    }
}

fn create_order(order_id: String, cart: domain::cart::Cart) -> Result<String, CheckoutError> {
    println!("Creating order: {}", order_id);

    validate_cart(cart.clone())?;

    use bindings::golem::order_client::order_client::*;

    let api = Api::new(order_id.as_str());

    let order =
        cart.try_into().map_err(|e| CheckoutError::OrderCreate(OrderCreateError { message: e }))?;

    api.blocking_initialize_order(&order).map_err(|_| {
        CheckoutError::OrderCreate(OrderCreateError {
            message: "Failed to create order".to_string(),
        })
    })?;

    Ok(order_id)
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
    static STATE: RefCell<Option<domain::cart::Cart>> = const { RefCell::new(None) };
}

fn with_state<T>(f: impl FnOnce(&mut domain::cart::Cart) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        if state.is_none() {
            let worker_name =
                std::env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME must be set");
            let value = domain::cart::Cart::new(worker_name);
            *state = Some(value);
        }

        f(state.as_mut().unwrap())
    })
}

impl Guest for Component {
    fn add_item(product_id: String, quantity: u32) -> Result<(), AddItemError> {
        with_state(|state| {
            println!(
                "Adding item with product {} to the cart of user {}",
                product_id, state.user_id
            );

            let updated = state.update_item_quantity(product_id.clone(), quantity);

            if !updated {
                let product = get_product(product_id.clone());
                let pricing = get_pricing(
                    product_id.clone(),
                    state.currency.clone(),
                    domain::cart::PRICING_ZONE_DEFAULT.to_string(),
                );
                match (product, pricing) {
                    (Some(product), Some(pricing)) => {
                        state.add_item(domain::cart::CartItem {
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
        })
    }

    fn update_email(email: String) -> Result<(), UpdateEmailError> {
        with_state(|state| {
            println!("Updating email {} for the cart of user {}", email, state.user_id);

            match EmailAddress::from_str(email.as_str()) {
                Ok(_) => {
                    state.email = Some(email);
                    Ok(())
                }
                Err(e) => Err(UpdateEmailError::EmailNotValid(EmailNotValidError {
                    message: format!("Invalid email: {e}"),
                })),
            }
        })
    }

    fn remove_item(product_id: String) -> Result<(), RemoveItemError> {
        with_state(|state| {
            println!(
                "Removing item with product {} from the cart of user {}",
                product_id, state.user_id
            );

            if state.remove_item(product_id.clone()) {
                Ok(())
            } else {
                Err(RemoveItemError::ItemNotFound(item_not_found_error(product_id)))
            }
        })
    }

    fn update_item_quantity(
        product_id: String,
        quantity: u32,
    ) -> Result<(), UpdateItemQuantityError> {
        with_state(|state| {
            println!(
                "Updating quantity of item with product {} to {} in the cart of user {}",
                product_id, quantity, state.user_id
            );

            let updated = state.update_item_quantity(product_id.clone(), quantity);

            if updated {
                Ok(())
            } else {
                Err(UpdateItemQuantityError::ItemNotFound(item_not_found_error(product_id)))
            }
        })
    }

    fn checkout() -> Result<OrderConfirmation, CheckoutError> {
        with_state(|state| {
            let order_id = generate_order_id();
            println!("Checkout for order {}", order_id);

            create_order(order_id.clone(), state.clone())?;

            state.order_created(order_id.clone());

            Ok(OrderConfirmation { order_id })
        })
    }

    fn update_billing_address(address: Address) -> Result<(), UpdateAddressError> {
        with_state(|state| {
            println!("Updating billing address in the cart of user {}", state.user_id);

            state.billing_address = Some(address.into());
            Ok(())
        })
    }

    fn update_shipping_address(address: Address) -> Result<(), UpdateAddressError> {
        with_state(|state| {
            println!("Updating shipping address in the cart of user {}", state.user_id);

            state.shipping_address = Some(address.into());
            Ok(())
        })
    }

    fn get() -> Option<Cart> {
        STATE.with_borrow_mut(|state| {
            println!("Getting cart");
            if let Some(cart) = state {
                let mut items = Vec::new();
                for item in cart.items.clone() {
                    let product_id = item.product_id;
                    let quantity = item.quantity;
                    let product = get_product(product_id.clone());
                    let pricing = get_pricing(
                        product_id.clone(),
                        cart.currency.clone(),
                        domain::cart::PRICING_ZONE_DEFAULT.to_string(),
                    );
                    match (product, pricing) {
                        (Some(product), Some(pricing)) => {
                            items.push(domain::cart::CartItem {
                                product_id,
                                product_name: product.name,
                                product_brand: product.brand,
                                price: pricing.price,
                                quantity,
                            });
                        }
                        _ => ()
                    }
                }
                cart.items = items;
                cart.recalculate_total();
                Some(cart.clone().into())
            } else { 
                None
            }
        })
    }
}

impl save_snapshot::Guest for Component {
    fn save() -> Vec<u8> {
        with_state(|state| {
            domain::cart::serdes::serialize(state).expect("Failed to serialize state")
        })
    }
}

impl load_snapshot::Guest for Component {
    fn load(bytes: Vec<u8>) -> Result<(), String> {
        with_state(|state| {
            let value = domain::cart::serdes::deserialize(&bytes)?;
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

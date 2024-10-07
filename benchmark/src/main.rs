mod domain;
mod goose_ext;

use goose::prelude::*;
use goose_ext::GooseRequestExt;
use rand::seq::SliceRandom;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    let custom_host = match std::env::var("HOST") {
        Ok(host) => host,
        Err(_) => "".to_string(),
    };

    GooseAttack::initialize()?
        .register_scenario(
            scenario!("Get Products")
                // After each transactions runs, sleep randomly from 5 to 15 seconds.
                .set_wait_time(Duration::from_secs(5), Duration::from_secs(15))?
                .register_transaction(transaction!(get_products)),
        )
        .register_scenario(
            scenario!("Get Pricing")
                // After each transactions runs, sleep randomly from 5 to 15 seconds.
                .set_wait_time(Duration::from_secs(5), Duration::from_secs(15))?
                .register_transaction(transaction!(get_pricing)),
        )
        .register_scenario(
            scenario!("Create and checkout cart")
                // After each transactions runs, sleep randomly from 5 to 15 seconds.
                .set_wait_time(Duration::from_secs(5), Duration::from_secs(15))?
                .register_transaction(transaction!(create_and_checkout_cart)),
        )
        .set_default(GooseDefault::Host, custom_host.as_str())?
        .execute()
        .await?;

    Ok(())
}

async fn get_products(user: &mut GooseUser) -> TransactionResult {
    let product_id = rand_product_id();

    let _goose =
        user.get_request("product-get", format!("/v1/product/{product_id}").as_str()).await?;

    Ok(())
}

async fn get_pricing(user: &mut GooseUser) -> TransactionResult {
    let product_id = rand_product_id();

    let _goose =
        user.get_request("pricing-get", format!("/v1/pricing/{product_id}").as_str()).await?;

    Ok(())
}

async fn create_and_checkout_cart(user: &mut GooseUser) -> TransactionResult {
    let item_count = 4;

    let product_ids = get_product_ids();
    let product_ids: Vec<_> =
        product_ids.choose_multiple(&mut rand::thread_rng(), item_count).collect();

    let user_id = get_user_ids().choose(&mut rand::thread_rng()).unwrap().to_string();

    let address = get_addresses().choose(&mut rand::thread_rng()).unwrap().to_owned();

    for product_id in product_ids.iter() {
        let _goose = user
            .put_request(
                "cart-add-item",
                format!("/v1/cart/{user_id}/items/{product_id}").as_str(),
                &domain::common::AddItem::new(1),
            )
            .await?;
    }

    let product_id = product_ids[0];

    let _goose = user
        .delete_request(
            "cart-delete-item",
            format!("/v1/cart/{user_id}/items/{product_id}").as_str(),
        )
        .await?;

    let _goose = user
        .put_request(
            "cart-set-billing-address",
            format!("/v1/cart/{user_id}/billing-address").as_str(),
            &address,
        )
        .await?;

    let _goose = user
        .post_request(
            "cart-checkout",
            format!("/v1/cart/{user_id}/checkout").as_str(),
            &serde_json::Value::Null,
        )
        .await?;

    Ok(())
}
fn rand_product_id() -> String {
    let product_ids = get_product_ids();

    product_ids.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn get_product_ids() -> Vec<String> {
    (1..=50).map(|v| format!("p{:03}", v)).collect()
}

fn get_user_ids() -> Vec<String> {
    (1..=4).map(|v| format!("user{:03}", v)).collect()
}

fn get_addresses() -> Vec<domain::common::Address> {
    let mut addresses = Vec::new();

    addresses.push(domain::common::Address {
        street1: "123 Main St".to_string(),
        street2: None,
        state_or_region: "CA".to_string(),
        phone_number: Some("555-555-5555".to_string()),
        postal_code: "12345".to_string(),
        business_name: None,
        name: Some("John Doe".to_string()),
        city: "San Francisco".to_string(),
        country: "USA".to_string(),
    });

    addresses.push(domain::common::Address {
        street1: "123 Main St".to_string(),
        street2: None,
        state_or_region: "Washington".to_string(),
        phone_number: Some("555-555-1234".to_string()),
        postal_code: "23456".to_string(),
        business_name: None,
        name: Some("John Doe".to_string()),
        city: "Washington DC".to_string(),
        country: "USA".to_string(),
    });

    addresses.push(domain::common::Address {
        street1: "123 Main St".to_string(),
        street2: None,
        state_or_region: "NY".to_string(),
        phone_number: Some("555-555-3456".to_string()),
        postal_code: "3456".to_string(),
        business_name: None,
        name: None,
        city: "New York".to_string(),
        country: "USA".to_string(),
    });

    addresses
}

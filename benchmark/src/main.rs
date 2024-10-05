use goose::prelude::*;
use rand::seq::SliceRandom;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
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
        .execute()
        .await?;

    Ok(())
}

async fn get_products(user: &mut GooseUser) -> TransactionResult {
    let product_id = rand_product_id();

    let request_builder = user
        .get_request_builder(&GooseMethod::Get, format!("/v1/product/{product_id}").as_str())?
        .header("Accept", "application/json");

    let request = GooseRequest::builder().set_request_builder(request_builder).build();

    let _goose = user.request(request).await?;

    Ok(())
}

async fn get_pricing(user: &mut GooseUser) -> TransactionResult {
    let product_id = rand_product_id();

    let request_builder = user
        .get_request_builder(&GooseMethod::Get, format!("/v1/pricing/{product_id}").as_str())?
        .header("Accept", "application/json");

    let request = GooseRequest::builder().set_request_builder(request_builder).build();

    let _goose = user.request(request).await?;

    Ok(())
}

fn rand_product_id() -> String {
    let product_ids = get_product_ids();

    product_ids.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn get_product_ids() -> Vec<String> {
    (1..=50).map(|v| format!("p{:03}", v)).collect()
}

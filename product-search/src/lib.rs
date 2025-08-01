mod bindings;

use crate::bindings::exports::golem::product_search_exports::api::*;
use crate::bindings::golem::api::host::resolve_component_id;
use crate::bindings::golem::api::host::GetWorkers;
struct Component;

fn get_product(product_id: String) -> Option<bindings::golem::product_exports::api::Product> {
    use bindings::golem::product_client::product_client::*;
    let api = Api::new(product_id.as_str());
    api.blocking_get()
}

impl From<bindings::golem::product_exports::api::Product> for Product {
    fn from(value: bindings::golem::product_exports::api::Product) -> Self {
        Product {
            product_id: value.product_id,
            name: value.name,
            brand: value.brand,
            description: value.description,
            tags: value.tags,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Guest for Component {
    fn find(filter: ProductFilter) -> Vec<Product> {
        let component_id = resolve_component_id("golem:product");

        let mut products: Vec<Product> = Vec::new();

        if let Some(component_id) = component_id {
            let get_workers = GetWorkers::new(component_id, None, false);

            while let Some(workers) = get_workers.get_next() {
                for worker in workers {
                    let product = get_product(worker.worker_id.worker_name);
                    if let Some(product) = product {
                        if filter
                            .brand
                            .clone()
                            .filter(|v| v.len() > 0)
                            .is_none_or(|v| v.to_lowercase() == product.brand.to_lowercase())
                            || filter
                                .name
                                .clone()
                                .filter(|v| v.len() > 0)
                                .is_none_or(|v| v.to_lowercase() == product.name.to_lowercase())
                        {
                            products.push(product.into());
                        }
                    }
                }
            }
        }

        products
    }
}

bindings::export!(Component with_types_in bindings);

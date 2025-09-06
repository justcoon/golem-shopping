mod bindings;

use crate::bindings::exports::golem::product_search_exports::api::*;
use golem_rust::bindings::golem::api::host::{resolve_component_id, GetWorkers};
use std::collections::HashSet;

fn get_products(
    workers: HashSet<String>,
    matcher: ProductQueryMatcher,
) -> Result<Vec<Product>, String> {
    // https://learn.golem.cloud/common-language-guide/rpc#writing-non-blocking-remote-calls

    let mut futures = vec![];
    let mut subs = vec![];
    for entry in workers {
        let api = bindings::golem::product_client::product_client::Api::new(entry.as_str());
        let response = api.get();
        let sub = response.subscribe();
        futures.push(response);
        subs.push(sub);
    }

    let n = futures.len();

    let mut values: Vec<Product> = vec![];
    let mut mapping: Vec<usize> = (0..n).collect();
    let mut remaining = subs.iter().collect::<Vec<_>>();

    // Repeatedly poll the futures until all of them are ready
    while !remaining.is_empty() {
        let poll_result = golem_rust::wasm_rpc::wasi::io::poll::poll(remaining.as_slice());

        // poll_result is a list of indexes of the futures that are ready
        for idx in &poll_result {
            let counter_idx = mapping[*idx as usize];
            let future = &futures[counter_idx];
            let value = future.get().ok_or("Get product failed")?;
            if let Some(v) = value {
                if matcher.matches(&v) {
                    values.push(v.into());
                }
            }
        }

        // Removing the completed futures from the list
        remaining = remaining
            .into_iter()
            .enumerate()
            .filter_map(
                |(idx, item)| {
                    if poll_result.contains(&(idx as u32)) {
                        None
                    } else {
                        Some(item)
                    }
                },
            )
            .collect();

        // Updating the index mapping
        mapping = mapping
            .into_iter()
            .enumerate()
            .filter_map(
                |(idx, item)| {
                    if poll_result.contains(&(idx as u32)) {
                        None
                    } else {
                        Some(item)
                    }
                },
            )
            .collect();
    }

    Ok(values)
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

#[derive(Clone, Debug)]
struct ProductQueryMatcher {
    terms: Vec<String>,
    field_filters: Vec<(String, String)>,
}

impl ProductQueryMatcher {
    // Parse a simple query string into terms and field filters
    fn new(query: &str) -> Self {
        let mut terms = Vec::new();
        let mut field_filters = Vec::new();

        let tokens = Self::tokenize(query);

        for part in tokens {
            if let Some((field, value)) = part.split_once(':') {
                field_filters.push((field.to_string(), value.to_string()));
            } else {
                terms.push(part.to_string());
            }
        }

        Self { terms, field_filters }
    }

    // Tokenize the query string, handling quoted strings
    fn tokenize(query: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for c in query.chars() {
            match c {
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        tokens.push(current.trim().to_string());
                        current.clear();
                    }
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                _ => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current.trim().to_string());
        }

        tokens
    }

    // Check if a product matches the query
    pub fn matches(&self, product: &bindings::golem::product_exports::api::Product) -> bool {
        fn text_matches(text: &str, query: &str) -> bool {
            query == "*" || text.to_lowercase().contains(&query.to_lowercase())
        }

        // Check field filters first
        for (field, value) in self.field_filters.iter() {
            let matches = match field.to_lowercase().as_str() {
                "name" => text_matches(&product.name, &value),
                "brand" => text_matches(&product.brand, &value),
                "description" => text_matches(&product.description, &value),
                "tag" | "tags" => product.tags.iter().any(|tag| text_matches(tag, &value)),
                _ => false, // Unknown field
            };

            if !matches {
                return false;
            }
        }

        // If no terms to match, just check if field filters passed
        if self.terms.is_empty() {
            return true;
        }

        // Check search terms against all searchable fields
        for term in self.terms.iter() {
            let matches = text_matches(&product.name, &term)
                || text_matches(&product.brand, &term)
                || text_matches(&product.description, &term)
                || product.tags.iter().any(|tag| text_matches(tag, &term));

            if !matches {
                return false;
            }
        }

        true
    }
}

struct Component;

impl Guest for Component {
    fn search(query: String) -> Result<Vec<Product>, String> {
        let component_id = resolve_component_id("golem:product");

        if let Some(component_id) = component_id {
            let mut values: Vec<Product> = Vec::new();
            let matcher = ProductQueryMatcher::new(&query);

            let get_workers = GetWorkers::new(component_id, None, false);

            let mut processed_worker_names: HashSet<String> = HashSet::new();

            while let Some(workers) = get_workers.get_next() {
                let worker_names = workers
                    .iter()
                    .map(|w| w.worker_id.worker_name.clone())
                    .filter(|n| !processed_worker_names.contains(n))
                    .collect::<HashSet<_>>();

                let products = get_products(worker_names.clone(), matcher.clone())?;
                processed_worker_names.extend(worker_names);
                values.extend(products);
            }

            Ok(values)
        } else {
            Err("Product component not found".to_string())
        }
    }
}

bindings::export!(Component with_types_in bindings);

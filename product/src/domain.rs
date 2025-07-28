pub mod product {
    use crate::bindings;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Product {
        pub product_id: String,
        pub name: String,
        pub brand: String,
        pub description: String,
        pub tags: Vec<String>,
    }

    impl Product {
        pub fn new(product_id: String) -> Self {
            Self { product_id, name: "".to_string(), brand: "".to_string(), description: "".to_string(), tags: vec![] }
        }
    }

    impl From<bindings::exports::golem::product_exports::api::Product> for Product {
        fn from(value: bindings::exports::golem::product_exports::api::Product) -> Self {
            Self {
                product_id: value.product_id,
                name: value.name,
                brand: value.brand,
                description: value.description,
                tags: value.tags,
            }
        }
    }

    impl From<Product> for bindings::exports::golem::product_exports::api::Product {
        fn from(value: Product) -> Self {
            Self {
                product_id: value.product_id,
                name: value.name,
                brand: value.brand,
                description: value.description,
                tags: value.tags,
            }
        }
    }

    pub mod serdes {
        use crate::domain::product::Product;

        pub const SERIALIZATION_VERSION_V1: u8 = 1u8;

        pub fn serialize(value: &Product) -> Result<Vec<u8>, String> {
            let data = serde_json::to_vec_pretty(value).map_err(|err| err.to_string())?;

            let mut result = vec![SERIALIZATION_VERSION_V1];
            result.extend(data);

            Ok(result)
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Product, String> {
            let (version, data) = bytes.split_at(1);

            match version[0] {
                SERIALIZATION_VERSION_V1 => {
                    let value: Product =
                        serde_json::from_slice(data).map_err(|err| err.to_string())?;

                    Ok(value)
                }
                _ => Err("Unsupported serialization version".to_string()),
            }
        }
    }
}

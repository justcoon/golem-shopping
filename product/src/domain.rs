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
        pub created_at: chrono::DateTime<chrono::Utc>,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    impl Product {
        pub fn new(product_id: String) -> Self {
            let now = chrono::Utc::now();
            Self { product_id, name: "".to_string(), brand: "".to_string(), description: "".to_string(), tags: vec![], created_at: now, updated_at: now }
        }

        pub fn update(&mut self, name: String, brand: String, description: String, tags: Vec<String>) {
            self.name = name;
            self.brand = brand;
            self.description = description;
            self.tags = tags;
            self.updated_at = chrono::Utc::now();
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
                created_at: value.created_at.into(),
                updated_at: value.updated_at.into(),
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
                created_at: value.created_at.into(),
                updated_at: value.updated_at.into(),
            }
        }
    }

    impl From<bindings::wasi::clocks::wall_clock::Datetime> for chrono::DateTime<chrono::Utc> {
        fn from(value: bindings::wasi::clocks::wall_clock::Datetime) -> chrono::DateTime<chrono::Utc> {
            chrono::DateTime::from_timestamp(value.seconds as i64, value.nanoseconds)
                .expect("Received invalid datetime from wasi")
        }
    }

    impl From<chrono::DateTime<chrono::Utc>> for bindings::wasi::clocks::wall_clock::Datetime {
        fn from(value: chrono::DateTime<chrono::Utc>) -> bindings::wasi::clocks::wall_clock::Datetime {
            bindings::wasi::clocks::wall_clock::Datetime {
                seconds: value.timestamp() as u64,
                nanoseconds: value.timestamp_subsec_nanos(),
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

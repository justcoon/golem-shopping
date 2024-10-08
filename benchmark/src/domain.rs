pub mod common {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Address {
        pub street1: String,
        pub street2: Option<String>,
        pub city: String,
        pub state_or_region: String,
        pub country: String,
        pub postal_code: String,
        pub name: Option<String>,
        pub business_name: Option<String>,
        pub phone_number: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct AddItem {
        pub quantity: u32,
    }

    impl AddItem {
        pub fn new(quantity: u32) -> Self {
            Self { quantity }
        }
    }
}

pub mod cart {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct OrderCreated {
        pub order_id: String,
    }
}

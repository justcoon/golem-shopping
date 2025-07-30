pub mod cart {
    use crate::bindings;
    use serde::{Deserialize, Serialize};

    pub const CURRENCY_DEFAULT: &str = "USD";
    pub const PRICING_ZONE_DEFAULT: &str = "global";

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Address {
        pub street: String,
        pub city: String,
        pub state_or_region: String,
        pub country: String,
        pub postal_code: String,
        pub name: Option<String>,
        pub phone_number: Option<String>,
    }

    impl From<bindings::exports::golem::cart_exports::api::Address> for Address {
        fn from(value: bindings::exports::golem::cart_exports::api::Address) -> Self {
            Self {
                street: value.street,
                state_or_region: value.state_or_region,
                phone_number: value.phone_number,
                postal_code: value.postal_code,
                name: value.name,
                city: value.city,
                country: value.country,
            }
        }
    }

    impl From<Address> for bindings::exports::golem::cart_exports::api::Address {
        fn from(value: Address) -> Self {
            Self {
                street: value.street,
                state_or_region: value.state_or_region,
                phone_number: value.phone_number,
                postal_code: value.postal_code,
                name: value.name,
                city: value.city,
                country: value.country,
            }
        }
    }

    impl From<Address> for bindings::golem::order_exports::api::Address {
        fn from(value: Address) -> Self {
            Self {
                street: value.street,
                state_or_region: value.state_or_region,
                phone_number: value.phone_number,
                postal_code: value.postal_code,
                name: value.name,
                city: value.city,
                country: value.country,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Cart {
        pub user_id: String,
        pub email: Option<String>,
        pub items: Vec<CartItem>,
        pub billing_address: Option<Address>,
        pub shipping_address: Option<Address>,
        pub total: f32,
        pub currency: String,
        pub previous_order_ids: Vec<String>,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    impl Cart {
        pub fn new(user_id: String) -> Self {
            Self {
                user_id,
                email: None,
                items: vec![],
                billing_address: None,
                shipping_address: None,
                total: 0.0,
                currency: CURRENCY_DEFAULT.to_string(),
                updated_at: chrono::Utc::now(),
                previous_order_ids: vec![],
            }
        }

        pub fn order_created(&mut self, order_id: String) {
            self.clear();
            self.previous_order_ids.push(order_id);
        }

        pub fn clear(&mut self) {
            self.items.clear();
            self.billing_address = None;
            self.shipping_address = None;
            self.total = 0.0;
            self.updated_at = chrono::Utc::now();
        }

        pub fn recalculate_total(&mut self) {
            self.total = get_total_price(self.items.clone());
            self.updated_at = chrono::Utc::now();
        }

        pub fn add_item(&mut self, item: CartItem) -> bool {
            self.items.push(item);
            self.recalculate_total();
            true
        }

        pub fn set_items(&mut self, items: Vec<CartItem>) {
            self.items = items;
            self.recalculate_total();
        }

        pub fn set_billing_address(&mut self, address: Address) {
            self.billing_address = Some(address);
            self.updated_at = chrono::Utc::now();
        }

        pub fn set_shipping_address(&mut self, address: Address) {
            self.shipping_address = Some(address);
            self.updated_at = chrono::Utc::now();
        }

        pub fn set_email(&mut self, email: String) {
            self.email = Some(email);
            self.updated_at = chrono::Utc::now();
        }

        pub fn update_item_quantity(&mut self, product_id: String, quantity: u32) -> bool {
            let mut updated = false;

            for item in &mut self.items {
                if item.product_id == product_id {
                    item.quantity = quantity;
                    updated = true;
                }
            }

            if updated {
                self.recalculate_total();
            }

            updated
        }

        pub fn remove_item(&mut self, product_id: String) -> bool {
            let exist = self.items.iter().any(|item| item.product_id == product_id);

            if exist {
                self.items.retain(|item| item.product_id != product_id);
                self.recalculate_total();
            }

            exist
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct CartItem {
        pub product_id: String,
        pub product_name: String,
        pub product_brand: String,
        pub price: f32,
        pub quantity: u32,
    }

    impl From<CartItem> for bindings::golem::order_exports::api::OrderItem {
        fn from(value: CartItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                product_name: value.product_name,
                product_brand: value.product_brand,
            }
        }
    }
    impl From<CartItem> for bindings::exports::golem::cart_exports::api::CartItem {
        fn from(value: CartItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                product_name: value.product_name,
                product_brand: value.product_brand,
            }
        }
    }

    impl TryFrom<Cart> for bindings::golem::order_exports::api::CreateOrder {
        type Error = String;

        fn try_from(value: Cart) -> Result<Self, Self::Error> {
            let email = value.email.ok_or("Missing email")?;
            Ok(Self {
                user_id: value.user_id,
                email,
                items: value.items.into_iter().map(|item| item.into()).collect(),
                total: value.total,
                currency: value.currency,
                shipping_address: value.shipping_address.map(|a| a.into()),
                billing_address: value.billing_address.map(|a| a.into()),
            })
        }
    }

    impl From<Cart> for bindings::exports::golem::cart_exports::api::Cart {
        fn from(value: Cart) -> Self {
            Self {
                user_id: value.user_id,
                email: value.email,
                items: value.items.into_iter().map(|item| item.into()).collect(),
                total: value.total,
                currency: value.currency,
                updated_at: value.updated_at.into(),
                shipping_address: value.shipping_address.map(|a| a.into()),
                billing_address: value.billing_address.map(|a| a.into()),
                previous_order_ids: value.previous_order_ids,
            }
        }
    }

    pub fn get_total_price(items: Vec<CartItem>) -> f32 {
        let mut total = 0f32;

        for item in items {
            total += item.price * item.quantity as f32;
        }

        total
    }

    impl From<bindings::wasi::clocks::wall_clock::Datetime> for chrono::DateTime<chrono::Utc> {
        fn from(
            value: bindings::wasi::clocks::wall_clock::Datetime,
        ) -> chrono::DateTime<chrono::Utc> {
            chrono::DateTime::from_timestamp(value.seconds as i64, value.nanoseconds)
                .expect("Received invalid datetime from wasi")
        }
    }

    impl From<chrono::DateTime<chrono::Utc>> for bindings::wasi::clocks::wall_clock::Datetime {
        fn from(
            value: chrono::DateTime<chrono::Utc>,
        ) -> bindings::wasi::clocks::wall_clock::Datetime {
            bindings::wasi::clocks::wall_clock::Datetime {
                seconds: value.timestamp() as u64,
                nanoseconds: value.timestamp_subsec_nanos(),
            }
        }
    }

    pub mod serdes {
        use crate::domain::cart::Cart;

        pub const SERIALIZATION_VERSION_V1: u8 = 1u8;

        pub fn serialize(value: &Cart) -> Result<Vec<u8>, String> {
            let data = serde_json::to_vec_pretty(value).map_err(|err| err.to_string())?;

            let mut result = vec![SERIALIZATION_VERSION_V1];
            result.extend(data);

            Ok(result)
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Cart, String> {
            let (version, data) = bytes.split_at(1);

            match version[0] {
                SERIALIZATION_VERSION_V1 => {
                    let value: Cart =
                        serde_json::from_slice(data).map_err(|err| err.to_string())?;

                    Ok(value)
                }
                _ => Err("Unsupported serialization version".to_string()),
            }
        }
    }
}

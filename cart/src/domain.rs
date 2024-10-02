pub mod cart {
    use crate::bindings;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
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

    impl From<bindings::exports::golem::cart::api::Address> for Address {
        fn from(value: bindings::exports::golem::cart::api::Address) -> Self {
            Self {
                street1: value.street1,
                street2: value.street2,
                state_or_region: value.state_or_region,
                phone_number: value.phone_number,
                postal_code: value.postal_code,
                business_name: value.business_name,
                name: value.name,
                city: value.city,
                country: value.country,
            }
        }
    }

    impl From<Address> for bindings::exports::golem::cart::api::Address {
        fn from(value: Address) -> Self {
            Self {
                street1: value.street1,
                street2: value.street2,
                state_or_region: value.state_or_region,
                phone_number: value.phone_number,
                postal_code: value.postal_code,
                business_name: value.business_name,
                name: value.name,
                city: value.city,
                country: value.country,
            }
        }
    }

    impl From<Address> for bindings::golem::order::api::Address {
        fn from(value: Address) -> Self {
            Self {
                street1: value.street1,
                street2: value.street2,
                state_or_region: value.state_or_region,
                phone_number: value.phone_number,
                postal_code: value.postal_code,
                business_name: value.business_name,
                name: value.name,
                city: value.city,
                country: value.country,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Cart {
        pub user_id: String,
        pub items: Vec<CartItem>,
        pub billing_address: Option<Address>,
        pub shipping_address: Option<Address>,
        pub total: f32,
        pub currency: String,
        pub timestamp: u64,
        pub previous_order_ids: Vec<String>,
    }

    impl Cart {
        pub fn new(user_id: String) -> Self {
            Self {
                user_id,
                items: vec![],
                billing_address: None,
                shipping_address: None,
                total: 0.0,
                currency: "USD".to_string(),
                timestamp: 0,
                previous_order_ids: vec![],
            }
        }

        pub fn order_created(&mut self, order_id: String) {
            self.items.clear();
            self.billing_address = None;
            self.shipping_address = None;
            self.total = 0.0;
            self.previous_order_ids.push(order_id);
        }

        pub fn recalculate_total(&mut self) {
            self.total = get_total_price(self.items.clone());
        }

        pub fn has_item(&self, product_id: String) -> bool {
            self.items.clone().into_iter().any(|item| item.product_id == product_id)
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
    pub struct CartItem {
        pub product_id: String,
        pub name: String,
        pub price: f32,
        pub quantity: u32,
    }

    impl From<CartItem> for bindings::golem::order::api::OrderItem {
        fn from(value: CartItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                name: value.name,
            }
        }
    }
    impl From<CartItem> for bindings::exports::golem::cart::api::CartItem {
        fn from(value: CartItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                name: value.name,
            }
        }
    }

    impl From<Cart> for bindings::golem::order::api::CreateOrder {
        fn from(value: Cart) -> Self {
            Self {
                user_id: value.user_id,
                items: value.items.into_iter().map(|item| item.into()).collect(),
                total: value.total,
                currency: value.currency,
                timestamp: value.timestamp,
                shipping_address: value.shipping_address.map(|a| a.into()),
                billing_address: value.billing_address.map(|a| a.into()),
            }
        }
    }

    impl From<Cart> for bindings::exports::golem::cart::api::Cart {
        fn from(value: Cart) -> Self {
            Self {
                user_id: value.user_id,
                items: value.items.into_iter().map(|item| item.into()).collect(),
                total: value.total,
                currency: value.currency,
                timestamp: value.timestamp,
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

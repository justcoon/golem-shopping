pub mod order {
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

    impl From<bindings::exports::golem::order_exports::api::Address> for Address {
        fn from(value: bindings::exports::golem::order_exports::api::Address) -> Self {
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

    impl From<Address> for bindings::exports::golem::order_exports::api::Address {
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
    pub struct Order {
        pub order_id: String,
        pub user_id: String,
        pub email: Option<String>,
        pub order_status: OrderStatus,
        pub items: Vec<OrderItem>,
        pub billing_address: Option<Address>,
        pub shipping_address: Option<Address>,
        pub total: f32,
        pub currency: String,
        pub timestamp: u64,
    }

    impl Order {
        pub fn new(order_id: String, user_id: String) -> Self {
            Self {
                order_id,
                user_id,
                email: None,
                order_status: OrderStatus::New,
                items: vec![],
                shipping_address: None,
                billing_address: None,
                total: 0f32,
                currency: CURRENCY_DEFAULT.to_string(),
                timestamp: 0,
            }
        }

        pub fn recalculate_total(&mut self) {
            self.total = get_total_price(self.items.clone());
        }

        pub fn add_item(&mut self, item: OrderItem) -> bool {
            self.items.push(item);
            self.recalculate_total();
            true
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
    pub struct OrderItem {
        pub product_id: String,
        pub product_name: String,
        pub product_brand: String,
        pub price: f32,
        pub quantity: u32,
    }

    impl From<OrderItem> for bindings::exports::golem::order_exports::api::OrderItem {
        fn from(value: OrderItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                product_name: value.product_name,
                product_brand: value.product_brand,
            }
        }
    }

    impl From<bindings::exports::golem::order_exports::api::OrderItem> for OrderItem {
        fn from(value: bindings::exports::golem::order_exports::api::OrderItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                product_name: value.product_name,
                product_brand: value.product_brand,
            }
        }
    }

    impl From<Order> for bindings::exports::golem::order_exports::api::Order {
        fn from(value: Order) -> Self {
            Self {
                order_id: value.order_id,
                user_id: value.user_id,
                email: value.email,
                items: value.items.into_iter().map(|item| item.into()).collect(),
                total: value.total,
                order_status: value.order_status.into(),
                currency: value.currency,
                timestamp: value.timestamp,
                shipping_address: value.shipping_address.map(|a| a.into()),
                billing_address: value.billing_address.map(|a| a.into()),
            }
        }
    }

    impl From<bindings::exports::golem::order_exports::api::Order> for Order {
        fn from(value: bindings::exports::golem::order_exports::api::Order) -> Self {
            Self {
                order_id: value.order_id,
                user_id: value.user_id,
                email: value.email,
                items: value.items.into_iter().map(|item| item.into()).collect(),
                total: value.total,
                order_status: value.order_status.into(),
                currency: value.currency,
                timestamp: value.timestamp,
                shipping_address: value.shipping_address.map(|a| a.into()),
                billing_address: value.billing_address.map(|a| a.into()),
            }
        }
    }

    #[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum OrderStatus {
        New,
        Shipped,
        Cancelled,
    }

    impl From<OrderStatus> for bindings::exports::golem::order_exports::api::OrderStatus {
        fn from(value: OrderStatus) -> Self {
            match value {
                OrderStatus::New => Self::New,
                OrderStatus::Shipped => Self::Shipped,
                OrderStatus::Cancelled => Self::Cancelled,
            }
        }
    }

    impl From<bindings::exports::golem::order_exports::api::OrderStatus> for OrderStatus {
        fn from(value: bindings::exports::golem::order_exports::api::OrderStatus) -> Self {
            match value {
                bindings::exports::golem::order_exports::api::OrderStatus::New => Self::New,
                bindings::exports::golem::order_exports::api::OrderStatus::Shipped => Self::Shipped,
                bindings::exports::golem::order_exports::api::OrderStatus::Cancelled => Self::Cancelled,
            }
        }
    }

    pub fn get_total_price(items: Vec<OrderItem>) -> f32 {
        let mut total = 0f32;

        for item in items {
            total += item.price * item.quantity as f32;
        }

        total
    }

    pub mod serdes {
        use crate::domain::order::Order;

        pub const SERIALIZATION_VERSION_V1: u8 = 1u8;

        pub fn serialize(value: &Order) -> Result<Vec<u8>, String> {
            let data = serde_json::to_vec_pretty(value).map_err(|err| err.to_string())?;

            let mut result = vec![SERIALIZATION_VERSION_V1];
            result.extend(data);

            Ok(result)
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Order, String> {
            let (version, data) = bytes.split_at(1);

            match version[0] {
                SERIALIZATION_VERSION_V1 => {
                    let value: Order =
                        serde_json::from_slice(data).map_err(|err| err.to_string())?;

                    Ok(value)
                }
                _ => Err("Unsupported serialization version".to_string()),
            }
        }
    }
}

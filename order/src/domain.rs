pub mod order {
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

    impl From<bindings::exports::golem::order::api::Address> for Address {
        fn from(value: bindings::exports::golem::order::api::Address) -> Self {
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

    impl From<Address> for bindings::exports::golem::order::api::Address {
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
    pub struct Order {
        pub order_id: String,
        pub user_id: String,
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
                order_status: OrderStatus::New,
                items: vec![],
                shipping_address: None,
                billing_address: None,
                total: 0f32,
                currency: "USD".to_string(),
                timestamp: 0,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct OrderItem {
        pub product_id: String,
        pub name: String,
        pub price: f32,
        pub quantity: u32,
    }

    impl From<OrderItem> for bindings::exports::golem::order::api::OrderItem {
        fn from(value: OrderItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                name: value.name,
            }
        }
    }

    impl From<bindings::exports::golem::order::api::OrderItem> for OrderItem {
        fn from(value: bindings::exports::golem::order::api::OrderItem) -> Self {
            Self {
                product_id: value.product_id,
                quantity: value.quantity,
                price: value.price,
                name: value.name,
            }
        }
    }

    impl From<Order> for bindings::exports::golem::order::api::Order {
        fn from(value: Order) -> Self {
            Self {
                order_id: value.order_id,
                user_id: value.user_id,
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

    impl From<bindings::exports::golem::order::api::Order> for Order {
        fn from(value: bindings::exports::golem::order::api::Order) -> Self {
            Self {
                order_id: value.order_id,
                user_id: value.user_id,
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
    pub enum OrderStatus {
        New,
        Shipped,
        Cancelled,
    }

    impl From<OrderStatus> for bindings::exports::golem::order::api::OrderStatus {
        fn from(value: OrderStatus) -> Self {
            match value {
                OrderStatus::New => Self::New,
                OrderStatus::Shipped => Self::Shipped,
                OrderStatus::Cancelled => Self::Cancelled,
            }
        }
    }

    impl From<bindings::exports::golem::order::api::OrderStatus> for OrderStatus {
        fn from(value: bindings::exports::golem::order::api::OrderStatus) -> Self {
            match value {
                bindings::exports::golem::order::api::OrderStatus::New => Self::New,
                bindings::exports::golem::order::api::OrderStatus::Shipped => Self::Shipped,
                bindings::exports::golem::order::api::OrderStatus::Cancelled => Self::Cancelled,
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
}

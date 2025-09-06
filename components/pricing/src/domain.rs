pub mod pricing {
    use crate::bindings;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Pricing {
        pub product_id: String,
        pub msrp_prices: Vec<PricingItem>,
        pub list_prices: Vec<PricingItem>,
        pub sale_prices: Vec<SalePricingItem>,
        pub created_at: chrono::DateTime<chrono::Utc>,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    impl Pricing {
        pub fn new(product_id: String) -> Self {
            let now = chrono::Utc::now();
            Self {
                product_id,
                msrp_prices: vec![],
                list_prices: vec![],
                sale_prices: vec![],
                created_at: now,
                updated_at: now,
            }
        }

        pub fn get_price(&self, currency: String, zone: String) -> Option<PricingItem> {
            get_price(currency, zone, self.clone())
        }

        pub fn set_prices(
            &mut self,
            msrp_prices: Vec<PricingItem>,
            list_prices: Vec<PricingItem>,
            sale_prices: Vec<SalePricingItem>,
        ) {
            self.msrp_prices = msrp_prices;
            self.list_prices = list_prices;
            self.sale_prices = sale_prices;
            self.updated_at = chrono::Utc::now();
        }

        pub fn update_prices(
            &mut self,
            msrp_prices: Vec<PricingItem>,
            list_prices: Vec<PricingItem>,
            sale_prices: Vec<SalePricingItem>,
        ) {
            self.msrp_prices = merge_items(msrp_prices, self.msrp_prices.clone());
            self.list_prices = merge_items(list_prices, self.list_prices.clone());
            self.sale_prices = merge_sale_items(sale_prices, self.sale_prices.clone());
            self.updated_at = chrono::Utc::now();
        }
    }

    impl From<bindings::exports::golem::pricing_exports::api::Pricing> for Pricing {
        fn from(value: bindings::exports::golem::pricing_exports::api::Pricing) -> Self {
            Self {
                product_id: value.product_id,
                msrp_prices: value.msrp_prices.into_iter().map(|item| item.into()).collect(),
                list_prices: value.list_prices.into_iter().map(|item| item.into()).collect(),
                sale_prices: value.sale_prices.into_iter().map(|item| item.into()).collect(),
                created_at: value.created_at.into(),
                updated_at: value.updated_at.into(),
            }
        }
    }

    impl From<Pricing> for bindings::exports::golem::pricing_exports::api::Pricing {
        fn from(value: Pricing) -> Self {
            Self {
                product_id: value.product_id,
                msrp_prices: value.msrp_prices.into_iter().map(|item| item.into()).collect(),
                list_prices: value.list_prices.into_iter().map(|item| item.into()).collect(),
                sale_prices: value.sale_prices.into_iter().map(|item| item.into()).collect(),
                created_at: value.created_at.into(),
                updated_at: value.updated_at.into(),
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct PricingItem {
        pub price: f32,
        pub currency: String,
        pub zone: String,
    }

    impl PricingItem {
        fn key(&self) -> (String, String) {
            (self.zone.clone(), self.currency.clone())
        }
    }

    impl From<bindings::exports::golem::pricing_exports::api::PricingItem> for PricingItem {
        fn from(value: bindings::exports::golem::pricing_exports::api::PricingItem) -> Self {
            Self { price: value.price, currency: value.currency, zone: value.zone }
        }
    }

    impl From<PricingItem> for bindings::exports::golem::pricing_exports::api::PricingItem {
        fn from(value: PricingItem) -> Self {
            Self { price: value.price, currency: value.currency, zone: value.zone }
        }
    }

    impl From<SalePricingItem> for PricingItem {
        fn from(value: SalePricingItem) -> Self {
            Self { price: value.price, currency: value.currency, zone: value.zone }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct SalePricingItem {
        pub price: f32,
        pub currency: String,
        pub zone: String,
        pub start: Option<chrono::DateTime<chrono::Utc>>,
        pub end: Option<chrono::DateTime<chrono::Utc>>,
    }

    impl SalePricingItem {
        fn key(
            &self,
        ) -> (
            String,
            String,
            Option<chrono::DateTime<chrono::Utc>>,
            Option<chrono::DateTime<chrono::Utc>>,
        ) {
            (self.zone.clone(), self.currency.clone(), self.start, self.end)
        }
    }

    impl From<bindings::exports::golem::pricing_exports::api::SalePricingItem> for SalePricingItem {
        fn from(value: bindings::exports::golem::pricing_exports::api::SalePricingItem) -> Self {
            Self {
                price: value.price,
                currency: value.currency,
                zone: value.zone,
                start: value.start.map(|v| v.into()),
                end: value.end.map(|v| v.into()),
            }
        }
    }

    impl From<SalePricingItem> for bindings::exports::golem::pricing_exports::api::SalePricingItem {
        fn from(value: SalePricingItem) -> Self {
            Self {
                price: value.price,
                currency: value.currency,
                zone: value.zone,
                start: value.start.map(|v| v.into()),
                end: value.end.map(|v| v.into()),
            }
        }
    }

    fn get_price(currency: String, zone: String, pricing: Pricing) -> Option<PricingItem> {
        let now = chrono::Utc::now();

        let sale_price = pricing.sale_prices.into_iter().find(|x| {
            x.zone == zone
                && x.currency == currency
                && x.start.is_none_or(|v| now >= v)
                && x.end.is_none_or(|v| now < v)
        });

        if sale_price.is_some() {
            sale_price.map(|p| p.into())
        } else {
            let list_price =
                pricing.list_prices.into_iter().find(|x| x.zone == zone && x.currency == currency);

            if list_price.is_some() {
                list_price
            } else {
                pricing.msrp_prices.into_iter().find(|x| x.zone == zone && x.currency == currency)
            }
        }
    }

    fn merge_items(updates: Vec<PricingItem>, current: Vec<PricingItem>) -> Vec<PricingItem> {
        if updates.is_empty() {
            current
        } else if current.is_empty() {
            updates
        } else {
            let mut merge_map: HashMap<(String, String), PricingItem> = HashMap::new();

            for item in updates {
                merge_map.insert(item.key(), item);
            }

            for item in current {
                let key = item.key();
                if !merge_map.contains_key(&key) {
                    merge_map.insert(key, item);
                }
            }

            merge_map.into_values().collect()
        }
    }

    fn merge_sale_items(
        updates: Vec<SalePricingItem>,
        current: Vec<SalePricingItem>,
    ) -> Vec<SalePricingItem> {
        if updates.is_empty() {
            current
        } else if current.is_empty() {
            updates
        } else {
            let mut merge_map: HashMap<
                (
                    String,
                    String,
                    Option<chrono::DateTime<chrono::Utc>>,
                    Option<chrono::DateTime<chrono::Utc>>,
                ),
                SalePricingItem,
            > = HashMap::new();

            for item in updates {
                merge_map.insert(item.key(), item);
            }

            for item in current {
                let key = item.key();
                if !merge_map.contains_key(&key) {
                    merge_map.insert(key, item);
                }
            }

            let mut values: Vec<SalePricingItem> = merge_map.into_values().collect();
            values.sort_by(|a, b| match (a.start, b.start) {
                (Some(a), Some(b)) => a.cmp(&b),
                (Some(_), None) => std::cmp::Ordering::Greater,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (None, None) => std::cmp::Ordering::Equal,
            });
            values
        }
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
        use crate::domain::pricing::Pricing;

        pub const SERIALIZATION_VERSION_V1: u8 = 1u8;

        pub fn serialize(value: &Pricing) -> Result<Vec<u8>, String> {
            let data = serde_json::to_vec_pretty(value).map_err(|err| err.to_string())?;

            let mut result = vec![SERIALIZATION_VERSION_V1];
            result.extend(data);

            Ok(result)
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Pricing, String> {
            let (version, data) = bytes.split_at(1);

            match version[0] {
                SERIALIZATION_VERSION_V1 => {
                    let value: Pricing =
                        serde_json::from_slice(data).map_err(|err| err.to_string())?;

                    Ok(value)
                }
                _ => Err("Unsupported serialization version".to_string()),
            }
        }
    }
}

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
    }

    impl Pricing {
        pub fn new(product_id: String) -> Self {
            Self { product_id, msrp_prices: vec![], list_prices: vec![] }
        }

        pub fn get_price(&self, currency: String, zone: String) -> Option<PricingItem> {
            get_price(currency, zone, self.clone())
        }

        pub fn update_prices(
            &mut self,
            msrp_prices: Vec<PricingItem>,
            list_prices: Vec<PricingItem>,
        ) {
            self.msrp_prices = merge_items(msrp_prices, self.msrp_prices.clone());
            self.list_prices = merge_items(list_prices, self.list_prices.clone());
        }
    }

    impl From<bindings::exports::golem::pricing::api::Pricing> for Pricing {
        fn from(value: bindings::exports::golem::pricing::api::Pricing) -> Self {
            Self {
                product_id: value.product_id,
                msrp_prices: value.msrp_prices.into_iter().map(|item| item.into()).collect(),
                list_prices: value.list_prices.into_iter().map(|item| item.into()).collect(),
            }
        }
    }

    impl From<Pricing> for bindings::exports::golem::pricing::api::Pricing {
        fn from(value: Pricing) -> Self {
            Self {
                product_id: value.product_id,
                msrp_prices: value.msrp_prices.into_iter().map(|item| item.into()).collect(),
                list_prices: value.list_prices.into_iter().map(|item| item.into()).collect(),
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

    impl From<bindings::exports::golem::pricing::api::PricingItem> for PricingItem {
        fn from(value: bindings::exports::golem::pricing::api::PricingItem) -> Self {
            Self { price: value.price, currency: value.currency, zone: value.zone }
        }
    }

    impl From<PricingItem> for bindings::exports::golem::pricing::api::PricingItem {
        fn from(value: PricingItem) -> Self {
            Self { price: value.price, currency: value.currency, zone: value.zone }
        }
    }

    fn get_price(currency: String, zone: String, pricing: Pricing) -> Option<PricingItem> {
        let list_price =
            pricing.list_prices.into_iter().find(|x| x.zone == zone && x.currency == currency);

        if list_price.is_some() {
            list_price
        } else {
            pricing.msrp_prices.into_iter().find(|x| x.zone == zone && x.currency == currency)
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
                merge_map.insert((item.zone.clone(), item.currency.clone()), item);
            }

            for item in current {
                if !merge_map.contains_key(&(item.zone.clone(), item.currency.clone())) {
                    merge_map.insert((item.zone.clone(), item.currency.clone()), item);
                }
            }

            merge_map.into_values().collect()
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

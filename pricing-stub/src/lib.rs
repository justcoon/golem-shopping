#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureGetPriceResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::pricing_stub::stub_pricing::Guest for Component {
    type Api = crate::Api;
    type FutureGetPriceResult = crate::FutureGetPriceResult;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::pricing_stub::stub_pricing::GuestFutureGetPriceResult
for FutureGetPriceResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Option<crate::bindings::golem::pricing::api::PricingItem>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}", "golem:pricing/api.{get-price}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .option()
                    .expect("option not found")
                    .map(|inner| {
                        let record = inner;
                        crate::bindings::golem::pricing::api::PricingItem {
                            price: record
                                .field(0usize)
                                .expect("record field not found")
                                .f32()
                                .expect("f32 not found"),
                            currency: record
                                .field(1usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            zone: record
                                .field(2usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                        }
                    }))
            })
    }
}
impl crate::bindings::exports::golem::pricing_stub::stub_pricing::GuestFutureGetResult
for FutureGetResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Option<crate::bindings::golem::pricing::api::Pricing>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!("Failed to invoke remote {}", "golem:pricing/api.{get}"),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .option()
                    .expect("option not found")
                    .map(|inner| {
                        let record = inner;
                        crate::bindings::golem::pricing::api::Pricing {
                            asset_id: record
                                .field(0usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            msrp_prices: record
                                .field(1usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let record = item;
                                    crate::bindings::golem::pricing::api::PricingItem {
                                        price: record
                                            .field(0usize)
                                            .expect("record field not found")
                                            .f32()
                                            .expect("f32 not found"),
                                        currency: record
                                            .field(1usize)
                                            .expect("record field not found")
                                            .string()
                                            .expect("string not found")
                                            .to_string(),
                                        zone: record
                                            .field(2usize)
                                            .expect("record field not found")
                                            .string()
                                            .expect("string not found")
                                            .to_string(),
                                    }
                                })
                                .expect("list not found"),
                            list_prices: record
                                .field(2usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let record = item;
                                    crate::bindings::golem::pricing::api::PricingItem {
                                        price: record
                                            .field(0usize)
                                            .expect("record field not found")
                                            .f32()
                                            .expect("f32 not found"),
                                        currency: record
                                            .field(1usize)
                                            .expect("record field not found")
                                            .string()
                                            .expect("string not found")
                                            .to_string(),
                                        zone: record
                                            .field(2usize)
                                            .expect("record field not found")
                                            .string()
                                            .expect("string not found")
                                            .to_string(),
                                    }
                                })
                                .expect("list not found"),
                        }
                    }))
            })
    }
}
impl crate::bindings::exports::golem::pricing_stub::stub_pricing::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_initialize_pricing(
        &self,
        msrp_prices: Vec<crate::bindings::golem::pricing::api::PricingItem>,
        list_prices: Vec<crate::bindings::golem::pricing::api::PricingItem>,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:pricing/api.{initialize-pricing}",
                &[
                    WitValue::builder()
                        .list_fn(
                            &msrp_prices,
                            |item, item_builder| {
                                item_builder
                                    .record()
                                    .item()
                                    .f32(item.price)
                                    .item()
                                    .string(&item.currency)
                                    .item()
                                    .string(&item.zone)
                                    .finish()
                            },
                        ),
                    WitValue::builder()
                        .list_fn(
                            &list_prices,
                            |item, item_builder| {
                                item_builder
                                    .record()
                                    .item()
                                    .f32(item.price)
                                    .item()
                                    .string(&item.currency)
                                    .item()
                                    .string(&item.zone)
                                    .finish()
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:pricing/api.{initialize-pricing}"
                ),
            );
        ()
    }
    fn initialize_pricing(
        &self,
        msrp_prices: Vec<crate::bindings::golem::pricing::api::PricingItem>,
        list_prices: Vec<crate::bindings::golem::pricing::api::PricingItem>,
    ) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:pricing/api.{initialize-pricing}",
                &[
                    WitValue::builder()
                        .list_fn(
                            &msrp_prices,
                            |item, item_builder| {
                                item_builder
                                    .record()
                                    .item()
                                    .f32(item.price)
                                    .item()
                                    .string(&item.currency)
                                    .item()
                                    .string(&item.zone)
                                    .finish()
                            },
                        ),
                    WitValue::builder()
                        .list_fn(
                            &list_prices,
                            |item, item_builder| {
                                item_builder
                                    .record()
                                    .item()
                                    .f32(item.price)
                                    .item()
                                    .string(&item.currency)
                                    .item()
                                    .string(&item.zone)
                                    .finish()
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:pricing/api.{initialize-pricing}"
                ),
            );
        ()
    }
    fn blocking_get_price(
        &self,
        currency: String,
        zone: String,
    ) -> Option<crate::bindings::golem::pricing::api::PricingItem> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:pricing/api.{get-price}",
                &[
                    WitValue::builder().string(&currency),
                    WitValue::builder().string(&zone),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:pricing/api.{get-price}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .option()
            .expect("option not found")
            .map(|inner| {
                let record = inner;
                crate::bindings::golem::pricing::api::PricingItem {
                    price: record
                        .field(0usize)
                        .expect("record field not found")
                        .f32()
                        .expect("f32 not found"),
                    currency: record
                        .field(1usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    zone: record
                        .field(2usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                }
            }))
    }
    fn get_price(
        &self,
        currency: String,
        zone: String,
    ) -> crate::bindings::exports::golem::pricing_stub::stub_pricing::FutureGetPriceResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "golem:pricing/api.{get-price}",
                &[
                    WitValue::builder().string(&currency),
                    WitValue::builder().string(&zone),
                ],
            );
        crate::bindings::exports::golem::pricing_stub::stub_pricing::FutureGetPriceResult::new(FutureGetPriceResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get(&self) -> Option<crate::bindings::golem::pricing::api::Pricing> {
        let result = self
            .rpc
            .invoke_and_await("golem:pricing/api.{get}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "golem:pricing/api.{get}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .option()
            .expect("option not found")
            .map(|inner| {
                let record = inner;
                crate::bindings::golem::pricing::api::Pricing {
                    asset_id: record
                        .field(0usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    msrp_prices: record
                        .field(1usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            let record = item;
                            crate::bindings::golem::pricing::api::PricingItem {
                                price: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .f32()
                                    .expect("f32 not found"),
                                currency: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                zone: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        })
                        .expect("list not found"),
                    list_prices: record
                        .field(2usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            let record = item;
                            crate::bindings::golem::pricing::api::PricingItem {
                                price: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .f32()
                                    .expect("f32 not found"),
                                currency: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                zone: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        })
                        .expect("list not found"),
                }
            }))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::golem::pricing_stub::stub_pricing::FutureGetResult {
        let result = self.rpc.async_invoke_and_await("golem:pricing/api.{get}", &[]);
        crate::bindings::exports::golem::pricing_stub::stub_pricing::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);

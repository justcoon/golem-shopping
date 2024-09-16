#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::shopping_product_stub::stub_shopping_product::Guest
for Component {
    type Api = crate::Api;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::shopping_product_stub::stub_shopping_product::GuestFutureGetResult
for FutureGetResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(
        &self,
    ) -> Option<Option<crate::bindings::golem::shopping_product::api::Product>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:shopping-product/api.{get}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .option()
                    .expect("option not found")
                    .map(|inner| {
                        let record = inner;
                        crate::bindings::golem::shopping_product::api::Product {
                            product_id: record
                                .field(0usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            name: record
                                .field(1usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            description: record
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
impl crate::bindings::exports::golem::shopping_product_stub::stub_shopping_product::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_initialize_product(&self, name: String, description: String) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:shopping-product/api.{initialize-product}",
                &[
                    WitValue::builder().string(&name),
                    WitValue::builder().string(&description),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:shopping-product/api.{initialize-product}"
                ),
            );
        ()
    }
    fn initialize_product(&self, name: String, description: String) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:shopping-product/api.{initialize-product}",
                &[
                    WitValue::builder().string(&name),
                    WitValue::builder().string(&description),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:shopping-product/api.{initialize-product}"
                ),
            );
        ()
    }
    fn blocking_get(
        &self,
    ) -> Option<crate::bindings::golem::shopping_product::api::Product> {
        let result = self
            .rpc
            .invoke_and_await("golem:shopping-product/api.{get}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:shopping-product/api.{get}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .option()
            .expect("option not found")
            .map(|inner| {
                let record = inner;
                crate::bindings::golem::shopping_product::api::Product {
                    product_id: record
                        .field(0usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    name: record
                        .field(1usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    description: record
                        .field(2usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                }
            }))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::golem::shopping_product_stub::stub_shopping_product::FutureGetResult {
        let result = self
            .rpc
            .async_invoke_and_await("golem:shopping-product/api.{get}", &[]);
        crate::bindings::exports::golem::shopping_product_stub::stub_shopping_product::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);

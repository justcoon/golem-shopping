#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct SaveSnapshot {
    rpc: WasmRpc,
}
impl SaveSnapshot {}
pub struct FutureSaveResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct LoadSnapshot {
    rpc: WasmRpc,
}
impl LoadSnapshot {}
pub struct FutureLoadResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::product_stub::stub_product::Guest for Component {
    type SaveSnapshot = crate::SaveSnapshot;
    type FutureSaveResult = crate::FutureSaveResult;
    type LoadSnapshot = crate::LoadSnapshot;
    type FutureLoadResult = crate::FutureLoadResult;
    type Api = crate::Api;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::product_stub::stub_product::GuestFutureSaveResult
for FutureSaveResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Vec<u8>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:product/save-snapshot.{save}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .list_elements(|item| item.u8().expect("u8 not found"))
                    .expect("list not found"))
            })
    }
}
impl crate::bindings::exports::golem::product_stub::stub_product::GuestSaveSnapshot
for SaveSnapshot {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_save(&self) -> Vec<u8> {
        let result = self
            .rpc
            .invoke_and_await("golem:product/save-snapshot.{save}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:product/save-snapshot.{save}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| item.u8().expect("u8 not found"))
            .expect("list not found"))
    }
    fn save(
        &self,
    ) -> crate::bindings::exports::golem::product_stub::stub_product::FutureSaveResult {
        let result = self
            .rpc
            .async_invoke_and_await("golem:product/save-snapshot.{save}", &[]);
        crate::bindings::exports::golem::product_stub::stub_product::FutureSaveResult::new(FutureSaveResult {
            future_invoke_result: result,
        })
    }
}
impl crate::bindings::exports::golem::product_stub::stub_product::GuestFutureLoadResult
for FutureLoadResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<(), String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:product/load-snapshot.{load}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => Ok(()),
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::product_stub::stub_product::GuestLoadSnapshot
for LoadSnapshot {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_load(&self, bytes: Vec<u8>) -> Result<(), String> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:product/load-snapshot.{load}",
                &[
                    WitValue::builder()
                        .list_fn(&bytes, |item, item_builder| { item_builder.u8(*item) }),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:product/load-snapshot.{load}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn load(
        &self,
        bytes: Vec<u8>,
    ) -> crate::bindings::exports::golem::product_stub::stub_product::FutureLoadResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "golem:product/load-snapshot.{load}",
                &[
                    WitValue::builder()
                        .list_fn(&bytes, |item, item_builder| { item_builder.u8(*item) }),
                ],
            );
        crate::bindings::exports::golem::product_stub::stub_product::FutureLoadResult::new(FutureLoadResult {
            future_invoke_result: result,
        })
    }
}
impl crate::bindings::exports::golem::product_stub::stub_product::GuestFutureGetResult
for FutureGetResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Option<crate::bindings::golem::product::api::Product>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!("Failed to invoke remote {}", "golem:product/api.{get}"),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .option()
                    .expect("option not found")
                    .map(|inner| {
                        let record = inner;
                        crate::bindings::golem::product::api::Product {
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
                            tags: record
                                .field(3usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    item.string().expect("string not found").to_string()
                                })
                                .expect("list not found"),
                        }
                    }))
            })
    }
}
impl crate::bindings::exports::golem::product_stub::stub_product::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_initialize_product(
        &self,
        name: String,
        description: String,
        tags: Vec<String>,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:product/api.{initialize-product}",
                &[
                    WitValue::builder().string(&name),
                    WitValue::builder().string(&description),
                    WitValue::builder()
                        .list_fn(
                            &tags,
                            |item, item_builder| { item_builder.string(item) },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:product/api.{initialize-product}"
                ),
            );
        ()
    }
    fn initialize_product(
        &self,
        name: String,
        description: String,
        tags: Vec<String>,
    ) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:product/api.{initialize-product}",
                &[
                    WitValue::builder().string(&name),
                    WitValue::builder().string(&description),
                    WitValue::builder()
                        .list_fn(
                            &tags,
                            |item, item_builder| { item_builder.string(item) },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:product/api.{initialize-product}"
                ),
            );
        ()
    }
    fn blocking_get(&self) -> Option<crate::bindings::golem::product::api::Product> {
        let result = self
            .rpc
            .invoke_and_await("golem:product/api.{get}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "golem:product/api.{get}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .option()
            .expect("option not found")
            .map(|inner| {
                let record = inner;
                crate::bindings::golem::product::api::Product {
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
                    tags: record
                        .field(3usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            item.string().expect("string not found").to_string()
                        })
                        .expect("list not found"),
                }
            }))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::golem::product_stub::stub_product::FutureGetResult {
        let result = self.rpc.async_invoke_and_await("golem:product/api.{get}", &[]);
        crate::bindings::exports::golem::product_stub::stub_product::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);

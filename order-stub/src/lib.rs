#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureAddItemResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureRemoveItemResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUpdateItemQuantityResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::Guest
    for Component
{
    type Api = crate::Api;
    type FutureAddItemResult = crate::FutureAddItemResult;
    type FutureRemoveItemResult = crate::FutureRemoveItemResult;
    type FutureUpdateItemQuantityResult = crate::FutureUpdateItemQuantityResult;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::GuestFutureAddItemResult
for FutureAddItemResult {
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
                            "golem:shopping-order/api.{add-item}"
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
impl crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::GuestFutureRemoveItemResult
for FutureRemoveItemResult {
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
                            "golem:shopping-order/api.{remove-item}"
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
impl crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::GuestFutureUpdateItemQuantityResult
for FutureUpdateItemQuantityResult {
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
                            "golem:shopping-order/api.{update-item-quantity}"
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
impl crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::GuestFutureGetResult
    for FutureGetResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Option<crate::bindings::golem::shopping_order::api::Order>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:shopping-order/api.{get}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .option()
                    .expect("option not found")
                    .map(|inner| {
                        let record = inner;
                        crate::bindings::golem::shopping_order::api::Order {
                            order_id: record
                                .field(0usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            user_id: record
                                .field(1usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            order_status: {
                                let case_idx = record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .enum_value()
                                    .expect("enum not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::shopping_order::api::OrderStatus::New
                                    }
                                    1u32 => {
                                        crate::bindings::golem::shopping_order::api::OrderStatus::Confirmed
                                    }
                                    2u32 => {
                                        crate::bindings::golem::shopping_order::api::OrderStatus::Cancelled
                                    }
                                    _ => unreachable!("invalid enum case index"),
                                }
                            },
                            items: record
                                .field(3usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let record = item;
                                    crate::bindings::golem::shopping_order::api::OrderItem {
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
                                        price: record
                                            .field(2usize)
                                            .expect("record field not found")
                                            .f32()
                                            .expect("f32 not found"),
                                        quantity: record
                                            .field(3usize)
                                            .expect("record field not found")
                                            .u32()
                                            .expect("u32 not found"),
                                    }
                                })
                                .expect("list not found"),
                            total: record
                                .field(4usize)
                                .expect("record field not found")
                                .f32()
                                .expect("f32 not found"),
                            currency: record
                                .field(5usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            timestamp: record
                                .field(6usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                        }
                    }))
            })
    }
}
impl crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri { value: location.value };
        Self { rpc: WasmRpc::new(&location) }
    }
    fn blocking_initialize_order(
        &self,
        data: crate::bindings::golem::shopping_order::api::CreateOrder,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:shopping-order/api.{initialize-order}",
                &[WitValue::builder()
                    .record()
                    .item()
                    .string(&data.user_id)
                    .item()
                    .list_fn(&data.items, |item, item_builder| {
                        item_builder
                            .record()
                            .item()
                            .string(&item.product_id)
                            .item()
                            .string(&item.name)
                            .item()
                            .f32(item.price)
                            .item()
                            .u32(item.quantity)
                            .finish()
                    })
                    .item()
                    .f32(data.total)
                    .item()
                    .string(&data.currency)
                    .item()
                    .u64(data.timestamp)
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:shopping-order/api.{initialize-order}"
            ));
        ()
    }
    fn initialize_order(
        &self,
        data: crate::bindings::golem::shopping_order::api::CreateOrder,
    ) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:shopping-order/api.{initialize-order}",
                &[WitValue::builder()
                    .record()
                    .item()
                    .string(&data.user_id)
                    .item()
                    .list_fn(&data.items, |item, item_builder| {
                        item_builder
                            .record()
                            .item()
                            .string(&item.product_id)
                            .item()
                            .string(&item.name)
                            .item()
                            .f32(item.price)
                            .item()
                            .u32(item.quantity)
                            .finish()
                    })
                    .item()
                    .f32(data.total)
                    .item()
                    .string(&data.currency)
                    .item()
                    .u64(data.timestamp)
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "golem:shopping-order/api.{initialize-order}"
            ));
        ()
    }
    fn blocking_add_item(&self, product_id: String, quantity: u32) -> Result<(), String> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:shopping-order/api.{add-item}",
                &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:shopping-order/api.{add-item}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn add_item(
        &self,
        product_id: String,
        quantity: u32,
    ) -> crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureAddItemResult{
        let result = self.rpc.async_invoke_and_await(
            "golem:shopping-order/api.{add-item}",
            &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
        );
        crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureAddItemResult::new(FutureAddItemResult {
            future_invoke_result: result,
        })
    }
    fn blocking_remove_item(&self, product_id: String) -> Result<(), String> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:shopping-order/api.{remove-item}",
                &[WitValue::builder().string(&product_id)],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:shopping-order/api.{remove-item}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn remove_item(
        &self,
        product_id: String,
    ) -> crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureRemoveItemResult{
        let result = self.rpc.async_invoke_and_await(
            "golem:shopping-order/api.{remove-item}",
            &[WitValue::builder().string(&product_id)],
        );
        crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureRemoveItemResult::new(FutureRemoveItemResult {
            future_invoke_result: result,
        })
    }
    fn blocking_update_item_quantity(
        &self,
        product_id: String,
        quantity: u32,
    ) -> Result<(), String> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:shopping-order/api.{update-item-quantity}",
                &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:shopping-order/api.{update-item-quantity}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn update_item_quantity(
        &self,
        product_id: String,
        quantity: u32,
    ) -> crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureUpdateItemQuantityResult{
        let result = self.rpc.async_invoke_and_await(
            "golem:shopping-order/api.{update-item-quantity}",
            &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
        );
        crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureUpdateItemQuantityResult::new(FutureUpdateItemQuantityResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get(&self) -> Option<crate::bindings::golem::shopping_order::api::Order> {
        let result = self.rpc.invoke_and_await("golem:shopping-order/api.{get}", &[]).expect(
            &format!("Failed to invoke-and-await remote {}", "golem:shopping-order/api.{get}"),
        );
        (result.tuple_element(0).expect("tuple not found").option().expect("option not found").map(
            |inner| {
                let record = inner;
                crate::bindings::golem::shopping_order::api::Order {
                    order_id: record
                        .field(0usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    user_id: record
                        .field(1usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    order_status: {
                        let case_idx = record
                            .field(2usize)
                            .expect("record field not found")
                            .enum_value()
                            .expect("enum not found");
                        match case_idx {
                            0u32 => crate::bindings::golem::shopping_order::api::OrderStatus::New,
                            1u32 => {
                                crate::bindings::golem::shopping_order::api::OrderStatus::Confirmed
                            }
                            2u32 => {
                                crate::bindings::golem::shopping_order::api::OrderStatus::Cancelled
                            }
                            _ => unreachable!("invalid enum case index"),
                        }
                    },
                    items: record
                        .field(3usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            let record = item;
                            crate::bindings::golem::shopping_order::api::OrderItem {
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
                                price: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .f32()
                                    .expect("f32 not found"),
                                quantity: record
                                    .field(3usize)
                                    .expect("record field not found")
                                    .u32()
                                    .expect("u32 not found"),
                            }
                        })
                        .expect("list not found"),
                    total: record
                        .field(4usize)
                        .expect("record field not found")
                        .f32()
                        .expect("f32 not found"),
                    currency: record
                        .field(5usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    timestamp: record
                        .field(6usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                }
            },
        ))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureGetResult
    {
        let result = self.rpc.async_invoke_and_await("golem:shopping-order/api.{get}", &[]);
        crate::bindings::exports::golem::shopping_order_stub::stub_shopping_order::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);

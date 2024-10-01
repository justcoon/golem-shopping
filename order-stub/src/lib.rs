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
pub struct FutureAddItemResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureRemoveItemResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUpdateItemQuantityResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUpdateShippingAddressResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUpdateBillingAddressResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureShipOrderResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureCancelOrderResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::order_stub::stub_order::Guest for Component {
    type SaveSnapshot = crate::SaveSnapshot;
    type FutureSaveResult = crate::FutureSaveResult;
    type LoadSnapshot = crate::LoadSnapshot;
    type FutureLoadResult = crate::FutureLoadResult;
    type Api = crate::Api;
    type FutureAddItemResult = crate::FutureAddItemResult;
    type FutureRemoveItemResult = crate::FutureRemoveItemResult;
    type FutureUpdateItemQuantityResult = crate::FutureUpdateItemQuantityResult;
    type FutureUpdateShippingAddressResult = crate::FutureUpdateShippingAddressResult;
    type FutureUpdateBillingAddressResult = crate::FutureUpdateBillingAddressResult;
    type FutureShipOrderResult = crate::FutureShipOrderResult;
    type FutureCancelOrderResult = crate::FutureCancelOrderResult;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureSaveResult
    for FutureSaveResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Vec<u8>> {
        self.future_invoke_result.get().map(|result| {
            let result = result
                .expect(&format!("Failed to invoke remote {}", "golem:order/save-snapshot.{save}"));
            (result
                .tuple_element(0)
                .expect("tuple not found")
                .list_elements(|item| item.u8().expect("u8 not found"))
                .expect("list not found"))
        })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestSaveSnapshot for SaveSnapshot {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri { value: location.value };
        Self { rpc: WasmRpc::new(&location) }
    }
    fn blocking_save(&self) -> Vec<u8> {
        let result = self.rpc.invoke_and_await("golem:order/save-snapshot.{save}", &[]).expect(
            &format!("Failed to invoke-and-await remote {}", "golem:order/save-snapshot.{save}"),
        );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| item.u8().expect("u8 not found"))
            .expect("list not found"))
    }
    fn save(&self) -> crate::bindings::exports::golem::order_stub::stub_order::FutureSaveResult {
        let result = self.rpc.async_invoke_and_await("golem:order/save-snapshot.{save}", &[]);
        crate::bindings::exports::golem::order_stub::stub_order::FutureSaveResult::new(
            FutureSaveResult { future_invoke_result: result },
        )
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureLoadResult
    for FutureLoadResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), String>> {
        self.future_invoke_result.get().map(|result| {
            let result = result
                .expect(&format!("Failed to invoke remote {}", "golem:order/load-snapshot.{load}"));
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
        })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestLoadSnapshot for LoadSnapshot {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri { value: location.value };
        Self { rpc: WasmRpc::new(&location) }
    }
    fn blocking_load(&self, bytes: Vec<u8>) -> Result<(), String> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/load-snapshot.{load}",
                &[WitValue::builder().list_fn(&bytes, |item, item_builder| item_builder.u8(*item))],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:order/load-snapshot.{load}"
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
    fn load(
        &self,
        bytes: Vec<u8>,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureLoadResult {
        let result = self.rpc.async_invoke_and_await(
            "golem:order/load-snapshot.{load}",
            &[WitValue::builder().list_fn(&bytes, |item, item_builder| item_builder.u8(*item))],
        );
        crate::bindings::exports::golem::order_stub::stub_order::FutureLoadResult::new(
            FutureLoadResult { future_invoke_result: result },
        )
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureAddItemResult
    for FutureAddItemResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}", "golem:order/api.{add-item}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureRemoveItemResult
    for FutureRemoveItemResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}", "golem:order/api.{remove-item}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureUpdateItemQuantityResult
    for FutureUpdateItemQuantityResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:order/api.{update-item-quantity}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureUpdateShippingAddressResult
    for FutureUpdateShippingAddressResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:order/api.{update-shipping-address}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureUpdateBillingAddressResult
    for FutureUpdateBillingAddressResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:order/api.{update-billing-address}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureShipOrderResult
    for FutureShipOrderResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}", "golem:order/api.{ship-order}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureCancelOrderResult
    for FutureCancelOrderResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Result<(), crate::bindings::golem::order::api::Error>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:order/api.{cancel-order}"
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
                            Err({
                                let (case_idx, inner) = err_value
                                    .expect("result err value not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::golem::order::api::Error::ProductNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ProductNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::golem::order::api::Error::PricingNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::PricingNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    2u32 => {
                                        crate::bindings::golem::order::api::Error::AddressNotValid({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::AddressNotValidError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    3u32 => {
                                        crate::bindings::golem::order::api::Error::ItemNotFound({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ItemNotFoundError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                product_id: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    4u32 => {
                                        crate::bindings::golem::order::api::Error::EmptyItems({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::EmptyItemsError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                            }
                                        })
                                    }
                                    5u32 => {
                                        crate::bindings::golem::order::api::Error::ActionNotAllowed({
                                            let record = inner.expect("variant case not found");
                                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                                message: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .string()
                                                    .expect("string not found")
                                                    .to_string(),
                                                status: {
                                                    let case_idx = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .enum_value()
                                                        .expect("enum not found");
                                                    match case_idx {
                                                        0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                        1u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Shipped
                                                        }
                                                        2u32 => {
                                                            crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                        }
                                                        _ => unreachable!("invalid enum case index"),
                                                    }
                                                },
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            })
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestFutureGetResult
    for FutureGetResult
{
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable =
            unsafe { bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle()) };
        pollable
    }
    fn get(&self) -> Option<Option<crate::bindings::golem::order::api::Order>> {
        self.future_invoke_result.get().map(|result| {
            let result =
                result.expect(&format!("Failed to invoke remote {}", "golem:order/api.{get}"));
            (result
                .tuple_element(0)
                .expect("tuple not found")
                .option()
                .expect("option not found")
                .map(|inner| {
                    let record = inner;
                    crate::bindings::golem::order::api::Order {
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
                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                1u32 => crate::bindings::golem::order::api::OrderStatus::Shipped,
                                2u32 => crate::bindings::golem::order::api::OrderStatus::Cancelled,
                                _ => unreachable!("invalid enum case index"),
                            }
                        },
                        items: record
                            .field(3usize)
                            .expect("record field not found")
                            .list_elements(|item| {
                                let record = item;
                                crate::bindings::golem::order::api::OrderItem {
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
                        billing_address: record
                            .field(4usize)
                            .expect("record field not found")
                            .option()
                            .expect("option not found")
                            .map(|inner| {
                                let record = inner;
                                crate::bindings::golem::order::api::Address {
                                    street1: record
                                        .field(0usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    street2: record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                    city: record
                                        .field(2usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    state_or_region: record
                                        .field(3usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    country: record
                                        .field(4usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    postal_code: record
                                        .field(5usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    name: record
                                        .field(6usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                    business_name: record
                                        .field(7usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                    phone_number: record
                                        .field(8usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                }
                            }),
                        shipping_address: record
                            .field(5usize)
                            .expect("record field not found")
                            .option()
                            .expect("option not found")
                            .map(|inner| {
                                let record = inner;
                                crate::bindings::golem::order::api::Address {
                                    street1: record
                                        .field(0usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    street2: record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                    city: record
                                        .field(2usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    state_or_region: record
                                        .field(3usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    country: record
                                        .field(4usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    postal_code: record
                                        .field(5usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    name: record
                                        .field(6usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                    business_name: record
                                        .field(7usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                    phone_number: record
                                        .field(8usize)
                                        .expect("record field not found")
                                        .option()
                                        .expect("option not found")
                                        .map(|inner| {
                                            inner.string().expect("string not found").to_string()
                                        }),
                                }
                            }),
                        total: record
                            .field(6usize)
                            .expect("record field not found")
                            .f32()
                            .expect("f32 not found"),
                        currency: record
                            .field(7usize)
                            .expect("record field not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                        timestamp: record
                            .field(8usize)
                            .expect("record field not found")
                            .u64()
                            .expect("u64 not found"),
                    }
                }))
        })
    }
}
impl crate::bindings::exports::golem::order_stub::stub_order::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri { value: location.value };
        Self { rpc: WasmRpc::new(&location) }
    }
    fn blocking_initialize_order(
        &self,
        data: crate::bindings::golem::order::api::CreateOrder,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/api.{initialize-order}",
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
                    .option_fn(data.billing_address.is_some(), |some_builder| {
                        some_builder
                            .record()
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().street1)
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().street2.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .street2
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().city)
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().state_or_region)
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().country)
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().postal_code)
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().business_name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .business_name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().phone_number.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .phone_number
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .finish()
                    })
                    .item()
                    .option_fn(data.shipping_address.is_some(), |some_builder| {
                        some_builder
                            .record()
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().street1)
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().street2.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .street2
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().city)
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().state_or_region)
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().country)
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().postal_code)
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().business_name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .business_name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().phone_number.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .phone_number
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
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
                "golem:order/api.{initialize-order}"
            ));
        ()
    }
    fn initialize_order(&self, data: crate::bindings::golem::order::api::CreateOrder) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:order/api.{initialize-order}",
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
                    .option_fn(data.billing_address.is_some(), |some_builder| {
                        some_builder
                            .record()
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().street1)
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().street2.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .street2
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().city)
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().state_or_region)
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().country)
                            .item()
                            .string(&data.billing_address.as_ref().unwrap().postal_code)
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().business_name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .business_name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.billing_address.as_ref().unwrap().phone_number.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.billing_address
                                            .as_ref()
                                            .unwrap()
                                            .phone_number
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .finish()
                    })
                    .item()
                    .option_fn(data.shipping_address.is_some(), |some_builder| {
                        some_builder
                            .record()
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().street1)
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().street2.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .street2
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().city)
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().state_or_region)
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().country)
                            .item()
                            .string(&data.shipping_address.as_ref().unwrap().postal_code)
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().business_name.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .business_name
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
                            .item()
                            .option_fn(
                                data.shipping_address.as_ref().unwrap().phone_number.is_some(),
                                |some_builder| {
                                    some_builder.string(
                                        data.shipping_address
                                            .as_ref()
                                            .unwrap()
                                            .phone_number
                                            .as_ref()
                                            .unwrap(),
                                    )
                                },
                            )
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
            .expect(&format!("Failed to invoke remote {}", "golem:order/api.{initialize-order}"));
        ()
    }
    fn blocking_add_item(
        &self,
        product_id: String,
        quantity: u32,
    ) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/api.{add-item}",
                &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
            )
            .expect(&format!("Failed to invoke-and-await remote {}", "golem:order/api.{add-item}"));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn add_item(
        &self,
        product_id: String,
        quantity: u32,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureAddItemResult {
        let result = self.rpc.async_invoke_and_await(
            "golem:order/api.{add-item}",
            &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
        );
        crate::bindings::exports::golem::order_stub::stub_order::FutureAddItemResult::new(
            FutureAddItemResult { future_invoke_result: result },
        )
    }
    fn blocking_remove_item(
        &self,
        product_id: String,
    ) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/api.{remove-item}",
                &[WitValue::builder().string(&product_id)],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:order/api.{remove-item}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn remove_item(
        &self,
        product_id: String,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureRemoveItemResult {
        let result = self.rpc.async_invoke_and_await(
            "golem:order/api.{remove-item}",
            &[WitValue::builder().string(&product_id)],
        );
        crate::bindings::exports::golem::order_stub::stub_order::FutureRemoveItemResult::new(
            FutureRemoveItemResult { future_invoke_result: result },
        )
    }
    fn blocking_update_item_quantity(
        &self,
        product_id: String,
        quantity: u32,
    ) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/api.{update-item-quantity}",
                &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:order/api.{update-item-quantity}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn update_item_quantity(
        &self,
        product_id: String,
        quantity: u32,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureUpdateItemQuantityResult
    {
        let result = self.rpc.async_invoke_and_await(
            "golem:order/api.{update-item-quantity}",
            &[WitValue::builder().string(&product_id), WitValue::builder().u32(quantity)],
        );
        crate::bindings::exports::golem::order_stub::stub_order::FutureUpdateItemQuantityResult::new(
            FutureUpdateItemQuantityResult { future_invoke_result: result },
        )
    }
    fn blocking_update_shipping_address(
        &self,
        address: crate::bindings::golem::order::api::Address,
    ) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/api.{update-shipping-address}",
                &[WitValue::builder()
                    .record()
                    .item()
                    .string(&address.street1)
                    .item()
                    .option_fn(address.street2.is_some(), |some_builder| {
                        some_builder.string(address.street2.as_ref().unwrap())
                    })
                    .item()
                    .string(&address.city)
                    .item()
                    .string(&address.state_or_region)
                    .item()
                    .string(&address.country)
                    .item()
                    .string(&address.postal_code)
                    .item()
                    .option_fn(address.name.is_some(), |some_builder| {
                        some_builder.string(address.name.as_ref().unwrap())
                    })
                    .item()
                    .option_fn(address.business_name.is_some(), |some_builder| {
                        some_builder.string(address.business_name.as_ref().unwrap())
                    })
                    .item()
                    .option_fn(address.phone_number.is_some(), |some_builder| {
                        some_builder.string(address.phone_number.as_ref().unwrap())
                    })
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:order/api.{update-shipping-address}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn update_shipping_address(
        &self,
        address: crate::bindings::golem::order::api::Address,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureUpdateShippingAddressResult
    {
        let result = self.rpc.async_invoke_and_await(
            "golem:order/api.{update-shipping-address}",
            &[WitValue::builder()
                .record()
                .item()
                .string(&address.street1)
                .item()
                .option_fn(address.street2.is_some(), |some_builder| {
                    some_builder.string(address.street2.as_ref().unwrap())
                })
                .item()
                .string(&address.city)
                .item()
                .string(&address.state_or_region)
                .item()
                .string(&address.country)
                .item()
                .string(&address.postal_code)
                .item()
                .option_fn(address.name.is_some(), |some_builder| {
                    some_builder.string(address.name.as_ref().unwrap())
                })
                .item()
                .option_fn(address.business_name.is_some(), |some_builder| {
                    some_builder.string(address.business_name.as_ref().unwrap())
                })
                .item()
                .option_fn(address.phone_number.is_some(), |some_builder| {
                    some_builder.string(address.phone_number.as_ref().unwrap())
                })
                .finish()],
        );
        crate::bindings::exports::golem::order_stub::stub_order::FutureUpdateShippingAddressResult::new(FutureUpdateShippingAddressResult {
            future_invoke_result: result,
        })
    }
    fn blocking_update_billing_address(
        &self,
        address: crate::bindings::golem::order::api::Address,
    ) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:order/api.{update-billing-address}",
                &[WitValue::builder()
                    .record()
                    .item()
                    .string(&address.street1)
                    .item()
                    .option_fn(address.street2.is_some(), |some_builder| {
                        some_builder.string(address.street2.as_ref().unwrap())
                    })
                    .item()
                    .string(&address.city)
                    .item()
                    .string(&address.state_or_region)
                    .item()
                    .string(&address.country)
                    .item()
                    .string(&address.postal_code)
                    .item()
                    .option_fn(address.name.is_some(), |some_builder| {
                        some_builder.string(address.name.as_ref().unwrap())
                    })
                    .item()
                    .option_fn(address.business_name.is_some(), |some_builder| {
                        some_builder.string(address.business_name.as_ref().unwrap())
                    })
                    .item()
                    .option_fn(address.phone_number.is_some(), |some_builder| {
                        some_builder.string(address.phone_number.as_ref().unwrap())
                    })
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke-and-await remote {}",
                "golem:order/api.{update-billing-address}"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn update_billing_address(
        &self,
        address: crate::bindings::golem::order::api::Address,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureUpdateBillingAddressResult
    {
        let result = self.rpc.async_invoke_and_await(
            "golem:order/api.{update-billing-address}",
            &[WitValue::builder()
                .record()
                .item()
                .string(&address.street1)
                .item()
                .option_fn(address.street2.is_some(), |some_builder| {
                    some_builder.string(address.street2.as_ref().unwrap())
                })
                .item()
                .string(&address.city)
                .item()
                .string(&address.state_or_region)
                .item()
                .string(&address.country)
                .item()
                .string(&address.postal_code)
                .item()
                .option_fn(address.name.is_some(), |some_builder| {
                    some_builder.string(address.name.as_ref().unwrap())
                })
                .item()
                .option_fn(address.business_name.is_some(), |some_builder| {
                    some_builder.string(address.business_name.as_ref().unwrap())
                })
                .item()
                .option_fn(address.phone_number.is_some(), |some_builder| {
                    some_builder.string(address.phone_number.as_ref().unwrap())
                })
                .finish()],
        );
        crate::bindings::exports::golem::order_stub::stub_order::FutureUpdateBillingAddressResult::new(FutureUpdateBillingAddressResult {
            future_invoke_result: result,
        })
    }
    fn blocking_ship_order(&self) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self.rpc.invoke_and_await("golem:order/api.{ship-order}", &[]).expect(
            &format!("Failed to invoke-and-await remote {}", "golem:order/api.{ship-order}"),
        );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn ship_order(
        &self,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureShipOrderResult {
        let result = self.rpc.async_invoke_and_await("golem:order/api.{ship-order}", &[]);
        crate::bindings::exports::golem::order_stub::stub_order::FutureShipOrderResult::new(
            FutureShipOrderResult { future_invoke_result: result },
        )
    }
    fn blocking_cancel_order(&self) -> Result<(), crate::bindings::golem::order::api::Error> {
        let result = self.rpc.invoke_and_await("golem:order/api.{cancel-order}", &[]).expect(
            &format!("Failed to invoke-and-await remote {}", "golem:order/api.{cancel-order}"),
        );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::golem::order::api::Error::ProductNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ProductNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        1u32 => crate::bindings::golem::order::api::Error::PricingNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::PricingNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        2u32 => crate::bindings::golem::order::api::Error::AddressNotValid({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::AddressNotValidError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        3u32 => crate::bindings::golem::order::api::Error::ItemNotFound({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ItemNotFoundError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                product_id: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        4u32 => crate::bindings::golem::order::api::Error::EmptyItems({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::EmptyItemsError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        }),
                        5u32 => crate::bindings::golem::order::api::Error::ActionNotAllowed({
                            let record = inner.expect("variant case not found");
                            crate::bindings::golem::order::api::ActionNotAllowedError {
                                message: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                status: {
                                    let case_idx = record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .enum_value()
                                        .expect("enum not found");
                                    match case_idx {
                                                0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                                                1u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Shipped
                                                }
                                                2u32 => {
                                                    crate::bindings::golem::order::api::OrderStatus::Cancelled
                                                }
                                                _ => unreachable!("invalid enum case index"),
                                            }
                                },
                            }
                        }),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn cancel_order(
        &self,
    ) -> crate::bindings::exports::golem::order_stub::stub_order::FutureCancelOrderResult {
        let result = self.rpc.async_invoke_and_await("golem:order/api.{cancel-order}", &[]);
        crate::bindings::exports::golem::order_stub::stub_order::FutureCancelOrderResult::new(
            FutureCancelOrderResult { future_invoke_result: result },
        )
    }
    fn blocking_get(&self) -> Option<crate::bindings::golem::order::api::Order> {
        let result = self
            .rpc
            .invoke_and_await("golem:order/api.{get}", &[])
            .expect(&format!("Failed to invoke-and-await remote {}", "golem:order/api.{get}"));
        (result.tuple_element(0).expect("tuple not found").option().expect("option not found").map(
            |inner| {
                let record = inner;
                crate::bindings::golem::order::api::Order {
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
                            0u32 => crate::bindings::golem::order::api::OrderStatus::New,
                            1u32 => crate::bindings::golem::order::api::OrderStatus::Shipped,
                            2u32 => crate::bindings::golem::order::api::OrderStatus::Cancelled,
                            _ => unreachable!("invalid enum case index"),
                        }
                    },
                    items: record
                        .field(3usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            let record = item;
                            crate::bindings::golem::order::api::OrderItem {
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
                    billing_address: record
                        .field(4usize)
                        .expect("record field not found")
                        .option()
                        .expect("option not found")
                        .map(|inner| {
                            let record = inner;
                            crate::bindings::golem::order::api::Address {
                                street1: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                street2: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                                city: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                state_or_region: record
                                    .field(3usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                country: record
                                    .field(4usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                postal_code: record
                                    .field(5usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                name: record
                                    .field(6usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                                business_name: record
                                    .field(7usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                                phone_number: record
                                    .field(8usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                            }
                        }),
                    shipping_address: record
                        .field(5usize)
                        .expect("record field not found")
                        .option()
                        .expect("option not found")
                        .map(|inner| {
                            let record = inner;
                            crate::bindings::golem::order::api::Address {
                                street1: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                street2: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                                city: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                state_or_region: record
                                    .field(3usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                country: record
                                    .field(4usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                postal_code: record
                                    .field(5usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                name: record
                                    .field(6usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                                business_name: record
                                    .field(7usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                                phone_number: record
                                    .field(8usize)
                                    .expect("record field not found")
                                    .option()
                                    .expect("option not found")
                                    .map(|inner| {
                                        inner.string().expect("string not found").to_string()
                                    }),
                            }
                        }),
                    total: record
                        .field(6usize)
                        .expect("record field not found")
                        .f32()
                        .expect("f32 not found"),
                    currency: record
                        .field(7usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    timestamp: record
                        .field(8usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                }
            },
        ))
    }
    fn get(&self) -> crate::bindings::exports::golem::order_stub::stub_order::FutureGetResult {
        let result = self.rpc.async_invoke_and_await("golem:order/api.{get}", &[]);
        crate::bindings::exports::golem::order_stub::stub_order::FutureGetResult::new(
            FutureGetResult { future_invoke_result: result },
        )
    }
}
bindings::export!(Component with_types_in bindings);

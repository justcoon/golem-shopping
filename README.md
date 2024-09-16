# golem-shopping

## Components and Workers

* pricing
  -worker - per product, worker name: id of product
  
* product
  - worker - per product, worker name: id of product
  
* cart 
  - worker - per user/customer (there is always only one cart per user), worker name: id of user/customer
  - dependencies: 
    - pricing, 
    - product, 
    - order
  - env:
    - PRODUCT_COMPONENT_ID
    - PRICING_COMPONENT_ID
    - ORDER_COMPONENT_ID
* order
  - worker - per order, worker name: id of order
  - dependencies:
      - pricing, 
      - product
  - env:
      - PRODUCT_COMPONENT_ID
      - PRICING_COMPONENT_ID
      - ORDER_COMPONENT_ID

## Commands

```
golem-cli stubgen initialize-workspace --targets order  --targets product --targets pricing --callers cart --callers order
```

```
cargo make regenerate-stubs
cargo make release-build-flow
```

```
golem-cloud-cli component add --component-name pricing target/wasm32-wasi/release/pricing.wasm
golem-cloud-cli component add --component-name product target/wasm32-wasi/release/product.wasm
golem-cloud-cli component add --component-name cart target/wasm32-wasi/release/cart_composed.wasm
golem-cloud-cli component add --component-name order target/wasm32-wasi/release/order_composed.wasm
```
# golem-shopping

## Components and Workers

* pricing
  - worker - per product, worker name: id of product
  
* product
  - worker - per product, worker name: id of product
  
* cart 
  - worker - per user/customer (there is always only one cart per user), worker name: id of user/customer
  - dependencies: 
    - pricing 
    - product 
    - order
  - env:
    - PRODUCT_COMPONENT_ID
    - PRICING_COMPONENT_ID
    - ORDER_COMPONENT_ID
* order
  - worker - per order, worker name: id of order
  - dependencies:
      - pricing 
      - product
  - env:
      - PRODUCT_COMPONENT_ID
      - PRICING_COMPONENT_ID

components have implementation for [snapshots based updates of golem workers](https://learn.golem.cloud/docs/rust-language-guide/updating#manual-snapshot-based-update)

## Commands

golem wasm-rpc stub generator

```
golem-cli stubgen initialize-workspace --targets order --targets product --targets pricing --callers cart --callers order
```

```
cargo make regenerate-stubs
cargo make release-build-flow
```

add components with golem-cli

```
golem-cli component add --component-name pricing target/wasm32-wasi/release/pricing.wasm
golem-cli component add --component-name product target/wasm32-wasi/release/product.wasm
golem-cli component add --component-name cart target/wasm32-wasi/release/cart_composed.wasm
golem-cli component add --component-name order target/wasm32-wasi/release/order_composed.wasm
```

add cart worker with golem-cli
```
golem-cli worker add --component-name cart  --worker-name user001 --env PRODUCT_COMPONENT_ID=35ec4b88-00e2-4948-a2b0-d6d9527fa437 --env PRICING_COMPONENT_ID=83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1 --env ORDER_COMPONENT_ID=98570ba9-0c35-4f80-ae7d-54a8ff957e64
```

upgrade cart workers of component with golem-cli
```
golem-cli component try-update-workers --component-name cart --update-mode manual
```

see also:
* [API Definitions and Deployments](./api/README.md)
* [benchmark](./benchmark/README.md)

golem documentation references:
* [snapshots based update of golem workers](https://learn.golem.cloud/docs/rust-language-guide/updating#manual-snapshot-based-update)
* [golem wasm-rpc](https://learn.golem.cloud/docs/rust-language-guide/rpc)
# golem-shopping


* cart - dependencies: pricing, product, order
* order - dependencies: pricing, product

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
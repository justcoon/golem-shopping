# golem-shopping

Project representing simple shopping application with components: product, pricing, cart and order. 
Application is written in Rust and aim to be run on [golem](https://learn.golem.cloud/)

## Components and Workers

* pricing
  - worker - per product, worker name: id of product
  - [api](./pricing/wit/pricing.wit)
  
* product
  - worker - per product, worker name: id of product
  - [api](./product/wit/product.wit)
  
* cart 
  - worker - per user/customer (there is always only one cart per user), worker name: id of user/customer
  - [api](./cart/wit/cart.wit)
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
  - [api](./order/wit/order.wit)
  - dependencies:
      - pricing 
      - product
  - env:
      - PRODUCT_COMPONENT_ID
      - PRICING_COMPONENT_ID

Components have implementation for [snapshots based updates of golem workers](https://learn.golem.cloud/docs/rust-language-guide/updating#manual-snapshot-based-update)

[REST APIs](./api/README.md) are provided by [golem workers api gateway](https://learn.golem.cloud/docs/invoke/making-custom-apis)



## Commands


release build of all components

```
golem-cli app build
```

add components with golem-cli

```
golem-cli component add
```

get component data with golem-cli

```
golem-cli component get product
golem-cli component get pricing
golem-cli component get order
```

add cart worker with [golem-cli](https://learn.golem.cloud/docs/cli/workers#start-new-worker) (env variables are representing related component id-s)
```
golem-cli worker new cart/user001 --env PRODUCT_COMPONENT_ID=35ec4b88-00e2-4948-a2b0-d6d9527fa437 --env PRICING_COMPONENT_ID=83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1 --env ORDER_COMPONENT_ID=98570ba9-0c35-4f80-ae7d-54a8ff957e64
```

upgrade cart workers of component with golem-cli
```
golem-cli component try-update-workers --component-name cart --update-mode manual
```

invocation of worker functions with golem-cli
```
golem-cli worker invoke cart/user014 golem:cart-exports/api.{get}
golem-cli worker invoke product/p001 golem:product-exports/api.{get} 
golem-cli worker invoke pricing/p001 golem:pricing-exports/api.{get} 
```

## References

* [APIs](./api/README.md)
* [benchmark](./benchmark/README.md)

golem documentation:
* [golem rust setup](https://learn.golem.cloud/docs/rust-language-guide/setup)
* [golem docker deployment](https://learn.golem.cloud/docs/deploy/docker)
* [snapshots based update of golem workers](https://learn.golem.cloud/docs/rust-language-guide/updating#manual-snapshot-based-update)
* [golem wasm-rpc](https://learn.golem.cloud/docs/rust-language-guide/rpc)
* [wasm interface type - wit](https://component-model.bytecodealliance.org/design/wit.html)
# golem-shopping

Project representing simple shopping application with components: product, pricing, cart and order. 
Application is written in Rust and aim to be run on [golem](https://learn.golem.cloud/)

## Components and Workers

* pricing
  - worker - per product, worker name: id of product
  - [api](./pricing/src_wit/pricing.wit)
* product
  - worker - per product, worker name: id of product
  - [api](./product/src_wit/product.wit)
* cart 
  - worker - per user/customer (there is always only one cart per user), worker name: id of user/customer
  - [api](./cart/src_wit/cart.wit)
  - dependencies: 
    - pricing 
    - product 
    - order
* order
  - worker - per order, worker name: id of order
  - [api](./order/src_wit/order.wit)
  - dependencies:
      - pricing 
      - product
* product-search
  - worker - per request
  - [api](./product-search/src_wit/product-search.wit)
  - dependencies:
    - product

Components have implementation for [snapshots based updates of golem workers](https://learn.golem.cloud/rust-language-guide/updating#manual-snapshot-based-update)

[REST APIs](./api/README.md) are provided by [golem workers api gateway](https://learn.golem.cloud/invoke/making-custom-apis)


## Commands


release build of all components

```
golem-cli app build
```

deploy components with golem-cli

```
golem-cli app deploy
```

get component data with golem-cli

```
golem-cli component get golem:product
golem-cli component get golem:pricing
golem-cli component get golem:order
golem-cli component get golem:cart
golem-cli component get golem:product-search
```

add cart worker with golem-cli
```
golem-cli worker new golem:cart/user001
```

upgrade cart workers of component with golem-cli
```
golem-cli component update-workers golem:cart --update-mode manual
```

invocation of worker functions with golem-cli
```
golem-cli worker invoke golem:cart/user001 golem:cart-exports/api.{get}
golem-cli worker invoke golem:product/p001 golem:product-exports/api.{get} 
golem-cli worker invoke golem:pricing/p001 golem:pricing-exports/api.{get} 
golem-cli worker invoke golem:product-search/- golem:product-search-exports/api.{find} '{ name: none, brand: some("Brand D") }'
```

## References

* [benchmark](./benchmark/README.md)

golem documentation:
* [golem rust setup](https://learn.golem.cloud/docs/rust-language-guide/setup)
* [golem docker deployment](https://learn.golem.cloud/docs/deploy/docker)
* [snapshots based update of golem workers](https://learn.golem.cloud/rust-language-guide/updating#manual-snapshot-based-update)
* [golem worker to worker communication](https://learn.golem.cloud/common-language-guide/rpc)
* [wasm interface type - wit](https://component-model.bytecodealliance.org/design/wit.html)
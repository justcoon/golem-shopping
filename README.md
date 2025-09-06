# Golem Shopping Application

A distributed shopping application built with Rust and WebAssembly, designed to run on the [Golem](https://learn.golem.cloud/). This application demonstrates a microservices architecture using Golem's serverless functions.

## 🚀 Quick Start

1. **Prerequisites**:
   - Install [Rust](https://www.rust-lang.org/tools/install)
   - Install [Golem CLI](https://learn.golem.cloud/docs/golem-cli/install)
   - Install [Docker](https://docs.docker.com/get-docker/) (for local development)

2. **Build and Deploy**:
   ```bash
   # Build all components
   golem-cli app build
   
   # Deploy to Golem Network
   golem-cli app deploy
   ```

3. **Import Sample Data**:
   For information on importing sample data, see the [Data README](./data/README.md).

4. **Run the Frontend**:
   See the [Frontend README](./frontend/README.md) for detailed frontend setup and development instructions.

## 🏗️ Project Structure

### Components and Workers

* pricing
  - worker - per product, worker name: id of product
  - [api](./components/pricing/src_wit/pricing.wit)
* product
  - worker - per product, worker name: id of product
  - [api](./components/product/src_wit/product.wit)
* cart 
  - worker - per user/customer (there is always only one cart per user), worker name: id of user/customer
  - [api](./components/cart/src_wit/cart.wit)
  - dependencies: 
    - pricing 
    - product 
    - order
* order
  - worker - per order, worker name: id of order
  - [api](./components/order/src_wit/order.wit)
  - dependencies:
      - pricing 
      - product
* product-search
  - worker - per request
  - [api](./components/product-search/src_wit/product-search.wit)
  - dependencies:
    - product

Components have implementation for [snapshots based updates of golem workers](https://learn.golem.cloud/rust-language-guide/updating#manual-snapshot-based-update)

REST APIs are provided by [golem workers api gateway](https://learn.golem.cloud/invoke/making-custom-apis)


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
golem-cli worker invoke golem:product-search/- golem:product-search-exports/api.{search} '"brand:\"Brand C\""'
```

## 📊 Benchmarking

For detailed information about performance testing, including how to import test data and run load tests, please see the [Benchmark README](./benchmark/README.md).

## 🌐 Frontend

For detailed information about the frontend, including setup, development, and available scripts, please see the [Frontend README](./frontend/README.md).

## 📚 Documentation

### Architecture

For a detailed architecture overview, see [architecture.puml](./architecture.puml) or the generated [architecture.png](./architecture.png).

### Golem Documentation
* [Golem Rust Setup](https://learn.golem.cloud/docs/rust-language-guide/setup)
* [Docker Deployment](https://learn.golem.cloud/docs/deploy/docker)
* [Worker Updates](https://learn.golem.cloud/rust-language-guide/updating#manual-snapshot-based-update)
* [Worker Communication](https://learn.golem.cloud/common-language-guide/rpc)
* [WASM Interface Types](https://component-model.bytecodealliance.org/design/wit.html)

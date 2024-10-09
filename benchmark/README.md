## Benchmarks

### import product and pricing data

* product with ids: p001 - p049
* pricing for products with ids: p001 - p050

[drill](https://github.com/fcsonline/drill) framework is used to make imports

env variables:
* HOST - worker service api gateway host
* API_HOST - api deployment host/site

```
HOST=http://localhost:9006 API_HOST=golem-shopping.test.local drill --benchmark import.yaml --stats
HOST=https://golem-shopping.dev-api.golem.cloud API_HOST='golem-shopping.dev-api.golem.cloud' drill --benchmark import.yaml --stats
```

### run benchmark

* product with ids: p001 - p050
* pricing for products with ids: p001 - p050 (p050 related request may produce errors)
* cart for user with ids: user001 - user004

add cart workers

```
golem-cli worker add --component-name cart  --worker-name user001 --env PRODUCT_COMPONENT_ID="ef4b7e5f-c2ff-451f-becc-2f20e313ba29"  --env PRICING_COMPONENT_ID="6f7a022d-e600-40ec-bb5e-44cb94339d98"  --env ORDER_COMPONENT_ID="2982f02f-97bc-4539-801c-a5483a9c2d03"
golem-cli worker add --component-name cart  --worker-name user002 --env PRODUCT_COMPONENT_ID="ef4b7e5f-c2ff-451f-becc-2f20e313ba29"  --env PRICING_COMPONENT_ID="6f7a022d-e600-40ec-bb5e-44cb94339d98"  --env ORDER_COMPONENT_ID="2982f02f-97bc-4539-801c-a5483a9c2d03"
golem-cli worker add --component-name cart  --worker-name user003 --env PRODUCT_COMPONENT_ID="ef4b7e5f-c2ff-451f-becc-2f20e313ba29"  --env PRICING_COMPONENT_ID="6f7a022d-e600-40ec-bb5e-44cb94339d98"  --env ORDER_COMPONENT_ID="2982f02f-97bc-4539-801c-a5483a9c2d03"
golem-cli worker add --component-name cart  --worker-name user004 --env PRODUCT_COMPONENT_ID="ef4b7e5f-c2ff-451f-becc-2f20e313ba29"  --env PRICING_COMPONENT_ID="6f7a022d-e600-40ec-bb5e-44cb94339d98"  --env ORDER_COMPONENT_ID="2982f02f-97bc-4539-801c-a5483a9c2d03"
```

[goose](https://github.com/tag1consulting/goose) load testing framework

```
HOST=http://localhost:9006 API_HOST=golem-shopping.test.local cargo run --release -- --report-file=report-local.html --no-reset-metrics
HOST=https://golem-shopping.dev-api.golem.cloud cargo run --release -- --report-file=report-cloud-dev.html --no-reset-metrics
```
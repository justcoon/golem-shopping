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
```

### run benchmark

* product with ids: p001 - p050
* pricing for products with ids: p001 - p050 (p050 related request may produce errors)
* cart for user with ids: user001 - user010

[goose](https://github.com/tag1consulting/goose) load testing framework

```
HOST=http://localhost:9006 API_HOST=golem-shopping.test.local cargo run --release -- --report-file=report-local.html --no-reset-metrics
```
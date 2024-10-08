
# import product and pricing data

```
HOST=http://localhost:9006 API_HOST=golem-shopping.test.local drill --benchmark import.yaml --stats
HOST=https://golem-shopping.dev-api.golem.cloud API_HOST='golem-shopping.dev-api.golem.cloud' drill --benchmark import.yaml --stats
```

# run benchmark

```
HOST=http://localhost:9006 API_HOST=golem-shopping.test.local cargo run --release -- --report-file=report-local.html --no-reset-metrics
HOST=https://golem-shopping.dev-api.golem.cloud cargo run --release -- --report-file=report-cloud-dev.html --no-reset-metrics
```
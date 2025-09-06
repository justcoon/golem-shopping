## Data

### import product and pricing data


[drill](https://github.com/fcsonline/drill) framework is used to make imports

env variables:
* HOST - worker service api gateway host
* API_HOST - api deployment host/site

```
HOST=http://localhost:9006 API_HOST=golem-shopping.test.local drill --benchmark import.yaml --stats
```
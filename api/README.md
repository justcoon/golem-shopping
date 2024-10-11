## API Definitions and Deployments

API definitions:
* cart: 
  * template: cart.json.tmpl
  * env variables:
    * CART_COMPONENT_ID
    * CART_COMPONENT_VERSION
* order:
  * template: order.json.tmpl
  * env variables:
    * ORDER_COMPONENT_ID
    * ORDER_COMPONENT_VERSION
* pricing:
  * template: pricing.json.tmpl
  * env variables:
    * PRICING_COMPONENT_ID
    * PRICING_COMPONENT_VERSION
* product:
  * template: product.json.tmpl
  * env variables:
    * PRODUCT_COMPONENT_ID
    * PRODUCT_COMPONENT_VERSION

variables substitution in API definitions template files:

```
CART_COMPONENT_ID="df93bd28-e341-453f-9dd7-3fc6bec20b23"  CART_COMPONENT_VERSION=0  envsubst < cart.json.tmpl > cart.json
ORDER_COMPONENT_ID="2982f02f-97bc-4539-801c-a5483a9c2d03"  ORDER_COMPONENT_VERSION=0  envsubst < order.json.tmpl > order.json
PRICING_COMPONENT_ID="6f7a022d-e600-40ec-bb5e-44cb94339d98"  PRICING_COMPONENT_VERSION=0  envsubst < pricing.json.tmpl > pricing.json
PRODUCT_COMPONENT_ID="ef4b7e5f-c2ff-451f-becc-2f20e313ba29"  PRODUCT_COMPONENT_VERSION=0  envsubst < product.json.tmpl > product.json
```

add API definitions with golem-cli
```
golem-cli api-definition add order.json
golem-cli api-definition add cart.json
golem-cli api-definition add pricing.json
golem-cli api-definition add product.json
```

deploy with golem-cli
```
golem-cli api-deployment deploy --subdomain golem-shopping --host test.local  --definition order/0.0.1 --definition cart/0.0.1 --definition pricing/0.0.1 --definition product/0.0.1
```

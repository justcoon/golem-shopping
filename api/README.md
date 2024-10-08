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
CART_COMPONENT_ID="1dea2b41-497d-4011-9698-30718dd83d47"  CART_COMPONENT_VERSION=13  envsubst < cart.json.tmpl > cart.json
ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"  ORDER_COMPONENT_VERSION=8  envsubst < order.json.tmpl > order.json
PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  PRICING_COMPONENT_VERSION=4  envsubst < pricing.json.tmpl > pricing.json
PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  PRODUCT_COMPONENT_VERSION=3  envsubst < product.json.tmpl > product.json
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

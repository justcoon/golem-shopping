## API

REST API definitions for golem components, which utilize [golem workers api gateway](https://learn.golem.cloud/docs/invoke/making-custom-apis)

API definitions:
* cart: 
  * template: [cart.json.tmpl](./cart.json.tmpl)
  * env variables:
    * CART_COMPONENT_NAME
    * CART_COMPONENT_VERSION
* order:
  * template: [order.json.tmpl](./order.json.tmpl)
  * env variables:
    * ORDER_COMPONENT_NAME
    * ORDER_COMPONENT_VERSION
* pricing:
  * template: [pricing.json.tmpl](./pricing.json.tmpl)
  * env variables:
    * PRICING_COMPONENT_NAME
    * PRICING_COMPONENT_VERSION
* product:
  * template: [product.json.tmpl](./product.json.tmpl)
  * env variables:
    * PRODUCT_COMPONENT_NAME
    * PRODUCT_COMPONENT_VERSION

variables substitution in API definitions template files:

```
CART_COMPONENT_NAME="cart"  CART_COMPONENT_VERSION=0  envsubst < cart.json.tmpl > cart.json
ORDER_COMPONENT_NAME="order"  ORDER_COMPONENT_VERSION=0  envsubst < order.json.tmpl > order.json
PRICING_COMPONENT_NAME="pricing"  PRICING_COMPONENT_VERSION=0  envsubst < pricing.json.tmpl > pricing.json
PRODUCT_COMPONENT_NAME="product"  PRODUCT_COMPONENT_VERSION=0  envsubst < product.json.tmpl > product.json
```

add API definitions with golem-cli
```
golem-cli api definition new order.json
golem-cli api definition new cart.json
golem-cli api definition new pricing.json
golem-cli api definition new product.json
```

deploy with golem-cli
```
golem-cli api deployment deploy --subdomain golem-shopping --host test.local  order/0.0.1 cart/0.0.1 pricing/0.0.1 product/0.0.1
```

references
* [golem rib](https://github.com/fcsonline/golem-rib)
* [making custom apis](https://learn.golem.cloud/docs/invoke/making-custom-apis)

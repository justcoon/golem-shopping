

```
CART_COMPONENT_ID="1dea2b41-497d-4011-9698-30718dd83d47"  CART_COMPONENT_VERSION=13  envsubst < cart.json.tmpl > cart.json

ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"  ORDER_COMPONENT_VERSION=8  envsubst < order.json.tmpl > order.json

PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  PRICING_COMPONENT_VERSION=4  envsubst < pricing.json.tmpl > pricing.json

PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  PRODUCT_COMPONENT_VERSION=3  envsubst < product.json.tmpl > product.json
```
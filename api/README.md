## cloud

```
CART_COMPONENT_ID="1dea2b41-497d-4011-9698-30718dd83d47"  CART_COMPONENT_VERSION=13  envsubst < cart.json.tmpl > cart.json
ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"  ORDER_COMPONENT_VERSION=8  envsubst < order.json.tmpl > order.json
PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  PRICING_COMPONENT_VERSION=4  envsubst < pricing.json.tmpl > pricing.json
PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  PRODUCT_COMPONENT_VERSION=3  envsubst < product.json.tmpl > product.json
```


golem-cloud-cli worker add --component-name cart  --worker-name user001 --env PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  --env PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  --env ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"
golem-cloud-cli worker add --component-name cart  --worker-name user002 --env PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  --env PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  --env ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"
golem-cloud-cli worker add --component-name cart  --worker-name user003 --env PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  --env PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  --env ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"
golem-cloud-cli worker add --component-name cart  --worker-name user004 --env PRODUCT_COMPONENT_ID="35ec4b88-00e2-4948-a2b0-d6d9527fa437"  --env PRICING_COMPONENT_ID="83ab925a-32e4-4c9d-bbe9-2c3b874ebcf1"  --env ORDER_COMPONENT_ID="98570ba9-0c35-4f80-ae7d-54a8ff957e64"

```
golem-cloud-cli api-deployment deploy --subdomain golem-shopping --host dev-api.golem.cloud --definition order/0.0.1 --definition cart/0.0.1 --definition pricing/0.0.1 --definition product/0.0.1
```

## local

```
CART_COMPONENT_ID="db7ca016-81b0-4668-b670-ea5c2ee56396"  CART_COMPONENT_VERSION=0  envsubst < cart.json.tmpl > cart.json
ORDER_COMPONENT_ID="77a6cf92-f7b8-455c-9291-38954d9693a1"  ORDER_COMPONENT_VERSION=0  envsubst < order.json.tmpl > order.json
PRICING_COMPONENT_ID="cc36f32a-6fdd-44ed-8830-097e56cfbb14"  PRICING_COMPONENT_VERSION=0 envsubst < pricing.json.tmpl > pricing.json
PRODUCT_COMPONENT_ID="77a1d2ed-dcd5-42a6-b34e-0db72325b517"  PRODUCT_COMPONENT_VERSION=0  envsubst < product.json.tmpl > product.json
```

golem-cli worker add --component-name cart  --worker-name user001 --env PRODUCT_COMPONENT_ID="77a1d2ed-dcd5-42a6-b34e-0db72325b517"  --env PRICING_COMPONENT_ID="cc36f32a-6fdd-44ed-8830-097e56cfbb14"  --env ORDER_COMPONENT_ID="77a6cf92-f7b8-455c-9291-38954d9693a1"
golem-cli worker add --component-name cart  --worker-name user002 --env PRODUCT_COMPONENT_ID="77a1d2ed-dcd5-42a6-b34e-0db72325b517"  --env PRICING_COMPONENT_ID="cc36f32a-6fdd-44ed-8830-097e56cfbb14"  --env ORDER_COMPONENT_ID="77a6cf92-f7b8-455c-9291-38954d9693a1"
golem-cli worker add --component-name cart  --worker-name user003 --env PRODUCT_COMPONENT_ID="77a1d2ed-dcd5-42a6-b34e-0db72325b517"  --env PRICING_COMPONENT_ID="cc36f32a-6fdd-44ed-8830-097e56cfbb14"  --env ORDER_COMPONENT_ID="77a6cf92-f7b8-455c-9291-38954d9693a1"
golem-cli worker add --component-name cart  --worker-name user004 --env PRODUCT_COMPONENT_ID="77a1d2ed-dcd5-42a6-b34e-0db72325b517"  --env PRICING_COMPONENT_ID="cc36f32a-6fdd-44ed-8830-097e56cfbb14"  --env ORDER_COMPONENT_ID="77a6cf92-f7b8-455c-9291-38954d9693a1"
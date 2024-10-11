#!/bin/bash

set -euo pipefail

GOLEM_COMMAND="golem-cli"

CART_COMPONENT_NAME="cart"
ORDER_COMPONENT_NAME="order"
PRICING_COMPONENT_NAME="pricing"
PRODUCT_COMPONENT_NAME="product"

API_TEMPLATE_DIR="./api"
API_DIR="./api"

SED="sed -i"
if [[ "$OSTYPE" == "darwin"* ]]; then
  SED="sed -i ''"
fi

function update_component() {
  echo "Updating component: '${1?}' with wasm file '${2?}.wasm'"
  ${GOLEM_COMMAND} component update --non-interactive --component-name=${1?} target/wasm32-wasip1/release/${2?}.wasm
}

function update_workers() {
  ${GOLEM_COMMAND} component try-update-workers --component-name=${1?}
}


function get_component_id() {
  ${GOLEM_COMMAND} component get --component-name=${1?} --format json | jq -r '.componentUrn' | sed 's/urn:component://'
}

function get_component_version() {
  ${GOLEM_COMMAND} component get --component-name=${1?} --format json | jq -r '.componentVersion'
}

function init_api_definitions() {
  CART_COMPONENT_ID=$(get_component_id $CART_COMPONENT_NAME)
  CART_COMPONENT_VERSION=$(get_component_version $CART_COMPONENT_NAME)
  ORDER_COMPONENT_ID=$(get_component_id $ORDER_COMPONENT_NAME)
  ORDER_COMPONENT_VERSION=$(get_component_version $ORDER_COMPONENT_NAME)
  PRICING_COMPONENT_ID=$(get_component_id $PRICING_COMPONENT_NAME)
  PRICING_COMPONENT_VERSION=$(get_component_version $PRICING_COMPONENT_NAME)
  PRODUCT_COMPONENT_ID=$(get_component_id $PRODUCT_COMPONENT_NAME)
  PRODUCT_COMPONENT_VERSION=$(get_component_version $PRODUCT_COMPONENT_NAME)

  CART_COMPONENT_ID="${CART_COMPONENT_ID}"  CART_COMPONENT_VERSION="${CART_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/cart.json.tmpl > ${API_DIR}/cart.json
  ORDER_COMPONENT_ID="${ORDER_COMPONENT_ID}"  ORDER_COMPONENT_VERSION="${ORDER_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/order.json.tmpl > ${API_DIR}/order.json
  PRICING_COMPONENT_ID="${PRICING_COMPONENT_ID}"  PRICING_COMPONENT_VERSION="${PRICING_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/pricing.json.tmpl > ${API_DIR}/pricing.json
  PRODUCT_COMPONENT_ID="${PRODUCT_COMPONENT_ID}"  PRODUCT_COMPONENT_VERSION="${PRODUCT_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/product.json.tmpl > ${API_DIR}/product.json
}

function add_api_definition() {
  ${GOLEM_COMMAND} api-definition add ${API_DIR}/${1?}.json
}

# ${GOLEM_COMMAND} api-deployment deploy --subdomain golem-shopping --host test.local  --definition order/0.0.1 --definition cart/0.0.1 --definition pricing/0.0.1 --definition product/0.0.1
init_api_definitions
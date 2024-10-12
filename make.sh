#!/bin/bash

set -euo pipefail

GOLEM_COMMAND="golem-cli"

CART_COMPONENT_NAME="cart"
ORDER_COMPONENT_NAME="order"
PRICING_COMPONENT_NAME="pricing"
PRODUCT_COMPONENT_NAME="product"

API_TEMPLATE_DIR="./api"
API_DIR="./api"

API_HOST="test.local"
API_SUBDOMAIN="golem-shopping"
API_SITE="${API_SUBDOMAIN}.${API_HOST}"


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

function init_api_definitions_files() {
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

function delete_api_definition() {
  VERSION=$(get_api_definition_version ${1?})
  if [[ -n "${VERSION}" ]]; then
    echo "Deleting API definition: ${1?} with version ${VERSION}"
    ${GOLEM_COMMAND} api-definition delete --id ${1?}
  else
    echo "API definition ${1?} does not exist"
  fi
}

function get_api_definition_version() {
  ${GOLEM_COMMAND} api-definition list --id ${1?} --format json | jq -r '.[].version'
}

function add_api_definition_if_not_exists() {
    VERSION=$(get_api_definition_version ${1?})
    if [[ "${VERSION}" == "" ]]; then
      echo "Adding API definition: ${1?}"
      add_api_definition ${1?}
    else
      echo "API definition ${1?} already exists with version ${VERSION}"
    fi
}

function add_api_definitions() {
  add_api_definition_if_not_exists cart
  add_api_definition_if_not_exists order
  add_api_definition_if_not_exists pricing
  add_api_definition_if_not_exists product
}

function deploy_api_definitions() {
    CART_VERSION=$(get_api_definition_version cart)
    ORDER_VERSION=$(get_api_definition_version order)
    PRICING_VERSION=$(get_api_definition_version pricing)
    PRODUCT_VERSION=$(get_api_definition_version product)

    if [[ -n "${CART_VERSION}" && -n "${ORDER_VERSION}" && -n "${PRICING_VERSION}" && -n "${PRODUCT_VERSION}" ]]; then

      echo "Getting API deployment for site: ${API_SITE}"
      ${GOLEM_COMMAND} api-deployment get ${API_SITE}  > /dev/null 2>&1

      if [[ $? -ne 0 ]]; then
        echo "Deploying API definitions for site: ${API_SITE}"
        ${GOLEM_COMMAND} api-deployment deploy --subdomain ${API_SUBDOMAIN} --host ${API_HOST} --definition order/${ORDER_VERSION} --definition cart/${CART_VERSION} --definition pricing/${PRICING_VERSION} --definition product/${PRODUCT_VERSION}
      else
        echo "API deployment for site: ${API_SITE} already exists"
      fi
    else
      echo "Not all API definitions are available. CART_VERSION: ${CART_VERSION} ORDER_VERSION: ${ORDER_VERSION} PRICING_VERSION: ${PRICING_VERSION} PRODUCT_VERSION: ${PRODUCT_VERSION}"
    fi
}

function delete_api_deployment() {
    ${GOLEM_COMMAND} api-deployment delete ${API_SITE}
}

init_api_definitions_files

add_api_definitions

deploy_api_definitions
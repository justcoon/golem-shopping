#!/bin/bash

#set -euo pipefail

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
  ${GOLEM_COMMAND} component update --non-interactive --component-name=${1?} target/golem-temp/linked-wasm/${2?}.wasm
}

function update_workers() {
  ${GOLEM_COMMAND} component try-update-workers --component-name=${1?}
}

function get_component_id() {
  ${GOLEM_COMMAND} component get ${1?} --format json | jq -r '.componentId'
}

function get_component_version() {
  ${GOLEM_COMMAND} component get ${1?} --format json | jq -r '.componentVersion'
}

function get_component_wasm_file() {
  echo "target/golem-temp/linked-wasm/${1?}.wasm"
}

function add_component() {
  ${GOLEM_COMMAND} component add --component-name ${1?} ${2?}
}

function update_component() {
  ${GOLEM_COMMAND} component update --component-name ${1?} ${2?}
}

function add_or_update_component() {
  WASM_FILE=$(get_component_wasm_file ${2?})

  if [[ -f "${WASM_FILE}" ]]; then
    VERSION=$(get_component_version ${1?})

    if [[ -n "${VERSION}" ]]; then
      echo "Updating component: ${1?}"
      update_component ${1?} ${WASM_FILE}
    else
      echo "Adding component: ${1?}"
      add_component ${1?} ${WASM_FILE}
    fi
  else
    echo "WASM file: ${WASM_FILE}, of component: ${1?} does not exist"
  fi
}

function add_or_update_components() {
  ${GOLEM_COMMAND} app deploy
}

function build_components() {
  ${GOLEM_COMMAND} app build
}

function init_api_definitions_files() {
  CART_COMPONENT_VERSION=$(get_component_version $CART_COMPONENT_NAME)
  ORDER_COMPONENT_VERSION=$(get_component_version $ORDER_COMPONENT_NAME)
  PRICING_COMPONENT_VERSION=$(get_component_version $PRICING_COMPONENT_NAME)
  PRODUCT_COMPONENT_VERSION=$(get_component_version $PRODUCT_COMPONENT_NAME)

  CART_COMPONENT_NAME="${CART_COMPONENT_NAME}"  CART_COMPONENT_VERSION="${CART_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/cart.json.tmpl > ${API_DIR}/cart.json
  ORDER_COMPONENT_NAME="${ORDER_COMPONENT_NAME}"  ORDER_COMPONENT_VERSION="${ORDER_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/order.json.tmpl > ${API_DIR}/order.json
  PRICING_COMPONENT_NAME="${PRICING_COMPONENT_NAME}"  PRICING_COMPONENT_VERSION="${PRICING_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/pricing.json.tmpl > ${API_DIR}/pricing.json
  PRODUCT_COMPONENT_NAME="${PRODUCT_COMPONENT_NAME}"  PRODUCT_COMPONENT_VERSION="${PRODUCT_COMPONENT_VERSION}"  envsubst < ${API_TEMPLATE_DIR}/product.json.tmpl > ${API_DIR}/product.json
}

function add_api_definition() {
  ${GOLEM_COMMAND} api definition new ${API_DIR}/${1?}.json
}

function delete_api_definition() {
  VERSION=$(get_api_definition_version ${1?})
  if [[ -n "${VERSION}" ]]; then
    echo "Deleting API definition: ${1?} with version ${VERSION}"
    ${GOLEM_COMMAND} api definition delete --id ${1?} --version ${VERSION}
  else
    echo "API definition ${1?} does not exist"
  fi
}

function get_api_definition_version() {
  ${GOLEM_COMMAND} api definition list --id ${1?} --format json | jq -r '.[].version'
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
      ${GOLEM_COMMAND} api deployment get ${API_SITE}  > /dev/null 2>&1

      if [[ $? -ne 0 ]]; then
        echo "Deploying API definitions for site: ${API_SITE}"
        ${GOLEM_COMMAND} api deployment deploy --subdomain ${API_SUBDOMAIN} --host ${API_HOST} order/${ORDER_VERSION}  cart/${CART_VERSION} pricing/${PRICING_VERSION} product/${PRODUCT_VERSION}
      else
        echo "API deployment for site: ${API_SITE} already exists"
      fi
    else
      echo "Not all API definitions are available. CART_VERSION: ${CART_VERSION} ORDER_VERSION: ${ORDER_VERSION} PRICING_VERSION: ${PRICING_VERSION} PRODUCT_VERSION: ${PRODUCT_VERSION}"
    fi
}

function delete_api_deployment() {
    echo "Deleting API deployment for site: ${API_SITE}"
    ${GOLEM_COMMAND} api deployment delete ${API_SITE}
}

function deploy_api() {
  init_api_definitions_files
  add_api_definitions
  deploy_api_definitions
}

function undeploy_api() {
  delete_api_deployment

  delete_api_definition cart
  delete_api_definition order
  delete_api_definition pricing
  delete_api_definition product
}

function create_cart_workers() {
    arr=("$@")

    ORDER_COMPONENT_ID=$(get_component_id $ORDER_COMPONENT_NAME)
    PRICING_COMPONENT_ID=$(get_component_id $PRICING_COMPONENT_NAME)
    PRODUCT_COMPONENT_ID=$(get_component_id $PRODUCT_COMPONENT_NAME)

    for i in "${arr[@]}"; do
        ${GOLEM_COMMAND}  worker new ${CART_COMPONENT_NAME}/${i} --env PRODUCT_COMPONENT_ID="${PRODUCT_COMPONENT_ID}" --env PRICING_COMPONENT_ID="${PRICING_COMPONENT_ID}" --env ORDER_COMPONENT_ID="${ORDER_COMPONENT_ID}"
    done
}

function update_worker() {
  echo "Updating worker: '${1?}'"
  ${GOLEM_COMMAND} component try-update-workers --component-name=${1?} --update-mode manual
}

function update_workers() {
    update_worker $CART_COMPONENT_NAME
    update_worker $ORDER_COMPONENT_NAME
    update_worker $PRICING_COMPONENT_NAME
    update_worker $PRODUCT_COMPONENT_NAME
}

function help() {
    echo "Usage: $0 <command>"
    echo "Commands:"
    echo "  build-components - build all components"
    echo "  add-components - add or update components to golem"
    echo "  create-cart-workers - create cart workers for the given users"
    echo "  deploy-api - deploy api definitions"
    echo "  undeploy-api - undeploy api definitions"
    echo "  update-workers - update all workers to latest version"
    echo "  help - display this help message"
}

for arg in "$@"; do
    case $arg in
      build-components)
        build_components
        ;;
      add-components)
        add_or_update_components
        ;;
      create-cart-workers)
        create_cart_workers "user011" "user012" "user013" "user014" "user015" "user016"
        ;;
      deploy-api)
        deploy_api
        ;;
      undeploy-api)
        undeploy_api
        ;;
      update-workers)
        update_workers
        ;;
      help)
        help
        ;;
      *)
        echo "Invalid argument: $arg"
        help
        exit 1
        ;;
    esac
  done
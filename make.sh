#!/bin/bash

#set -euo pipefail

GOLEM_COMMAND="golem-cli"

CART_COMPONENT_NAME="golem:cart"
ORDER_COMPONENT_NAME="golem:order"
PRICING_COMPONENT_NAME="golem:pricing"
PRODUCT_COMPONENT_NAME="golem:product"

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
#!/bin/bash

SUBSTRATE_V="3.0.0"
SUBSTRATE_TIMESTAMP="monthly-2021-10"
SUBXT_V="f342b0636fe04703c17f25ab0412c5b826020cf8"

clone_node_template()
{
  echo ""
  echo "Let's clone substrate-node-template..."
  git clone https://github.com/substrate-developer-hub/substrate-node-template -b v${SUBSTRATE_V}+${SUBSTRATE_TIMESTAMP} substrate-node-chaos
}

add_pallet_chaos()
{
  echo ""
  echo "Let's add pallet-chaos to the runtime..."

  pushd substrate-node-chaos > /dev/null
  pushd pallets > /dev/null
  git clone ssh://git@github.com/paritytech/pallet-chaos.git
  popd > /dev/null

  git apply ../diff/add_chaos_runtime.diff
  popd > /dev/null
}

build_node_template()
{
  echo ""
  echo "Let's build the node-template executable..."
  pushd substrate-node-chaos > /dev/null

  # TODO
  # WASM vs Native

  cargo build --release
  popd > /dev/null
}

kill_node_template()
{
  NODE_RUNNING=$(ps aux | grep node-template | wc -l)

  if [ "$NODE_RUNNING" -gt "1" ]; then
   echo "We need to kill some zombie node-template processes in the system... Please type in your sudo password:"
   sudo killall node-template
  fi
}

screen_node_template()
{
  pushd substrate-node-chaos > /dev/null
  screen -d -m ./target/release/node-template --dev --tmp
  popd > /dev/null
}

run_node_template()
{
  if [ ! -d "substrate-node-chaos" ]; then
    clone_node_template
    add_pallet_chaos
  fi

  if [ ! -d "substrate-node-chaos/target/release/node-template" ]; then
    build_node_template
  fi

  echo ""
  echo "Let's start the substrate-node-chaos executable..."

  kill_node_template

  screen_node_template

  echo "Your substrate-node-chaos is running as a screen session in the background."
  echo "You can check it by running \"ps aux | grep node-template\""
  echo "You can kill it by running \"sudo killall node-template\"."
}

get_metadata()
{
  run_node_template
  echo ""
  echo "Waiting for the node to start so we can extract the metadata..."
  sleep 5
  curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
                      | jq .result \
                      | cut -d '"' -f 2 \
                      | xxd -r -p > ./metadata/substrate-node-chaos.scale
  if [ -s metadata/substrate-node-chaos.scale ]; then
    echo "Metadata extracted!"
  else
    echo "Something went wrong with the metadata extraction! Abort..."
    exit 1
  fi
}

build_chaoscope()
{
  cargo build --release
}

run_chaoscope()
{
  get_metadata

  echo ""
  echo "Let's build chaoscope..."
  build_chaoscope

  echo ""
  echo "Let's run chaoscope..."
  ./target/release/chaoscope
}

echo ""
echo "Our local setup will be based on:"
echo " - [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)"
echo " - [pallet-chaos](https://github.com/paritytech/pallet-chaos)"
echo ""

run_chaoscope
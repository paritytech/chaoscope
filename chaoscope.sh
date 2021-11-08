#!/bin/bash

SUBSTRATE_V="3.0.0"
SUBSTRATE_TIMESTAMP="monthly-2021-10"

echo ""
echo "   🅂⒰𝕭𝓼ṪᴙȺՇ乇"
echo "       _"
echo "      ⇖⇑⇗"
echo "      ⇐●⇒"
echo "      ⇙⇓⇘"
echo "       ‾"
echo "     ȻнД⓿丂"
echo ""
echo "⚠️Expect... Chaoscope! ⚠"
echo ""
echo "Our local setup will be based on:"
echo " - [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)"
echo " - [pallet-chaos](https://github.com/paritytech/pallet-chaos)"
echo ""
echo "Please, set your options as environment variables and call this script again."
echo "RUN_NODE=1               -> Spin a substrate-node-chaos node (as screen session)"
echo "RUN_CHAOSCOPE=1            -> Run chaoscope cli"
echo "BUILD_CHAOSCOPE_WASM=1     -> Build chaoscope.wasm"
echo ""
echo "ex.: RUN_NODE=1 RUN_CHAOSCOPE=1 ./chaoscope.sh"
echo ""

if [ ! -z "$RUN_NODE" ]; then
  if [ ! -d "substrate-node-chaos" ]; then
    echo ""
    echo "Let's clone substrate-node-template..."

    git clone https://github.com/substrate-developer-hub/substrate-node-template -b v${SUBSTRATE_V}+${SUBSTRATE_TIMESTAMP} substrate-node-chaos
    pushd substrate-node-chaos

    echo ""
    echo "Let's clone pallet-chaos..."
    pushd pallets
    git clone ssh://git@github.com/paritytech/pallet-chaos.git
    popd

    echo ""
    echo "Let's add pallet-chaos to the runtime..."
    git apply pallets/pallet-chaos/diff/add_chaos_runtime.diff
    popd
  fi

  if [ ! -d "substrate-node-chaos/target/release/node-template" ]; then
    echo ""
    echo "Let's build the node-template executable..."
    pushd substrate-node-chaos
    cargo build --release
    popd
  fi

  echo ""
  echo "Let's start the substrate-node-chaos executable..."
  pushd substrate-node-chaos
  screen -d -m ./target/release/node-template --dev --tmp

  echo "Your substrate-node-chaos is running as a screen session in the background."
  echo "You can check it by running \"ps aux | grep node-template\""
  echo "You can kill it by running \"sudo killall node-template\"."
  popd
fi

if [ ! -z "$RUN_CHAOSCOPE" ]; then
  echo ""
  echo "Let's run the chaoscope cli..."

  cargo run --release
fi

if [ ! -z "$BUILD_CHAOSCOPE_WASM" ]; then
  echo ""
  echo "Let's build chaoscope.wasm..."
  # ToDo
fi

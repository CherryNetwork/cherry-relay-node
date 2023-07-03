#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e

sudo apt update
sudo apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config protobuf-compiler cmake g++ make

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ]; then
    rustup update nightly
    rustup update stable
fi

rustup target add wasm32-unknown-unknown --toolchain nightly


cargo b --release && ./target/release/cherry purge-chain --chain node/service/chain-specs/mainnet-relay-regenesis.json -y && ./target/release/cherry --chain node/service/chain-specs/mainnet-relay-regenesis.json --bootnodes /ip4/13.39.104.56/tcp/30333/p2p/12D3KooWJmNgQE6hftytjeXnwNRNrkxFLoChSa5FSuU7F4DnBUEZ --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
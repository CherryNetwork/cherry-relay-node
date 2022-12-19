#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e

sudo apt update
sudo apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config
sudo apt install -y protobuf-compiler
sudo apt install cmake g++ make

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ]; then
    rustup update nightly
    rustup update stable
fi

rustup target add wasm32-unknown-unknown --toolchain nightly


cargo b --release && ./target/release/cherry purge-chain --chain node/service/chain-specs/mainnet-relay-regenesis.json -y && ./target/release/cherry --chain node/service/chain-specs/mainnet-relay-regenesis.json --bootnodes /ip4/15.236.154.200/tcp/30333/p2p/12D3KooWQTgHfboF9q1Ni8q3vG3MVJL5RMxYxYJvLnw3z7P2Mejp --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
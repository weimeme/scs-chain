#!/bin/bash

# Generating key in "db/chains/local_testnet/network/secret_ed25519"
./target/release/substrate-node key generate-node-key --chain local --base-path db
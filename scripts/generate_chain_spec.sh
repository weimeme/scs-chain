#!/bin/bash

# after generate node key
./target/release/substrate-node build-spec --raw  --base-path db --chain local > chain-spec.json
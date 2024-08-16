#!/bin/bash

# after generate node key
./target/release/scs-node build-spec --raw  --base-path db --chain local > chain-spec.json
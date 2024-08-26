#!/bin/bash

# after generate node key
./target/release/scs build-spec --raw  --base-path db --chain staging --disable-default-bootnode > chain-spec.json
#!/bin/bash

secret=$1
#stash, controller, session-key, beefy id
#generated with secret:
# ed25519
for i in 1 2 3 ; do for j in ed;
do subkey inspect --scheme ed25519 "//$secret"//fir//$j//$i; done; done

#and
# sr25519
for i in 1 2 3 ; do for j in sr ; do subkey inspect "//$secret"/fir/$j/$i; done; done

#and

# ecdsa
for i in 1 2 3 ; do for j in ecdsa; do subkey inspect --scheme ecdsa "//$secret"//fir//$j//$i; done; done

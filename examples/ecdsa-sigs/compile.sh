#!/bin/bash
NSIGS=$1

cd examples/ecdsa-sigs
rm ecdsa_sigs_test.circom 2> /dev/null
touch ecdsa_sigs_test.circom
echo "pragma circom 2.1.2;" >> ecdsa_sigs_test.circom
echo "include \"/home/ubuntu/zkconnect4/packages/circuits/circom/lib/batch_efficient_ecdsa_pubkey.circom\";" >> ecdsa_sigs_test.circom
echo "component main{ public [step_in] } = BatchEfficientECDSAPubKey(${1});" >> ecdsa_sigs_test.circom

circom ecdsa_sigs_test.circom --r1cs --wasm --output ./ --prime secq256k1
#!/bin/bash

circom ./examples/toy/toy.circom --r1cs --wasm --sym --output ./examples/toy/ --prime secq256k1

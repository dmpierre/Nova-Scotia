cd examples/bitcoin/circom
rm bitcoin_benchmark.circom
head -n -1 bitcoin.circom > bitcoin_benchmark.circom
echo include \"./bitcoin.circom\"\; >> bitcoin_benchmark.circom
echo "component main { public [step_in] } = Main($1);" >> bitcoin_benchmark.circom

# error: "ld: symbol(s) not found for architecture arm64" when using M2
# compile to wasm instead

circom bitcoin_benchmark.circom --r1cs --sym --wasm --prime secq256k1
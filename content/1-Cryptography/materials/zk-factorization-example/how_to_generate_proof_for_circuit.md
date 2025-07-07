cd pba-content/syllabus/1-Cryptography/zk-proofs/circuit_example_playground

# compile circuit

circom ../circuit.circom --r1cs --wasm --sym

# generate proving and verification keys for the circuit

snarkjs plonk setup circuit.r1cs powers_of_tau/pot10_final.ptau circuit_final.zkey
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

# generate witness trace

cd circuit_js
node generate_witness.js circuit.wasm ../input.json ../witness.wtns
cd ..

# generate proof for the witness

snarkjs plonk prove circuit_final.zkey witness.wtns proof.json public.json

# verify the proof

snarkjs plonk verify verification_key.json public.json proof.json

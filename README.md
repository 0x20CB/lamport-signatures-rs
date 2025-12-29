# Lamport-Signatures-rs (with Configuration)

## Preface

## Description

Lamport Signatures, created by Leslie Lamport, is one of the most simplistic and easy to understand hash-based algorithm scheme based on hashing bits, often being a primitive for Hash-based digital signature schemes like SPHINCS+ and XMSS.

The basis of lamport signature security is found in the hardness of computing the preimage of a given hash, using algorithms such as SHA256, SHA3 (Keccak), and BLAKE2.

## How Lamport Signatures Work

Lamport Signatures form the basis of the ecosystem of Hash-Based Cryptographic Digital Signatures.

In lamport signatures, the secret keys are generated using cryptographic randomness, 2 * n * 8 where n is the number of bytes being signed.

Two secret keys per bit are used to prove whether the bit is true or false (0/1).

To create the secret key, the secret keys are hashed using a one-way hashing algorithm and kept as the public key.

Being a one-time digital signature scheme, it offers one-time signatures that can be used to sign bytes of data by revealing the secret key for the bit neccessary.

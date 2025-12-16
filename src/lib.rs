//! # Lamport Signatures: A Revised Version of Lamport Signatures, a Post-Quantum Digital Signature Scheme That Offers Security Based On Hash Algorithms
//! 
//! ## Features
//! 
//! - [X] Modular
//! - [X] Protects Against Branching Attacks, Timing Attacks, and Other Side-Channel Attacks
//! - [ ] Protect Against Randomness Attacks
//! 
//! - [X] Make a no_std support version
//! 
//! ## Goals
//! 
//! - [X] To be the defacto crate for Lamport Signature implementation
//! - [X] To add to libslug
//! - [X] To add to RustCrypto
//! 
//! - [ ] no_std
//! 
//! 
//! 
//! Number of Keypairs: 2 * n * 8
//! 
//! Up to 64 byte signing...
//! 
//! 



#![no_std]

use digest::Digest;
use rand_core::RngCore;
use rand_core::CryptoRng;

pub trait DIGEST_SIZED_ARRAY {

}

impl DIGEST_SIZED_ARRAY for [u8;20] {

}

impl DIGEST_SIZED_ARRAY for [u8;28] {

}

impl DIGEST_SIZED_ARRAY for [u8;32] {

}

impl DIGEST_SIZED_ARRAY for [u8;48] {

}

impl DIGEST_SIZED_ARRAY for [u8;64] {

}



//=====BASE STRUCTS=====//

pub struct PublicKey<DSA: DIGEST_SIZED_ARRAY> {
    pk: [DSA;1024],
}

pub struct SecretKey<DSA: DIGEST_SIZED_ARRAY> {
    sk: [DSA;1024],
}

pub struct Keypair<DSA: DIGEST_SIZED_ARRAY> {
    pk: PublicKey<DSA>,
    sk: SecretKey<DSA>,
}

pub struct Signature<DSA: DIGEST_SIZED_ARRAY> {
    sig: [DSA;1024],
}



impl SecretKey {
    /// # Generate Secret Keys (Through CSPRNG)
    /// 
    /// ## Description
    /// 
    /// The number of keypairs generated is based on the value `n`, which is the number of bytes expected to be signed (usually ranging from 32-64 due to signing a hash).
    /// 
    /// In this stage, the hashing function is not need to be known as these are just the secret keys.
    /// 
    /// **Generation Algorithm:** `2n * 8` where `2`` represents a pair, `n` represents number of bytes that can be signed, and `8` which represents a byte.
    /// 
    /// **RNG:** It is recommended to have enough entropy on your system to generate the keys and to use a secure randomness generator for keys. Key sizes are typically 32 bytes, but can be extended up to 64 bytes for added security, although it will make the signature longer.
    /// 
    /// ## Parameters
    /// 
    /// `n`: number of bytes that can be signed using the key. Typically, you would sign the hash of a message so this value would be 32-64.
    /// 
    /// `rng`: the implemented cryptographic rng using the traits `RngCore` and `CryptoRng`
    /// 
    /// `size_of_secret_keys`: The byte size of the secret keys. Usually ranges from 32-64.
    pub fn generate_with_config<R: RngCore + CryptoRng>(n: usize, size_of_secret_keys: usize, mut rng: R) {
        // Math for number of bits in signable message and how many keypairs there will be.
        let bits_to_sign = n * 8;
        // Math for number of keypairs
        let number_of_keypairs = bits_to_sign * 2;




        for _ in 0..number_of_keypairs {
            let mut secret_key: [u8;32] = [0u8;32];
            rng.fill_bytes(&mut secret_key);
        }




    }
}



/// # Config
/// 
/// Contains all configuration data.
pub struct LamportSignatureConfig<H: Digest>  {
    hasher: H,
    n: usize, // number of bytes to sign

}

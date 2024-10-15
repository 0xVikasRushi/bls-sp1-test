#![no_main]

use bls12_381::{
    multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt,
};
use group::Group;
use rand::thread_rng;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    let a = G1Affine::from(G1Projective::random(&mut thread_rng()));
    let b = G2Affine::from(G2Projective::random(&mut thread_rng()));
    let b_neg = -b;

    println!("cycle-tracker-start: bls12_381-pairing-with-negative");

    // Compute e(a,b) * e(a,-b)
    let result = multi_miller_loop(&[(&a, &G2Prepared::from(b)), (&a, &G2Prepared::from(b_neg))])
        .final_exponentiation();

    // Check if the result is the identity in Gt
    assert_eq!(result, Gt::identity());

    println!("cycle-tracker-end: bls12_381-pairing-with-negative");
}

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use fibonacci_lib::PublicValuesStruct;

use bls12_381::{
    multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt,
    Scalar,
};

pub fn main() {
    let a = G1Affine::from(
        G1Affine::generator() * Scalar::from_raw([7, 7, 7, 7]).invert().unwrap().square(),
    );

    // Generate a random point in G2
    let b = G2Affine::from(
        G2Affine::generator() * Scalar::from_raw([8, 8, 8, 8]).invert().unwrap().square(),
    );

    // Calculate -b
    let b_neg = -b;

    // Prepare b and -b for the Miller loop
    let b_prepared = G2Prepared::from(b);
    let b_neg_prepared = G2Prepared::from(b_neg);

    // Calculate e(a,b) * e(a,-b) using multi_miller_loop
    let result =
        multi_miller_loop(&[(&a, &b_prepared), (&a, &b_neg_prepared)]).final_exponentiation();

    // The result should be the identity element in Gt
    assert_eq!(result, Gt::identity());

    // Optional: Verify that e(a,b) * e(a,-b) = 1 using individual pairings
    let pairing_product = pairing(&a, &b) + pairing(&a, &b_neg);
    assert_eq!(pairing_product, Gt::identity());

    let result_u32 = result.0.to_bytes()[0] as u32; // Convert Gt to u32
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { result: result_u32 });
    sp1_zkvm::io::commit_slice(&bytes);
}

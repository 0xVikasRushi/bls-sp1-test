//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use sp1_sdk::{utils, ProverClient, SP1Stdin};

pub const ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    let client = ProverClient::new();

    let stdin = SP1Stdin::new();

    let (pk, vk) = client.setup(ELF);

    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    println!("Proof generated successfully!");

    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}

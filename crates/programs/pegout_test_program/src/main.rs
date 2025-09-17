#![no_main]

sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use vault_prover_program_core::{CLAIMER_PUBKEY, PegoutTestProofInput, PegoutTestProofOutput};

pub fn main() {
    let pegout_input = sp1_zkvm::io::read::<PegoutTestProofInput>();
    assert_eq!(
        pegout_input.claimer_pubkey, CLAIMER_PUBKEY,
        "Claimer's pubkey is not correct."
    );
    // compress other public inputs into hash bytes `[u8; 32]`
    let mut bytes = Vec::with_capacity(32);
    bytes.extend(&pegout_input.dummy_data);
    let phi: [u8; 32] = Sha256::digest(bytes).into();

    // commit two public inputs, one is pegin id, the other one is a image of other original public inputs
    sp1_zkvm::io::commit(&PegoutTestProofOutput {
        pegin_txid: pegout_input.pegin_txid,
        phi,
    });
}

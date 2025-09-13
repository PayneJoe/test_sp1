#![no_main]

sp1_zkvm::entrypoint!(main);

use bitcoin::{
    Script,
    TapSighashType,
    Transaction,
    TxOut,
    XOnlyPublicKey,
    key::Secp256k1,
    secp256k1::{Message, VerifyOnly, schnorr::Signature},
    sighash::{Prevouts, SighashCache},
    taproot::ControlBlock,
};
use sp1_verifier::Groth16Verifier;
use sp1_verifier::hash_public_inputs;
use std::sync::LazyLock;

pub static SECP: LazyLock<Secp256k1<VerifyOnly>> = LazyLock::new(Secp256k1::verification_only);

pub fn main() {}
